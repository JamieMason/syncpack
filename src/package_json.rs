#[cfg(test)]
#[path = "package_json_test.rs"]
mod package_json_test;

use {
  crate::{dependency::Strategy, disk::DetectedFormatting, instance::Instance},
  log::error,
  serde::Serialize,
  serde_json::{ser::PrettyFormatter, Serializer, Value},
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
  /// The original file content as read from disk, used for change detection
  pub raw: String,
  /// The parsed JSON object
  pub contents: Value,
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
        raw,
        contents,
      })
      .ok()
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

  /// Deeply set a property in the parsed package.json
  pub fn set_prop(&mut self, pointer: &str, next_value: Value) {
    if pointer == "/" {
      self.contents = next_value;
    } else if let Some(value) = self.contents.pointer_mut(pointer) {
      *value = next_value;
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
        let versions_by_name = self.contents.pointer_mut(path_to_prop_str).unwrap().as_object_mut().unwrap();
        let old_specifier = versions_by_name.get_mut(&instance.descriptor.name).unwrap();
        *old_specifier = Value::String(raw_specifier);
      }
      Strategy::InvalidConfig => {
        unreachable!("unrecognised strategy");
      }
    };
  }

  /// Serialize the parsed JSON object back into pretty JSON as bytes.
  pub fn serialise(&self, formatting: &DetectedFormatting) -> Vec<u8> {
    serialise_json(&self.contents, formatting)
  }

  /// Write the package.json to disk, returns whether the file has changed
  pub fn write_to_disk(&mut self, indent_override: Option<&str>, formatting: &DetectedFormatting) -> bool {
    let vec = match indent_override {
      Some(indent) => self.serialise(&DetectedFormatting {
        indent: indent.to_string(),
        newline: formatting.newline.clone(),
      }),
      None => self.serialise(formatting),
    };
    std::fs::write(&self.file_path, &vec).expect("Failed to write package.json to disk");
    let next = String::from_utf8(vec).expect("Failed to convert JSON buffer to string");
    let has_changed = next != self.raw;
    if has_changed {
      self.raw = next;
    }
    has_changed
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

/// Serialize the parsed JSON object back into pretty JSON as bytes.
fn serialise_json(value: &serde_json::Value, formatting: &DetectedFormatting) -> Vec<u8> {
  let buffer = Vec::new();
  let indent = &formatting.indent.replace("\\t", "\t");
  let formatter = PrettyFormatter::with_indent(indent.as_bytes());
  let mut serializer = Serializer::with_formatter(buffer, formatter);
  value.serialize(&mut serializer).expect("Failed to serialize package.json");
  let mut writer = serializer.into_inner();
  writer.extend(formatting.newline.as_bytes());
  writer
}
