use {
  crate::{
    dependency_type::DependencyType,
    packages::Packages,
    semver_group::{AnySemverGroup, SemverGroup},
    version_group::{AnyVersionGroup, VersionGroup},
  },
  atty::Stream,
  log::debug,
  serde::Deserialize,
  std::{collections::HashMap, io},
};

fn empty_custom_types() -> HashMap<String, CustomType> {
  HashMap::new()
}

fn default_true() -> bool {
  true
}

fn default_indent() -> String {
  "  ".to_string()
}

fn default_sort_az() -> Vec<String> {
  vec![
    "bin".to_string(),
    "contributors".to_string(),
    "dependencies".to_string(),
    "devDependencies".to_string(),
    "keywords".to_string(),
    "peerDependencies".to_string(),
    "resolutions".to_string(),
    "scripts".to_string(),
  ]
}

fn default_sort_exports() -> Vec<String> {
  vec![
    "types".to_string(),
    "node-addons".to_string(),
    "node".to_string(),
    "browser".to_string(),
    "module".to_string(),
    "import".to_string(),
    "require".to_string(),
    "development".to_string(),
    "production".to_string(),
    "script".to_string(),
    "default".to_string(),
  ]
}

fn sort_first() -> Vec<String> {
  vec![
    "name".to_string(),
    "description".to_string(),
    "version".to_string(),
    "author".to_string(),
  ]
}

fn default_source() -> Vec<String> {
  vec![]
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomType {
  pub strategy: String,
  pub name_path: Option<String>,
  pub path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rcfile {
  #[serde(default = "empty_custom_types")]
  pub custom_types: HashMap<String, CustomType>,
  #[serde(default = "default_true")]
  pub format_bugs: bool,
  #[serde(default = "default_true")]
  pub format_repository: bool,
  #[serde(default = "default_indent")]
  pub indent: String,
  #[serde(default)]
  pub semver_groups: Vec<AnySemverGroup>,
  #[serde(default = "default_sort_az")]
  pub sort_az: Vec<String>,
  #[serde(default = "default_sort_exports")]
  pub sort_exports: Vec<String>,
  #[serde(default = "sort_first")]
  pub sort_first: Vec<String>,
  #[serde(default = "default_true")]
  pub sort_packages: bool,
  #[serde(default = "default_source")]
  pub source: Vec<String>,
  #[serde(default)]
  pub version_groups: Vec<AnyVersionGroup>,
}

impl Rcfile {
  /// Create a new rcfile containing only default values.
  pub fn new() -> Rcfile {
    Rcfile {
      custom_types: empty_custom_types(),
      format_bugs: default_true(),
      format_repository: default_true(),
      indent: default_indent(),
      semver_groups: vec![],
      sort_az: default_sort_az(),
      sort_exports: default_sort_exports(),
      sort_first: sort_first(),
      sort_packages: default_true(),
      source: default_source(),
      version_groups: vec![],
    }
  }

  /// Create a new rcfile from a single line of JSON piped into stdin
  pub fn from_stdin() -> Rcfile {
    if atty::is(Stream::Stdin) {
      debug!("No Rcfile piped into stdin, reverting to defaults");
      return Rcfile::new();
    }
    let mut buffer = String::new();
    let json = io::stdin().read_line(&mut buffer).map_or_else(|_| "{}".to_string(), |_| buffer);
    if json == "{}" {
      debug!("Empty Rcfile piped into stdin, reverting to defaults");
      return Rcfile::new();
    }
    debug!("A non-empty Rcfile was piped into stdin");
    Rcfile::from_json(json)
  }

  /// Create a new rcfile from a JSON string or revert to defaults
  pub fn from_json(json: String) -> Rcfile {
    serde_json::from_str(&json).unwrap_or_else(|_| Rcfile::new())
  }

  /// Create every semver group defined in the rcfile.
  pub fn get_semver_groups(&self) -> Vec<SemverGroup> {
    let mut all_groups: Vec<SemverGroup> = vec![];
    all_groups.push(SemverGroup::get_exact_local_specifiers());
    self.semver_groups.iter().for_each(|group_config| {
      all_groups.push(SemverGroup::from_config(group_config));
    });
    all_groups.push(SemverGroup::get_catch_all());
    all_groups
  }

  /// Create every version group defined in the rcfile.
  pub fn get_version_groups(&self, packages: &Packages) -> Vec<VersionGroup> {
    let mut all_groups: Vec<VersionGroup> = self
      .version_groups
      .iter()
      .map(|group_config| VersionGroup::from_config(group_config, packages))
      .collect();
    all_groups.push(VersionGroup::get_catch_all());
    all_groups
  }

  /// Get all custom types defined in the rcfile, combined with the default
  /// types.
  pub fn get_all_dependency_types(&self) -> Vec<DependencyType> {
    // Custom dependency types defined in the rcfile
    let custom_types = &self.custom_types;
    // Internal dependency types are also defined as custom types
    let default_types = HashMap::from([
      (
        String::from("dev"),
        CustomType {
          strategy: String::from("versionsByName"),
          name_path: None,
          path: String::from("devDependencies"),
        },
      ),
      (
        String::from("local"),
        CustomType {
          strategy: String::from("name~version"),
          name_path: Some(String::from("name")),
          path: String::from("version"),
        },
      ),
      (
        String::from("overrides"),
        CustomType {
          strategy: String::from("versionsByName"),
          name_path: None,
          path: String::from("overrides"),
        },
      ),
      (
        String::from("peer"),
        CustomType {
          strategy: String::from("versionsByName"),
          name_path: None,
          path: String::from("peerDependencies"),
        },
      ),
      (
        String::from("pnpmOverrides"),
        CustomType {
          strategy: String::from("versionsByName"),
          name_path: None,
          path: String::from("pnpm.overrides"),
        },
      ),
      (
        String::from("prod"),
        CustomType {
          strategy: String::from("versionsByName"),
          name_path: None,
          path: String::from("dependencies"),
        },
      ),
      (
        String::from("resolutions"),
        CustomType {
          strategy: String::from("versionsByName"),
          name_path: None,
          path: String::from("resolutions"),
        },
      ),
    ]);
    // Collect which dependency types are enabled
    let mut dependency_types: Vec<DependencyType> = vec![];
    default_types.iter().for_each(|(name, custom_type)| {
      dependency_types.push(DependencyType::new(name, custom_type));
    });
    custom_types.iter().for_each(|(name, custom_type)| {
      dependency_types.push(DependencyType::new(name, custom_type));
    });
    dependency_types
  }
}
