use {
  crate::{
    cli::Cli,
    config::Config,
    dependency_type::{DependencyType, Strategy},
    instance::InstanceDescriptor,
    package_json::PackageJson,
    rcfile::Rcfile,
    specifier::Specifier,
  },
  glob::glob,
  log::debug,
  serde::Deserialize,
  serde_json::Value,
  std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
  },
};

#[derive(Debug)]
pub struct Packages {
  pub all: Vec<Rc<RefCell<PackageJson>>>,
}

impl Packages {
  /// Create an empty collection of package.json files
  pub fn new() -> Self {
    Self { all: vec![] }
  }

  /// Get every package.json file matched by the user's source patterns
  pub fn from_config(config: &Config) -> Self {
    let file_paths = get_file_paths(config);
    let mut packages = Self::new();
    file_paths.iter().for_each(|file_path| {
      if let Some(package_json) = PackageJson::from_file(file_path) {
        packages.add_package(package_json);
      }
    });
    packages
  }

  /// Add a package.json file to this collection
  pub fn add_package(&mut self, package_json: PackageJson) -> &mut Self {
    self.all.push(Rc::new(RefCell::new(package_json)));
    self
  }

  /// Get a package.json file by its name
  pub fn get_by_name(&self, name: &str) -> Option<Rc<RefCell<PackageJson>>> {
    self.all.iter().find(|package| package.borrow().name == name).map(Rc::clone)
  }

  /// Return an index of every local package with a valid name and version
  pub fn get_local_versions(&self) -> HashMap<String, Rc<Specifier>> {
    self
      .all
      .iter()
      .filter_map(|package| -> Option<(String, Rc<Specifier>)> {
        let package = package.borrow();
        let name = package.get_prop("/name");
        let version = package.get_prop("/version");
        if let (Some(Value::String(name)), Some(Value::String(version))) = (name, version) {
          Some((name, Specifier::new(&version)))
        } else {
          None
        }
      })
      .collect()
  }

  /// Get every instance of a dependency from every package.json file
  pub fn get_all_instances<F>(&self, all_dependency_types: &Vec<DependencyType>, mut on_instance: F)
  where
    F: FnMut(InstanceDescriptor),
  {
    let _local_versions = self.get_local_versions();
    for package in self.all.iter() {
      for dependency_type in all_dependency_types {
        match dependency_type.strategy {
          Strategy::NameAndVersionProps => {
            if let (Some(Value::String(name)), Some(Value::String(raw_specifier))) = (
              package.borrow().get_prop(dependency_type.name_path.as_ref().unwrap()),
              package.borrow().get_prop(&dependency_type.path).or_else(|| {
                // Ensure that instances are still created for local packages
                // which are missing a version
                if dependency_type.name == "local" {
                  Some(Value::String("".to_string()))
                } else {
                  None
                }
              }),
            ) {
              on_instance(InstanceDescriptor {
                dependency_type: dependency_type.clone(),
                internal_name: name.to_string(),
                matches_cli_filter: false,
                name: name.to_string(),
                package: Rc::clone(package),
                specifier: Specifier::new(&raw_specifier),
              });
            }
          }
          Strategy::NamedVersionString => {
            if let Some(Value::String(specifier)) = package.borrow().get_prop(&dependency_type.path) {
              if let Some((name, raw_specifier)) = specifier.split_once('@') {
                on_instance(InstanceDescriptor {
                  dependency_type: dependency_type.clone(),
                  internal_name: name.to_string(),
                  matches_cli_filter: false,
                  name: name.to_string(),
                  package: Rc::clone(package),
                  specifier: Specifier::new(raw_specifier),
                });
              }
            }
          }
          Strategy::UnnamedVersionString => {
            if let Some(Value::String(raw_specifier)) = package.borrow().get_prop(&dependency_type.path) {
              on_instance(InstanceDescriptor {
                dependency_type: dependency_type.clone(),
                internal_name: dependency_type.name.clone(),
                matches_cli_filter: false,
                name: dependency_type.name.clone(),
                package: Rc::clone(package),
                specifier: Specifier::new(&raw_specifier),
              });
            }
          }
          Strategy::VersionsByName => {
            if let Some(Value::Object(versions_by_name)) = package.borrow().get_prop(&dependency_type.path) {
              for (name, raw_specifier) in versions_by_name {
                if let Value::String(raw_specifier) = raw_specifier {
                  on_instance(InstanceDescriptor {
                    dependency_type: dependency_type.clone(),
                    internal_name: name.to_string(),
                    matches_cli_filter: false,
                    name: name.to_string(),
                    package: Rc::clone(package),
                    specifier: Specifier::new(&raw_specifier),
                  });
                }
              }
            }
          }
          Strategy::InvalidConfig => {
            panic!("unrecognised strategy");
          }
        };
      }
    }
  }
}

