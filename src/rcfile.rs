use {
  crate::{
    dependency_type::DependencyType,
    group_selector::GroupSelector,
    packages::Packages,
    semver_group::{AnySemverGroup, SemverGroup},
    version_group::{AnyVersionGroup, VersionGroup},
  },
  serde::Deserialize,
  std::collections::HashMap,
};

mod discovery;
mod error;
mod javascript;
mod json;
mod package_json;
mod yaml;

fn empty_custom_types() -> HashMap<String, CustomType> {
  HashMap::new()
}

fn default_max_concurrent_requests() -> usize {
  12
}

fn default_true() -> bool {
  true
}

fn default_false() -> bool {
  false
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
    "svelte".to_string(),
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
pub struct DependencyGroup {
  pub alias_name: String,
  #[serde(default)]
  pub dependencies: Vec<String>,
  #[serde(default)]
  pub dependency_types: Vec<String>,
  #[serde(default)]
  pub packages: Vec<String>,
  #[serde(default)]
  pub specifier_types: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rcfile {
  #[serde(default = "empty_custom_types")]
  pub custom_types: HashMap<String, CustomType>,
  #[serde(default)]
  pub dependency_groups: Vec<DependencyGroup>,
  #[serde(default = "default_false")]
  pub format_bugs: bool,
  #[serde(default = "default_false")]
  pub format_repository: bool,
  #[serde(default = "default_indent")]
  pub indent: String,
  #[serde(default = "default_max_concurrent_requests")]
  pub max_concurrent_requests: usize,
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
  #[serde(default = "default_false")]
  pub strict: bool,
  #[serde(default)]
  pub version_groups: Vec<AnyVersionGroup>,
}

impl Default for Rcfile {
  fn default() -> Self {
    serde_json::from_str("{}").expect("An empty object should produce a default Rcfile")
  }
}

impl Rcfile {
  /// Create every alias defined in the rcfile.
  pub fn get_dependency_groups(&self, packages: &Packages, all_dependency_types: &[DependencyType]) -> Vec<GroupSelector> {
    self
      .dependency_groups
      .iter()
      .map(|dependency_group_config| {
        GroupSelector::new(
          /* all_packages: */ packages,
          /* include_dependencies: */ dependency_group_config.dependencies.clone(),
          /* include_dependency_types: */ dependency_group_config.dependency_types.clone(),
          /* alias_name: */ dependency_group_config.alias_name.clone(),
          /* include_packages: */ dependency_group_config.packages.clone(),
          /* include_specifier_types: */ dependency_group_config.specifier_types.clone(),
          /* all_dependency_types: */ all_dependency_types,
        )
      })
      .collect()
  }

  /// Create every semver group defined in the rcfile.
  pub fn get_semver_groups(&self, packages: &Packages, all_dependency_types: &[DependencyType]) -> Vec<SemverGroup> {
    let mut all_groups: Vec<SemverGroup> = vec![];
    all_groups.push(SemverGroup::get_exact_local_specifiers(all_dependency_types));
    self.semver_groups.iter().for_each(|group_config| {
      all_groups.push(SemverGroup::from_config(group_config, packages, all_dependency_types));
    });
    all_groups.push(SemverGroup::get_catch_all(all_dependency_types));
    all_groups
  }

  /// Create every version group defined in the rcfile.
  pub fn get_version_groups(&self, packages: &Packages, all_dependency_types: &[DependencyType]) -> Vec<VersionGroup> {
    let mut all_groups: Vec<VersionGroup> = self
      .version_groups
      .iter()
      .map(|group_config| VersionGroup::from_config(group_config, packages, all_dependency_types))
      .collect();
    all_groups.push(VersionGroup::get_catch_all(all_dependency_types));
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
