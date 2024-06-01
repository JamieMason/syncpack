use log::debug;
use serde_json::Value;
use std::path::PathBuf;

use crate::{
  dependency_type::{DependencyType, Strategy},
  package_json::PackageJson,
  specifier::Specifier,
};

pub type InstanceId = String;

#[derive(Debug)]
pub struct Instance {
  /// A unique identifier for this instance
  pub id: InstanceId,
  /// The dependency type to use to read/write this instance
  pub dependency_type: DependencyType,
  /// The file path of the package.json file this instance belongs to
  pub file_path: PathBuf,
  /// Whether this is a package developed in this repo
  pub is_local: bool,
  /// The dependency name eg. "react", "react-dom"
  pub name: String,
  /// The `.name` of the package.json this file is in
  pub package_name: String,
  /// The parsed dependency specifier
  pub specifier_type: Specifier,
  /// The raw dependency specifier eg. "16.8.0", "^16.8.0"
  pub specifier: String,
}

impl Instance {
  pub fn new(
    name: String,
    specifier: String,
    dependency_type: DependencyType,
    package: &PackageJson,
  ) -> Instance {
    let package_name = package.get_name();
    Instance {
      id: format!("{}|{}|{}", name, dependency_type.name, package_name),
      dependency_type,
      file_path: package.file_path.clone(),
      is_local: package_name == name,
      name,
      package_name,
      specifier_type: Specifier::new(specifier.as_str()),
      specifier: sanitise_specifier(specifier),
    }
  }

  /// Write a version to the package.json
  pub fn set_version(&mut self, package: &mut PackageJson, next_value: String) {
    match self.dependency_type.strategy {
      Strategy::NameAndVersionProps => {
        let path_to_prop = &self.dependency_type.path;
        let path_to_prop_str = path_to_prop.as_str();
        package.set_prop(path_to_prop_str, Value::String(next_value.clone()));
      }
      Strategy::NamedVersionString => {
        let path_to_prop = &self.dependency_type.path;
        let path_to_prop_str = path_to_prop.as_str();
        let full_value = format!("{}@{}", self.name, next_value);
        package.set_prop(path_to_prop_str, Value::String(full_value));
      }
      Strategy::UnnamedVersionString => {
        let path_to_prop = &self.dependency_type.path;
        let path_to_prop_str = path_to_prop.as_str();
        package.set_prop(path_to_prop_str, Value::String(next_value.clone()));
      }
      Strategy::VersionsByName => {
        let path_to_obj = &self.dependency_type.path;
        let name = &self.name;
        let path_to_obj_str = path_to_obj.as_str();
        let obj = package
          .contents
          .pointer_mut(path_to_obj_str)
          .unwrap()
          .as_object_mut()
          .unwrap();
        let value = obj.get_mut(name).unwrap();
        *value = Value::String(next_value.clone());
      }
      Strategy::InvalidConfig => {
        panic!("unrecognised strategy");
      }
    };
    // update in-memory state
    self.specifier = next_value.clone();
    self.specifier_type = Specifier::new(next_value.as_str());
  }

  /// Delete a version/dependency/instance from the package.json
  pub fn remove_from(&self, package: &mut PackageJson) {
    match self.dependency_type.strategy {
      Strategy::NameAndVersionProps => {
        //
      }
      Strategy::NamedVersionString => {
        //
      }
      Strategy::UnnamedVersionString => {
        //
      }
      Strategy::VersionsByName => {
        let path_to_obj = &self.dependency_type.path;
        let name = &self.name;
        let path_to_obj_str = path_to_obj.as_str();
        if let Some(value) = package.contents.pointer_mut(path_to_obj) {
          if let Value::Object(obj) = value {
            obj.remove(name);
          }
        }
      }
      Strategy::InvalidConfig => {
        panic!("unrecognised strategy");
      }
    };
  }
}

/// Convert non-semver specifiers to semver when behaviour is identical
fn sanitise_specifier(specifier: String) -> String {
  if specifier == "latest" || specifier == "x" {
    debug!("Sanitising specifier: {} -> *", specifier);
    "*".to_string()
  } else {
    specifier
  }
}