/// Normalize a source pattern by:
/// 1. Converting Windows backslashes to forward slashes for glob compatibility
/// 2. Ensuring pattern ends with /package.json
///
/// Examples:
/// - "projects\\apps\\*" -> "projects/apps/*/package.json"
/// - "projects/libs/*" -> "projects/libs/*/package.json"
/// - "package.json" -> "package.json"
/// - "apps\\*/package.json" -> "apps/*/package.json"
pub fn normalize_pattern(pattern: String) -> String {
  let normalized = pattern.replace('\\', "/");
  if normalized.contains("package.json") {
    normalized
  } else {
    format!("{normalized}/package.json")
  }
}

/// Resolve every source glob pattern into their absolute file paths of
/// package.json files
fn get_file_paths(config: &Config) -> Vec<PathBuf> {
  get_source_patterns(config)
    .iter()
    .map(|pattern| {
      if PathBuf::from(pattern).is_absolute() {
        pattern.clone()
      } else {
        config.cli.cwd.join(pattern).to_str().unwrap().to_string()
      }
    })
    .flat_map(|pattern| glob(&pattern).ok())
    .flat_map(|paths| {
      paths
        .filter_map(Result::ok)
        .filter(|path| !path.to_string_lossy().contains("node_modules"))
        .fold(vec![], |mut paths, path| {
          paths.push(path.clone());
          paths
        })
    })
    .collect()
}

/// Based on the user's config file and command line `--source` options, return
/// the source glob patterns which should be used to resolve package.json files
fn get_source_patterns(config: &Config) -> Vec<String> {
  get_cli_patterns(&config.cli)
    .or_else(|| {
      debug!("No --source patterns provided");
      None
    })
    .or_else(|| get_rcfile_patterns(&config.rcfile))
    .or_else(|| {
      debug!("No .source patterns in Rcfile");
      None
    })
    .or_else(|| {
      get_npm_and_yarn_patterns(&config.cli.cwd)
        .or_else(|| {
          debug!("No .workspaces.packages or workspaces patterns in package.json");
          None
        })
        .or_else(|| get_pnpm_patterns(&config.cli.cwd))
        .or_else(|| {
          debug!("No .packages patterns in pnpm-workspace.yaml");
          None
        })
        .or_else(|| get_lerna_patterns(&config.cli.cwd))
        .or_else(|| {
          debug!("No .packages patterns in lerna.json");
          None
        })
        .as_ref()
        .map(|patterns| {
          let mut patterns = patterns.clone();
          patterns.push("package.json".to_string());
          patterns
        })
    })
    .map(|patterns| patterns.into_iter().map(normalize_pattern).collect())
    .or_else(get_default_patterns)
    .unwrap()
}

/// Get source patterns provided via the `--source` CLI option
fn get_cli_patterns(cli: &Cli) -> Option<Vec<String>> {
  if cli.source_patterns.is_empty() {
    None
  } else {
    Some(cli.source_patterns.clone())
  }
}

/// Get source patterns from the syncpack config file
fn get_rcfile_patterns(rcfile: &Rcfile) -> Option<Vec<String>> {
  if rcfile.source.is_empty() {
    None
  } else {
    Some(rcfile.source.clone())
  }
}

/// Look for source patterns in the `pnpm-workspace.yaml` file
fn get_pnpm_patterns(cwd: &Path) -> Option<Vec<String>> {
  let file_path = cwd.join("pnpm-workspace.yaml");
  let json = fs::read_to_string(&file_path).ok()?;
  let pnpm_workspace: SourcesUnderPackages = serde_yaml::from_str(&json).ok()?;
  pnpm_workspace.packages
}

#[derive(Debug, Deserialize)]
struct SourcesUnderPackages {
  packages: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct SourcesUnderWorkspacesDotPackages {
  workspaces: SourcesUnderPackages,
}

#[derive(Debug, Deserialize)]
struct SourcesUnderWorkspaces {
  workspaces: Option<Vec<String>>,
}

/// Look for source patterns in the `package.json` file in the locations
/// searched by `npm` and `yarn`
fn get_npm_and_yarn_patterns(cwd: &Path) -> Option<Vec<String>> {
  let file_path = cwd.join("package.json");
  let json = fs::read_to_string(&file_path).ok()?;
  serde_json::from_str::<SourcesUnderWorkspacesDotPackages>(&json)
    .ok()
    .and_then(|package_json| package_json.workspaces.packages)
    .or_else(|| {
      serde_json::from_str::<SourcesUnderWorkspaces>(&json)
        .ok()
        .and_then(|package_json| package_json.workspaces)
    })
}

/// Look for source patterns in the `lerna.json` file
fn get_lerna_patterns(cwd: &Path) -> Option<Vec<String>> {
  let file_path = cwd.join("lerna.json");
  let json = fs::read_to_string(&file_path).ok()?;
  let lerna_json: SourcesUnderPackages = serde_json::from_str(&json).ok()?;
  lerna_json.packages
}

/// Default source patterns to use if no other source patterns are found
fn get_default_patterns() -> Option<Vec<String>> {
  debug!("Using default source patterns");
  Some(vec![String::from("package.json"), String::from("packages/*/package.json")])
}
