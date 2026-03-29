pub mod semver_group;

#[cfg(test)]
#[path = "rcfile_test.rs"]
mod rcfile_test;

use {
  crate::{
    context::ConfigError,
    dependency::DependencyType,
    group_selector::GroupSelector,
    packages::Packages,
    version_group::{AnyVersionGroup, VersionGroup},
  },
  semver_group::{AnySemverGroup, SemverGroup},
  serde::Deserialize,
  serde_json::Value,
  std::{collections::HashMap, mem},
};

pub fn compute_all_dependency_types(custom_types: &HashMap<String, CustomType>) -> Vec<DependencyType> {
  let default_types = HashMap::from([
    (
      String::from("dev"),
      CustomType {
        strategy: String::from("versionsByName"),
        name_path: None,
        path: String::from("devDependencies"),
        unknown_fields: HashMap::new(),
      },
    ),
    (
      String::from("local"),
      CustomType {
        strategy: String::from("name~version"),
        name_path: Some(String::from("name")),
        path: String::from("version"),
        unknown_fields: HashMap::new(),
      },
    ),
    (
      String::from("overrides"),
      CustomType {
        strategy: String::from("versionsByName"),
        name_path: None,
        path: String::from("overrides"),
        unknown_fields: HashMap::new(),
      },
    ),
    (
      String::from("peer"),
      CustomType {
        strategy: String::from("versionsByName"),
        name_path: None,
        path: String::from("peerDependencies"),
        unknown_fields: HashMap::new(),
      },
    ),
    (
      String::from("pnpmOverrides"),
      CustomType {
        strategy: String::from("versionsByName"),
        name_path: None,
        path: String::from("pnpm.overrides"),
        unknown_fields: HashMap::new(),
      },
    ),
    (
      String::from("prod"),
      CustomType {
        strategy: String::from("versionsByName"),
        name_path: None,
        path: String::from("dependencies"),
        unknown_fields: HashMap::new(),
      },
    ),
    (
      String::from("resolutions"),
      CustomType {
        strategy: String::from("versionsByName"),
        name_path: None,
        path: String::from("resolutions"),
        unknown_fields: HashMap::new(),
      },
    ),
  ]);
  default_types
    .iter()
    .chain(custom_types.iter())
    .map(|(name, custom_type)| DependencyType::new(name, custom_type))
    .collect()
}

mod discovery;
pub mod error;
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

fn default_indent() -> Option<String> {
  None
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
  #[serde(flatten)]
  pub unknown_fields: HashMap<String, Value>,
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
  #[serde(flatten)]
  pub unknown_fields: HashMap<String, Value>,
}

/// Raw deserialized config file. Converted to `Rcfile` via `From<RawRcfile>`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawRcfile {
  #[serde(rename = "$schema", skip_serializing)]
  _schema: Option<serde::de::IgnoredAny>,
  #[serde(default = "empty_custom_types")]
  pub custom_types: HashMap<String, CustomType>,
  #[serde(default)]
  pub dependency_groups: Vec<DependencyGroup>,
  #[serde(default = "default_false")]
  pub format_bugs: bool,
  #[serde(default = "default_false")]
  pub format_repository: bool,
  #[serde(default = "default_indent")]
  pub indent: Option<String>,
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
  #[serde(flatten)]
  pub unknown_fields: HashMap<String, Value>,
}

impl RawRcfile {
  /// Handle config that is no longer supported or was hallucinated by an LLM
  pub fn validate_unknown_fields(&self) -> Result<(), Vec<ConfigError>> {
    let mut errors: Vec<ConfigError> = vec![];
    self.unknown_fields.iter().for_each(|(key, _)| match key.as_str() {
      "dependencyTypes" => {
        errors.push(ConfigError::DeprecatedProperty {
          property: key.clone(),
          hint: "Use CLI flag instead: --dependency-types prod,dev,peer".to_string(),
        });
      }
      "specifierTypes" => {
        errors.push(ConfigError::DeprecatedProperty {
          property: key.clone(),
          hint: "Use CLI flag instead: --specifier-types exact,range".to_string(),
        });
      }
      "lintFormatting" => {
        errors.push(ConfigError::DeprecatedProperty {
          property: key.clone(),
          hint: "Use 'syncpack format --check' to validate formatting".to_string(),
        });
      }
      "lintSemverRanges" => {
        errors.push(ConfigError::DeprecatedProperty {
          property: key.clone(),
          hint: "Semver range checking is always enabled in 'syncpack lint'".to_string(),
        });
      }
      "lintVersions" => {
        errors.push(ConfigError::DeprecatedProperty {
          property: key.clone(),
          hint: "Version checking is always enabled in 'syncpack lint'".to_string(),
        });
      }
      _ => {
        if !key.starts_with("//") {
          errors.push(ConfigError::UnrecognisedProperty { path: key.clone() });
        }
      }
    });
    self.custom_types.iter().for_each(|(custom_type_name, value)| {
      value.unknown_fields.iter().for_each(|(key, _)| {
        if !key.starts_with("//") {
          errors.push(ConfigError::UnrecognisedProperty {
            path: format!("customTypes.{custom_type_name}.{key}"),
          });
        }
      });
    });
    self.dependency_groups.iter().enumerate().for_each(|(index, value)| {
      value.unknown_fields.iter().for_each(|(key, _)| {
        if !key.starts_with("//") {
          errors.push(ConfigError::UnrecognisedProperty {
            path: format!("dependencyGroups[{index}].{key}"),
          });
        }
      });
    });
    self.semver_groups.iter().enumerate().for_each(|(index, value)| {
      value.unknown_fields.iter().for_each(|(key, _)| {
        if !key.starts_with("//") {
          errors.push(ConfigError::UnrecognisedProperty {
            path: format!("semverGroups[{index}].{key}"),
          });
        }
      });
    });
    self.version_groups.iter().enumerate().for_each(|(index, value)| {
      value.unknown_fields.iter().for_each(|(key, _)| {
        if !key.starts_with("//") {
          errors.push(ConfigError::UnrecognisedProperty {
            path: format!("versionGroups[{index}].{key}"),
          });
        }
      });
    });
    if errors.is_empty() {
      Ok(())
    } else {
      Err(errors)
    }
  }
}

