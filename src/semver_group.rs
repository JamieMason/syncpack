use {
  crate::{group_selector::GroupSelector, packages::Packages, specifier::semver_range::SemverRange},
  serde::Deserialize,
};

#[derive(Debug)]
pub enum SemverGroupVariant {
  Disabled,
  Ignored,
  WithRange,
}

#[derive(Debug)]
pub struct SemverGroup {
  /// What behaviour has this group been configured to exhibit?
  pub variant: SemverGroupVariant,
  /// Data to determine which instances should be added to this group
  pub selector: GroupSelector,
  /// The Semver Range which all instances in this group should use
  pub range: Option<SemverRange>,
}

impl SemverGroup {
  /// Create a default group which ensures local packages are an exact version
  pub fn get_exact_local_specifiers() -> SemverGroup {
    SemverGroup {
      variant: SemverGroupVariant::WithRange,
      selector: GroupSelector::new(
        /* all_packages: */ &Packages::new(),
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
      variant: SemverGroupVariant::Disabled,
      selector: GroupSelector::new(
        /* all_packages: */ &Packages::new(),
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
  pub fn from_config(group: &AnySemverGroup, packages: &Packages) -> SemverGroup {
    let selector = GroupSelector::new(
      /* all_packages: */ packages,
      /* include_dependencies: */ group.dependencies.clone(),
      /* include_dependency_types: */ group.dependency_types.clone(),
      /* label: */ group.label.clone(),
      /* include_packages: */ group.packages.clone(),
      /* include_specifier_types: */ group.specifier_types.clone(),
    );

    if let Some(true) = group.is_disabled {
      SemverGroup {
        variant: SemverGroupVariant::Disabled,
        selector,
        range: None,
      }
    } else if let Some(true) = group.is_ignored {
      SemverGroup {
        variant: SemverGroupVariant::Ignored,
        selector,
        range: None,
      }
    } else if let Some(range) = &group.range {
      SemverGroup {
        variant: SemverGroupVariant::WithRange,
        selector,
        range: SemverRange::new(range),
      }
    } else {
      panic!("Invalid semver group");
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
}
