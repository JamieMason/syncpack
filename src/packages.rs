use {
  crate::{
    dependency::{DependencyType, Strategy},
    disk::{DetectedFormatting, Disk, DiskIo},
    instance::InstanceDescriptor,
    package_json::PackageJson,
    specifier::Specifier,
  },
  log::error,
  serde_json::Value,
  std::{collections::HashSet, path::PathBuf},
};

/// Index into the Packages.all arena.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PackageIdx(pub usize);

#[derive(Debug)]
pub struct Packages {
  pub all: Vec<PackageJson>,
  pub formatting: DetectedFormatting,
}

impl Packages {
  /// Create an empty collection of package.json files
  pub fn new() -> Self {
    Self {
      all: vec![],
      formatting: DetectedFormatting::default(),
    }
  }

  /// Get every package.json file matched by the user's source patterns
  pub fn from_config<T: DiskIo>(disk: &Disk<T>, file_paths: &[PathBuf]) -> Self {
    let mut packages = Self::new();
    packages.formatting = disk
      .package_json_root
      .as_ref()
      .map(|pkg| pkg.formatting.clone())
      .unwrap_or_default();
    file_paths.iter().for_each(|file_path| match disk.io.read_textfile(file_path) {
      Some(Ok(file)) => {
        if let Some(package_json) = PackageJson::from_raw(file.contents.clone(), file_path.clone()) {
          packages.add_package(package_json);
        }
      }
      Some(Err(err)) => {
        error!("{err}");
      }
      None => {
        error!("package.json not readable at {}", file_path.to_str().unwrap_or("unknown"));
      }
    });
    packages
  }

  /// Add a package.json file to this collection
  pub fn add_package(&mut self, package_json: PackageJson) -> &mut Self {
    self.all.push(package_json);
    self
  }

  /// Get a package.json file's index by its name
  pub fn get_by_name(&self, name: &str) -> Option<PackageIdx> {
    self.all.iter().position(|package| package.name == name).map(PackageIdx)
  }

  /// Get every instance of a dependency from every package.json file
  pub fn get_all_instances<F>(&self, all_dependency_types: &Vec<DependencyType>, mut on_instance: F)
  where
    F: FnMut(InstanceDescriptor, &PackageJson),
  {
    // Pre-compute local package names for O(1) is_local_dependency lookups
    let local_package_names: HashSet<String> = self.all.iter().map(|p| p.name.clone()).collect();
    let empty_version = Value::String(String::new());

    for (pkg_index, package) in self.all.iter().enumerate() {
      let package_idx = PackageIdx(pkg_index);
      let contents = &package.contents;
      for dependency_type in all_dependency_types {
        match dependency_type.strategy {
          Strategy::NameAndVersionProps => {
            let name_path = dependency_type.name_path.as_ref().unwrap();
            let name_val = contents.pointer(name_path);
            let version_val = contents.pointer(&dependency_type.path).or_else(|| {
              if dependency_type.name == "local" {
                Some(&empty_version)
              } else {
                None
              }
            });
            if let (Some(Value::String(name)), Some(Value::String(raw_specifier))) = (name_val, version_val) {
              on_instance(
                InstanceDescriptor {
                  dependency_type: dependency_type.clone(),
                  internal_name: name.to_string(),
                  is_local_dependency: local_package_names.contains(name.as_str()),
                  matches_cli_filter: false,
                  name: name.to_string(),
                  package_idx,
                  specifier: Specifier::new(raw_specifier),
                },
                package,
              );
            }
          }
          Strategy::NamedVersionString => {
            if let Some(Value::String(specifier)) = contents.pointer(&dependency_type.path) {
              if let Some((name, raw_specifier)) = specifier.split_once('@') {
                on_instance(
                  InstanceDescriptor {
                    dependency_type: dependency_type.clone(),
                    internal_name: name.to_string(),
                    is_local_dependency: local_package_names.contains(name),
                    matches_cli_filter: false,
                    name: name.to_string(),
                    package_idx,
                    specifier: Specifier::new(raw_specifier),
                  },
                  package,
                );
              }
            }
          }
          Strategy::UnnamedVersionString => {
            if let Some(Value::String(raw_specifier)) = contents.pointer(&dependency_type.path) {
              on_instance(
                InstanceDescriptor {
                  dependency_type: dependency_type.clone(),
                  internal_name: dependency_type.name.clone(),
                  is_local_dependency: local_package_names.contains(&dependency_type.name),
                  matches_cli_filter: false,
                  name: dependency_type.name.clone(),
                  package_idx,
                  specifier: Specifier::new(raw_specifier),
                },
                package,
              );
            }
          }
          Strategy::VersionsByName => {
            if let Some(Value::Object(versions_by_name)) = contents.pointer(&dependency_type.path) {
              for (name, raw_specifier) in versions_by_name {
                if let Value::String(raw_specifier) = raw_specifier {
                  on_instance(
                    InstanceDescriptor {
                      dependency_type: dependency_type.clone(),
                      internal_name: name.to_string(),
                      is_local_dependency: local_package_names.contains(name.as_str()),
                      matches_cli_filter: false,
                      name: name.to_string(),
                      package_idx,
                      specifier: Specifier::new(raw_specifier),
                    },
                    package,
                  );
                }
              }
            }
          }
          Strategy::InvalidConfig => {
            unreachable!("unrecognised strategy");
          }
        };
      }
    }
  }
}