fn validate_raw_dep_types(raw: &[String], all: &[DependencyType]) -> Result<(), ConfigError> {
  for s in raw {
    let name = s.trim_start_matches('!');
    if name != "**" && !all.iter().any(|dt| dt.name == name) {
      return Err(ConfigError::InvalidDependencyType { name: name.to_string() });
    }
  }
  Ok(())
}

impl TryFrom<RawRcfile> for Rcfile {
  type Error = ConfigError;

  fn try_from(raw: RawRcfile) -> Result<Self, ConfigError> {
    let all_dependency_types = compute_all_dependency_types(&raw.custom_types);
    let mut dependency_groups = vec![];
    for dg in raw.dependency_groups {
      let selector = GroupSelector::new(dg.dependencies, dg.dependency_types, dg.alias_name, dg.packages, dg.specifier_types);
      selector.validate_dependency_types(&all_dependency_types)?;
      dependency_groups.push(selector);
    }
    let mut semver_groups = vec![SemverGroup::get_exact_local_specifiers()];
    for group_config in raw.semver_groups {
      let semver_group = SemverGroup::from_config(group_config)?;
      semver_group.selector.validate_dependency_types(&all_dependency_types)?;
      semver_groups.push(semver_group);
    }
    semver_groups.push(SemverGroup::get_catch_all());
    for group in &raw.version_groups {
      validate_raw_dep_types(&group.dependency_types, &all_dependency_types)?;
    }

    Ok(Rcfile {
      dependency_groups,
      format_bugs: raw.format_bugs,
      format_repository: raw.format_repository,
      indent: raw.indent,
      max_concurrent_requests: raw.max_concurrent_requests,
      semver_groups,
      sort_az: raw.sort_az,
      sort_exports: raw.sort_exports,
      sort_first: raw.sort_first,
      sort_packages: raw.sort_packages,
      source: raw.source,
      strict: raw.strict,
      version_groups: raw.version_groups,
      all_dependency_types,
    })
  }
}

#[derive(Debug)]
pub struct Rcfile {
  pub dependency_groups: Vec<GroupSelector>,
  pub format_bugs: bool,
  pub format_repository: bool,
  pub indent: Option<String>,
  pub max_concurrent_requests: usize,
  pub semver_groups: Vec<SemverGroup>,
  pub sort_az: Vec<String>,
  pub sort_exports: Vec<String>,
  pub sort_first: Vec<String>,
  pub sort_packages: bool,
  pub source: Vec<String>,
  pub strict: bool,
  pub version_groups: Vec<AnyVersionGroup>,
  /// All dependency types (built-in + custom). Computed after deserialization.
  pub all_dependency_types: Vec<DependencyType>,
}

impl Default for Rcfile {
  fn default() -> Self {
    serde_json::from_str::<RawRcfile>("{}")
      .expect("An empty object should produce a default Rcfile")
      .try_into()
      .expect("Default Rcfile should always be valid")
  }
}

impl Rcfile {
  /// Create every version group defined in the rcfile.
  pub fn get_version_groups(&mut self, packages: &Packages) -> Result<Vec<VersionGroup>, ConfigError> {
    let mut all_groups: Vec<VersionGroup> = mem::take(&mut self.version_groups)
      .into_iter()
      .map(|group_config| VersionGroup::from_config(group_config, packages))
      .collect::<Result<Vec<_>, _>>()?;
    all_groups.push(VersionGroup::get_catch_all());
    Ok(all_groups)
  }
}
