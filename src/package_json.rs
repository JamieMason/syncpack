use {
  crate::{config::Config, dependency_type::Strategy, instance::Instance},
  log::error,
  serde::Serialize,
  serde_json::{ser::PrettyFormatter, Serializer, Value},
  std::{cell::RefCell, fs, path::PathBuf, rc::Rc},
};

#[derive(Debug)]
pub struct PackageJson {
  /// The name property of the package.json
  pub name: String,
  /// The path to the package.json file
  pub file_path: PathBuf,
  /// Syncpack formatting mismatches found in the file
  pub formatting_mismatches: RefCell<Vec<Rc<FormatMismatch>>>,
  /// The original, unedited raw JSON string
  pub json: RefCell<String>,
  /// The parsed JSON object
  pub contents: RefCell<Value>,
}

#[derive(Debug)]
pub struct FormatMismatch {
  /// The formatted value
  pub expected: Value,
  /// The name of the package.json file being linted
  pub package: Rc<RefCell<PackageJson>>,
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
  /// Read a package.json file from the given location
  pub fn from_file(file_path: &PathBuf) -> Option<Self> {
    fs::read_to_string(file_path)
      .inspect_err(|_| {
        error!("package.json not readable at {}", &file_path.to_str().unwrap());
      })
      .ok()
      .and_then(|json| {
        serde_json::from_str(&json)
          .inspect_err(|_| {
            error!("Invalid JSON: {}", &file_path.to_str().unwrap());
          })
          .map(|contents: Value| Self {
            name: contents
              .pointer("/name")
              .and_then(|name| name.as_str())
              .unwrap_or("NAME_IS_MISSING")
              .to_string(),
            file_path: file_path.clone(),
            formatting_mismatches: RefCell::new(vec![]),
            json: RefCell::new(contents.to_string()),
            contents: RefCell::new(contents),
          })
          .ok()
      })
  }

  /// Does a property exist at this path of the parsed package.json?
  pub fn has_prop(&self, pointer: &str) -> bool {
    self.contents.borrow().pointer(pointer).is_some()
  }

  pub fn has_formatting_mismatches(&self) -> bool {
    !self.formatting_mismatches.borrow().is_empty()
  }

  /// Convenience method to get a string property from the parsed package.json
  pub fn get_string(&self, pointer: &str) -> Option<String> {
    if let Some(Value::String(name)) = self.get_prop(pointer) {
      Some(name)
    } else {
      None
    }
  }

  /// Deeply get a property in the parsed package.json
  pub fn get_prop(&self, pointer: &str) -> Option<Value> {
    self.contents.borrow().pointer(pointer).cloned()
  }

  /// Deeply set a property in the parsed package.json
  pub fn set_prop(&self, pointer: &str, next_value: Value) {
    if pointer == "/" {
      *self.contents.borrow_mut() = next_value;
    } else if let Some(value) = self.contents.borrow_mut().pointer_mut(pointer) {
      *value = next_value;
    }
  }

  /// Update this package in-memory with the given instance's specifier
  pub fn copy_expected_specifier(&self, instance: &Instance) {
    let path_to_prop_str = &instance.descriptor.dependency_type.path.as_str();
    let raw_specifier = instance.expected_specifier.borrow().as_ref().unwrap().get_raw().clone();
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
        let mut contents = self.contents.borrow_mut();
        let versions_by_name = contents.pointer_mut(path_to_prop_str).unwrap().as_object_mut().unwrap();
        let old_specifier = versions_by_name.get_mut(&instance.descriptor.name).unwrap();
        *old_specifier = Value::String(raw_specifier);
        std::mem::drop(contents);
      }
      Strategy::InvalidConfig => {
        panic!("unrecognised strategy");
      }
    };
  }

  /// Serialize the parsed JSON object back into pretty JSON as bytes
  pub fn serialize(&self, indent: &str) -> Vec<u8> {
    // Create a pretty JSON formatter
    let indent_with_fixed_tabs = &indent.replace("\\t", "	");
    let formatter = PrettyFormatter::with_indent(indent_with_fixed_tabs.as_bytes());
    let buffer = Vec::new();
    let mut serializer = Serializer::with_formatter(buffer, formatter);
    // Write pretty JSON to the buffer
    self.contents.serialize(&mut serializer).expect("Failed to serialize package.json");
    // Append a new line to the buffer
    let mut writer = serializer.into_inner();
    writer.extend(b"\n");
    writer
  }

  /// Convert a buffer of pretty JSON as bytes to a pretty JSON string
  pub fn to_pretty_json(&self, vec: Vec<u8>) -> String {
    let from_utf8 = String::from_utf8(vec);
    from_utf8.expect("Failed to convert JSON buffer to string")
  }

  /// Write the package.json to disk, returns whether the file has changed
  pub fn write_to_disk(&self, config: &Config) -> bool {
    let vec = self.serialize(&config.rcfile.indent);
    std::fs::write(&self.file_path, &vec).expect("Failed to write package.json to disk");
    let next_json = self.to_pretty_json(vec);
    let has_changed = next_json != *self.json.borrow();
    if has_changed {
      *self.json.borrow_mut() = next_json;
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
