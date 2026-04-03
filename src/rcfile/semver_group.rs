use {
  crate::{errors::ConfigError, group_selector::GroupSelector, semver_range::SemverRange},
  serde::Deserialize,
  serde_json::Value,
  std::collections::HashMap,
};

#[derive(Debug)]
pub struct SemverGroup {
  /// Data to determine which instances should be added to this group
  pub selector: GroupSelector,
  /// The Semver Range which all instances in this group should use
  pub range: Option<SemverRange>,
}

impl SemverGroup {
  /// Create a default group which ensures local packages are an exact version
  pub fn get_exact_local_specifiers() -> SemverGroup {
    SemverGroup {
      selector: GroupSelector::new(
        /* include_dependencies: */ vec![],
        /* include_dependency_types: */ vec!["local".to_string()],
        /* label: */ "Local package versions must be exact".to_string(),
        /* include_packages: */ vec![],
        /* include_specifier_types: */ vec![],
      ),
      range: None,
    }
  }

  /// Create a default/catch-all group which would apply to any instance
  pub fn get_catch_all() -> SemverGroup {
    SemverGroup {
      selector: GroupSelector::new(
        /* include_dependencies: */ vec![],
        /* include_dependency_types: */ vec![],
        /* label: */ "Default Semver Group".to_string(),
        /* include_packages: */ vec![],
        /* include_specifier_types: */ vec![],
      ),
      range: None,
    }
  }

  /// Create a single version group from a config item from the rcfile.
  pub fn from_config(group: AnySemverGroup) -> Result<SemverGroup, ConfigError> {
    let selector = GroupSelector::new(
      /* include_dependencies: */ group.dependencies,
      /* include_dependency_types: */ group.dependency_types,
      /* label: */ group.label,
      /* include_packages: */ group.packages,
      /* include_specifier_types: */ group.specifier_types,
    );

    if let Some(true) = group.is_disabled {
      Ok(SemverGroup { selector, range: None })
    } else if let Some(true) = group.is_ignored {
      Ok(SemverGroup { selector, range: None })
    } else if let Some(range) = &group.range {
      Ok(SemverGroup {
        selector,
        range: SemverRange::new(range),
      })
    } else {
      Err(ConfigError::InvalidSemverGroup)
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnySemverGroup {
  #[serde(default)]
  pub dependencies: Vec<String>,
  #[serde(default)]
  pub dependency_types: Vec<String>,
  #[serde(default)]
  pub label: String,
  #[serde(default)]
  pub packages: Vec<String>,
  #[serde(default)]
  pub specifier_types: Vec<String>,
  //
  pub is_disabled: Option<bool>,
  pub is_ignored: Option<bool>,
  pub range: Option<String>,
  #[serde(flatten)]
  pub unknown_fields: HashMap<String, Value>,
}
