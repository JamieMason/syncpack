#[cfg(test)]
#[path = "package_json_test.rs"]
mod package_json_test;

use {
  crate::{
    dependency::Strategy,
    disk::{DetectedFormatting, DiskIo, DiskIoError, File},
    instance::Instance,
  },
  log::error,
  serde_json::Value,
  std::path::PathBuf,
};

#[derive(Debug)]
pub struct PackageJson {
  /// The name property of the package.json
  pub name: String,
  /// The path to the package.json file
  pub file_path: PathBuf,
  /// Syncpack formatting mismatches found in the file
  pub formatting_mismatches: Vec<FormatMismatch>,
  /// Whether the parsed JSON has been mutated since reading from disk
  dirty: bool,
  /// The parsed JSON object
  contents: Value,
}

#[derive(Debug)]
pub struct FormatMismatch {
  /// The formatted value
  pub expected: Value,
  /// The path to the property that was linted
  pub property_path: String,
  /// The broken linting rule
  pub variant: FormatMismatchVariant,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum FormatMismatchVariant {
  /// - ✓ `rcFile.formatBugs` is enabled
  /// - ✘ The `bugs` property is not formatted
  BugsPropertyIsNotFormatted,
  /// - ✓ `rcFile.formatRepository` is enabled
  /// - ✘ The `repository` property is not formatted
  RepositoryPropertyIsNotFormatted,
  /// - ✓ `rcFile.sortAz` is enabled
  /// - ✘ This property is not sorted alphabetically
  PropertyIsNotSortedAz,
  /// - ✓ `rcFile.sortPackages` is enabled
  /// - ✘ This package.json's properties are not sorted
  PackagePropertiesAreNotSorted,
  /// - ✓ `rcFile.sortExports` is enabled
  /// - ✘ The `exports` property is not sorted
  ExportsPropertyIsNotSorted,
}

impl PackageJson {
  /// Parse a package.json from a raw JSON string and a file path
  pub fn from_raw(raw: String, file_path: PathBuf) -> Option<Self> {
    serde_json::from_str(&raw)
      .inspect_err(|_| {
        error!("Invalid JSON: {}", file_path.to_str().unwrap_or("unknown"));
      })
      .map(|contents: Value| Self {
        name: contents
          .pointer("/name")
          .and_then(|name| name.as_str())
          .unwrap_or("NAME_IS_MISSING")
          .to_string(),
        file_path,
        formatting_mismatches: vec![],
        dirty: false,
        contents,
      })
      .ok()
  }

  /// Read-only access to the parsed JSON object
  pub fn contents(&self) -> &Value {
    &self.contents
  }

  /// Whether the parsed JSON has been mutated since reading from disk
  pub fn is_dirty(&self) -> bool {
    self.dirty
  }

  /// Does a property exist at this path of the parsed package.json?
  pub fn has_prop(&self, pointer: &str) -> bool {
    self.contents.pointer(pointer).is_some()
  }

  pub fn has_formatting_mismatches(&self) -> bool {
    !self.formatting_mismatches.is_empty()
  }

  /// Deeply get a property in the parsed package.json
  pub fn get_prop(&self, pointer: &str) -> Option<Value> {
    self.contents.pointer(pointer).cloned()
  }

  /// Deeply set a property in the parsed package.json, marks dirty only if
  /// the value actually changed
  pub fn set_prop(&mut self, pointer: &str, next_value: Value) {
    if pointer == "/" {
      if values_differ(&self.contents, &next_value) {
        self.contents = next_value;
        self.dirty = true;
      }
    } else if let Some(value) = self.contents.pointer_mut(pointer) {
      if values_differ(value, &next_value) {
        *value = next_value;
        self.dirty = true;
      }
    }
  }

  /// Set a key on a parent object, marks dirty if the value changed.
  /// Uses direct Map access to avoid JSON pointer escaping issues with
  /// keys containing `/` (eg. scoped npm packages like `@scope/name`).
  pub fn set_nested_prop(&mut self, parent_pointer: &str, key: &str, next_value: Value) {
    if let Some(Value::Object(obj)) = self.contents.pointer_mut(parent_pointer) {
      let is_changed = obj.get(key).is_none_or(|value| values_differ(value, &next_value));
      if is_changed {
        obj.insert(key.to_string(), next_value);
        self.dirty = true;
      }
    }
  }

  /// Remove a key from a parent object, marks dirty if the key existed
  pub fn remove_prop(&mut self, parent_pointer: &str, key: &str) {
    if let Some(Value::Object(obj)) = self.contents.pointer_mut(parent_pointer) {
      if obj.remove(key).is_some() {
        self.dirty = true;
      }
    }
  }

  /// Update this package in-memory with the given instance's specifier
  pub fn copy_expected_specifier(&mut self, instance: &Instance) {
    let path_to_prop_str = &instance.descriptor.dependency_type.path.as_str();
    let raw_specifier = instance.expected_specifier.borrow().as_ref().unwrap().get_raw().to_string();
    match instance.descriptor.dependency_type.strategy {
      Strategy::NameAndVersionProps => {
        self.set_prop(path_to_prop_str, Value::String(raw_specifier));
      }
      Strategy::NamedVersionString => {
        let full_value = format!("{}@{}", instance.descriptor.name, raw_specifier);
        self.set_prop(path_to_prop_str, Value::String(full_value));
      }
      Strategy::UnnamedVersionString => {
        self.set_prop(path_to_prop_str, Value::String(raw_specifier));
      }
      Strategy::VersionsByName => {
        self.set_nested_prop(path_to_prop_str, &instance.descriptor.name, Value::String(raw_specifier));
      }
      Strategy::InvalidConfig => {
        unreachable!("unrecognised strategy");
      }
    };
  }

  /// Write the package.json to disk, returns whether the file has changed
  pub fn write_to_disk<D: DiskIo>(
    &mut self,
    io: &D,
    indent_override: Option<&str>,
    formatting: &DetectedFormatting,
  ) -> Result<bool, DiskIoError> {
    if !self.dirty {
      return Ok(false);
    }
    let effective_formatting = match indent_override {
      Some(indent) => DetectedFormatting {
        indent: indent.to_string(),
        newline: formatting.newline.clone(),
      },
      None => formatting.clone(),
    };
    let file = File {
      filepath: self.file_path.clone(),
      formatting: effective_formatting,
      contents: &self.contents,
    };
    io.write_json_file(&file)?;
    self.dirty = false;
    Ok(true)
  }

  /// Return a short path for logging to the terminal
  pub fn get_relative_file_path(&self, cwd: &PathBuf) -> String {
    self
      .file_path
      .strip_prefix(cwd)
      .ok()
      .and_then(|path| path.to_str().map(|path_str| path_str.to_string()))
      .expect("Failed to create relative file path")
  }
}

/// Order-aware comparison of two JSON values. Unlike `Value::eq`, this treats
/// objects with different key order as different, matching serialisation behaviour.
fn values_differ(a: &Value, b: &Value) -> bool {
  match (a, b) {
    (Value::Object(a), Value::Object(b)) => {
      a.len() != b.len() || a.iter().zip(b.iter()).any(|((k1, v1), (k2, v2))| k1 != k2 || values_differ(v1, v2))
    }
    (Value::Array(a), Value::Array(b)) => a.len() != b.len() || a.iter().zip(b.iter()).any(|(a, b)| values_differ(a, b)),
    _ => a != b,
  }
}
