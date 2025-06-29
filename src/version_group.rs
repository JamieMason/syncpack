use {
  crate::{
    cli::SortBy,
    dependency::{Dependency, UpdateUrl},
    dependency_type::DependencyType,
    group_selector::GroupSelector,
    instance::Instance,
    package_json::PackageJson,
    packages::Packages,
    specifier::Specifier,
  },
  itertools::Itertools,
  log::warn,
  serde::Deserialize,
  std::{cell::RefCell, cmp::Ordering, collections::BTreeMap, rc::Rc, vec},
};

/// What behaviour has this group been configured to exhibit?
#[derive(Clone, Debug)]
pub enum VersionGroupVariant {
  Banned,
  HighestSemver,
  Ignored,
  LowestSemver,
  Pinned,
  SameRange,
  SnappedTo,
}

#[derive(Debug)]
pub struct VersionGroup {
  /// Group instances of each dependency together for comparison.
  pub dependencies: BTreeMap<String, Dependency>,
  /// Does every instance match the filter options provided via the CLI?
  pub matches_cli_filter: bool,
  /// The version to pin all instances to when variant is `Pinned`
  pub pin_version: Option<Specifier>,
  /// Data to determine which instances should be added to this group
  pub selector: GroupSelector,
  /// package.json files whose names match the `snapTo` config when variant is
  /// `SnappedTo`
  pub snap_to: Option<Vec<Rc<RefCell<PackageJson>>>>,
  /// What behaviour has this group been configured to exhibit?
  pub variant: VersionGroupVariant,
}

impl VersionGroup {
  /// Create a default/catch-all group which would apply to any instance
  pub fn get_catch_all(all_dependency_types: &[DependencyType]) -> VersionGroup {
    VersionGroup {
      dependencies: BTreeMap::new(),
      matches_cli_filter: false,
      pin_version: None,
      selector: GroupSelector::new(
        /* all_packages: */ &Packages::new(),
        /* include_dependencies: */ vec![],
        /* include_dependency_types: */ vec![],
        /* label: */ "Default Version Group".to_string(),
        /* include_packages: */ vec![],
        /* include_specifier_types: */ vec![],
        /* all_dependency_types: */ all_dependency_types,
      ),
      snap_to: None,
      variant: VersionGroupVariant::HighestSemver,
    }
  }

  pub fn add_instance(&mut self, instance: Rc<Instance>) {
    let dependency = self
      .dependencies
      .entry(instance.descriptor.internal_name.clone())
      .or_insert_with(|| {
        Dependency::new(
          /* internal_name: */ instance.descriptor.internal_name.clone(),
          /* variant: */ self.variant.clone(),
          /* pin_version: */ self.pin_version.clone(),
          /* snap_to: */ self.snap_to.clone(),
        )
      });
    if instance.descriptor.name != dependency.internal_name {
      dependency.has_alias = true;
    }
    if instance.descriptor.matches_cli_filter {
      self.matches_cli_filter = true;
      dependency.matches_cli_filter = true;
    }
    dependency.add_instance(Rc::clone(&instance));
  }

  /// Create a single version group from a config item from the rcfile.
  pub fn from_config(group: &AnyVersionGroup, packages: &Packages, all_dependency_types: &[DependencyType]) -> VersionGroup {
    let selector = GroupSelector::new(
      /* all_packages: */ packages,
      /* include_dependencies: */ group.dependencies.clone(),
      /* include_dependency_types: */ group.dependency_types.clone(),
      /* label: */ group.label.clone(),
      /* include_packages: */ group.packages.clone(),
      /* include_specifier_types: */ group.specifier_types.clone(),
      /* all_dependency_types: */ all_dependency_types,
    );

    if let Some(true) = group.is_banned {
      return VersionGroup {
        dependencies: BTreeMap::new(),
        matches_cli_filter: false,
        pin_version: None,
        selector,
        snap_to: None,
        variant: VersionGroupVariant::Banned,
      };
    }
    if let Some(true) = group.is_ignored {
      return VersionGroup {
        dependencies: BTreeMap::new(),
        matches_cli_filter: false,
        pin_version: None,
        selector,
        snap_to: None,
        variant: VersionGroupVariant::Ignored,
      };
    }
    if let Some(pin_version) = &group.pin_version {
      return VersionGroup {
        dependencies: BTreeMap::new(),
        matches_cli_filter: false,
        pin_version: Some(Specifier::new(pin_version, None)),
        selector,
        snap_to: None,
        variant: VersionGroupVariant::Pinned,
      };
    }
    if let Some(policy) = &group.policy {
      if policy == "sameRange" {
        return VersionGroup {
          dependencies: BTreeMap::new(),
          matches_cli_filter: false,
          pin_version: None,
          selector,
          snap_to: None,
          variant: VersionGroupVariant::SameRange,
        };
      } else {
        // @FIXME: show user friendly error message and exit with error code
        panic!("Unrecognised version group policy: {policy}");
      }
    }
    if let Some(snap_to) = &group.snap_to {
      return VersionGroup {
        dependencies: BTreeMap::new(),
        matches_cli_filter: false,
        pin_version: None,
        selector,
        snap_to: Some(
          snap_to
            .iter()
            .flat_map(|name| {
              packages.get_by_name(name).or_else(|| {
                // @FIXME: show user friendly error message and exit with error code
                warn!("Invalid Snapped To Version Group: No package.json file found with a name property of '{name}'");
                None
              })
            })
            .collect(),
        ),
        variant: VersionGroupVariant::SnappedTo,
      };
    }
    if let Some(prefer_version) = &group.prefer_version {
      return VersionGroup {
        dependencies: BTreeMap::new(),
        matches_cli_filter: false,
        pin_version: None,
        selector,
        snap_to: None,
        variant: if prefer_version == "lowestSemver" {
          VersionGroupVariant::LowestSemver
        } else {
          VersionGroupVariant::HighestSemver
        },
      };
    }
    VersionGroup {
      dependencies: BTreeMap::new(),
      matches_cli_filter: false,
      pin_version: None,
      selector,
      snap_to: None,
      variant: VersionGroupVariant::HighestSemver,
    }
  }

  /// Returns a sorted iterator of each included dependency
  pub fn get_sorted_dependencies(&self, sort: &SortBy) -> impl Iterator<Item = &Dependency> {
    self
      .dependencies
      .values()
      .filter(|dependency| dependency.matches_cli_filter)
      .sorted_by(|a, b| match sort {
        SortBy::Count => b.instances.len().cmp(&a.instances.len()),
        SortBy::Name => Ordering::Equal,
      })
  }

  /// Get the registry urls for every dependency that we're able to update.
  /// Examples of dependencies we can't update are those inside banned or
  /// ignored version groups, or ones that are pinned to a specific version.
  pub fn get_update_urls(&self) -> Option<Vec<UpdateUrl>> {
    match self.variant {
      VersionGroupVariant::HighestSemver => Some(self.dependencies.values().filter_map(|dep| dep.get_update_url()).collect()),
      _ => None,
    }
  }

  pub fn has_ignored_variant(&self) -> bool {
    matches!(self.variant, VersionGroupVariant::Ignored)
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyVersionGroup {
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
  pub is_banned: Option<bool>,
  pub is_ignored: Option<bool>,
  pub pin_version: Option<String>,
  pub policy: Option<String>,
  pub snap_to: Option<Vec<String>>,
  pub prefer_version: Option<String>,
}
