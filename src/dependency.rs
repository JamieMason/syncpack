//! Dependency represents all instances of a single dependency name.
//!
//! For example, if "react" appears in package-a, package-b, and package-c,
//! all three instances are grouped under one Dependency struct.
//!
//! Key points:
//! - Dependency aggregates instances: Vec<Rc<Instance>>
//! - Each Dependency belongs to one VersionGroup
//! - The variant field determines validation behavior (Banned, Pinned, etc.)
//!
//! See src/version_group.rs for how dependencies are organized.

use {
  crate::{
    context::Context, instance::Instance, instance_state::InstanceState, package_json::PackageJson, specifier::Specifier,
    version_group::VersionGroupVariant,
  },
  itertools::Itertools,
  std::{cell::RefCell, cmp::Ordering, collections::HashMap, rc::Rc, vec},
};

#[cfg(test)]
#[path = "dependency_test.rs"]
mod dependency_test;

/// Information for fetching package metadata from npm registry.
/// Used by the update command to fetch available versions.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UpdateUrl {
  /// The name of the dependency as used in package.json
  pub internal_name: String,
  /// The actual npm package name (may differ from internal_name for aliases)
  pub package_name: String,
}

/// All instances of a single dependency name within a version group.
///
/// For example, if "react" appears in 5 different package.json files,
/// all 5 instances are collected here. Visitor functions iterate through
/// these instances and assign InstanceState based on the variant.
///
/// Wrapped fields use RefCell for interior mutability during inspection.
#[derive(Debug)]
pub struct Dependency {
  /// The expected version specifier which all instances of this dependency
  /// should be set to, in the event that they should all use the same version.
  /// RefCell allows mutation during inspection without &mut Context.
  pub expected: RefCell<Option<Rc<Specifier>>>,
  /// Whether the internal name for this dependency is an alias.
  pub has_alias: bool,
  /// Every instance of this dependency in this version group.
  /// Rc<Instance> allows cheap sharing without cloning.
  pub instances: Vec<Rc<Instance>>,
  /// If this dependency is a local package, this is the local instance.
  /// RefCell allows mutation during inspection.
  pub local_instance: RefCell<Option<Rc<Instance>>>,
  /// Does every instance match the filter options provided via the CLI?
  pub matches_cli_filter: bool,
  /// The name of the dependency, e.g., "react", "@types/node"
  pub internal_name: String,
  /// The version to pin all instances to when variant is `Pinned`
  pub pinned_specifier: Option<Rc<Specifier>>,
  /// package.json files developed in the monorepo when variant is `SnappedTo`.
  /// Rc<RefCell<T>> for shared ownership with interior mutability.
  pub snapped_to_packages: Option<Vec<Rc<RefCell<PackageJson>>>>,
  /// What behaviour has this group been configured to exhibit?
  /// Determines which visitor function processes this dependency.
  pub variant: VersionGroupVariant,
}

impl Dependency {
  pub fn new(
    internal_name: String,
    variant: VersionGroupVariant,
    pinned_specifier: Option<Rc<Specifier>>,
    snapped_to_packages: Option<Vec<Rc<RefCell<PackageJson>>>>,
  ) -> Dependency {
    Dependency {
      expected: RefCell::new(None),
      has_alias: false,
      instances: vec![],
      local_instance: RefCell::new(None),
      matches_cli_filter: false,
      internal_name,
      pinned_specifier,
      snapped_to_packages,
      variant,
    }
  }

  pub fn get_update_url(&self) -> Option<UpdateUrl> {
    if self.matches_cli_filter && self.internal_name_is_supported() {
      self.instances.iter().find_map(|instance| instance.get_update_url())
    } else {
      None
    }
  }

  pub fn add_instance(&mut self, instance: Rc<Instance>) {
    self.instances.push(Rc::clone(&instance));
    if instance.is_local {
      *self.local_instance.borrow_mut() = Some(Rc::clone(&instance));
    }
  }

  /// Return the most severe state of all instances in this group
  pub fn get_state(&self) -> InstanceState {
    self
      .instances
      .iter()
      .fold(InstanceState::Unknown, |acc, instance| acc.max(instance.state.borrow().clone()))
  }

  /// Return every instance state which applies to this group
  pub fn get_states(&self) -> Vec<InstanceState> {
    self
      .instances
      .iter()
      .map(|instance| instance.state.borrow().clone())
      .collect::<Vec<InstanceState>>()
  }

  /// Set the expected version specifier to the given value
  pub fn set_expected_specifier(&self, specifier: &Rc<Specifier>) -> &Self {
    *self.expected.borrow_mut() = Some(Rc::clone(specifier));
    self
  }

  /// Return the local instance's version specifier, if it exists
  pub fn get_local_specifier(&self) -> Option<Rc<Specifier>> {
    self
      .local_instance
      .borrow()
      .as_ref()
      .map(|instance| Rc::clone(&instance.descriptor.specifier))
  }

  /// Whether the dependency name is a valid npm package name, is invalid, or
  /// contains [pnpm overrides](https://pnpm.io/settings#overrides) syntax
  /// syncpack does not support yet.
  fn internal_name_is_supported(&self) -> bool {
    // Package name is supported if it doesn't contain:
    // 1. a '>' character (which would indicate pnpm overrides syntax)
    // 2. a '@' character which is not at index 0
    !self.internal_name.contains('>') && self.internal_name.rfind('@').unwrap_or(0) == 0
  }

  /// Is this dependency a package developed in this repository?
  pub fn has_local_instance(&self) -> bool {
    self.local_instance.borrow().is_some()
  }

  /// Is this dependency a package developed in this repository, which has a
  /// missing or invalid .version property?
  pub fn has_local_instance_with_invalid_specifier(&self) -> bool {
    self
      .get_local_specifier()
      .is_some_and(|local| !matches!(&*local, Specifier::Exact(_)))
  }

  /// Does every instance in this group have a specifier which is exactly the
  /// same?
  pub fn every_specifier_is_already_identical(&self) -> bool {
    if let Some(first_actual) = self.instances.first().map(|instance| &instance.descriptor.specifier) {
      self.instances.iter().all(|instance| {
        Rc::ptr_eq(&instance.descriptor.specifier, first_actual) || instance.descriptor.specifier.get_raw() == first_actual.get_raw()
      })
    } else {
      false
    }
  }

  pub fn get_unique_specifiers(&self) -> Vec<Rc<Specifier>> {
    let mut unique_specifiers = Vec::new();
    for instance in self.instances.iter() {
      let spec = &instance.descriptor.specifier;
      if !unique_specifiers.iter().any(|s: &Rc<Specifier>| s.get_raw() == spec.get_raw()) {
        unique_specifiers.push(Rc::clone(spec));
      }
    }
    unique_specifiers
  }

  /// Get the highest (or lowest) semver specifier in this group.
  ///
  /// When an instance belongs to a semver group, its preferred range is applied
  /// to produce an adjusted specifier before comparison. This means a semver
  /// group that widens a range (e.g. exact → caret) can promote that instance
  /// to become the highest via the range-greediness tiebreaker.
  pub fn get_highest_or_lowest_specifier(&self) -> Option<Rc<Specifier>> {
    let prefer_highest = matches!(self.variant, VersionGroupVariant::HighestSemver);
    let specifiers = self
      .get_instances()
      .filter(|instance| instance.descriptor.specifier.get_node_version().is_some())
      .map(|instance| {
        instance
          .preferred_semver_range
          .as_ref()
          .and_then(|range| instance.descriptor.specifier.with_range(range))
          .unwrap_or_else(|| Rc::clone(&instance.descriptor.specifier))
      });

    if prefer_highest {
      specifiers.max()
    } else {
      specifiers.min()
    }
  }

  /// Given a list of every available update, returns a map of each chosen
  /// update and the current specifiers which are affected by that update.
  ///
  /// When updating to the latest version, all of the current specifiers will be
  /// assigned to the same/latest version.
  ///
  /// When only applying eg. patch updates, some specifiers will be assigned to
  /// different updates if they are not on the same minor version.
  pub fn get_eligible_registry_updates(&self, ctx: &Context) -> Option<HashMap<String, Vec<Rc<Specifier>>>> {
    ctx.updates_by_internal_name.get(&self.internal_name).map(|updates| {
      let mut specifiers_by_eligible_update: HashMap<String, Vec<Rc<Specifier>>> = HashMap::new();
      self.get_unique_specifiers().iter().for_each(|installed| {
        updates
          .iter()
          .filter(|update| update.is_eligible_update_for(installed, &ctx.config.cli.target))
          // @TODO: make whether to do this configurable
          .filter(|update| installed.has_same_release_channel_as(update))
          .fold(None, |preferred, specifier| match preferred {
            None => Some(specifier),
            Some(preferred) => {
              if specifier.get_node_version().cmp(&preferred.get_node_version()) == Ordering::Greater {
                Some(specifier)
              } else {
                Some(preferred)
              }
            }
          })
          .inspect(|highest_update| {
            let key = highest_update.get_raw().to_string();
            let affected = specifiers_by_eligible_update.entry(key).or_default();
            affected.push(Rc::clone(installed));
          });
      });
      specifiers_by_eligible_update
    })
  }

  /// Return the first instance from the packages which should be snapped to for
  /// a given dependency
  ///
  /// We compare the expected (not actual) specifier because we're looking for
  /// what we should suggest as the correct specifier once `fix` is applied
  ///
  /// Even though the actual specifiers on disk might currently match, we should
  /// suggest it match what we the snapped to specifier should be once fixed
  pub fn get_snapped_to_specifier(&self, every_instance_in_the_project: &[Rc<Instance>]) -> Option<Rc<Specifier>> {
    if let Some(snapped_to_packages) = &self.snapped_to_packages {
      for instance in every_instance_in_the_project {
        if *instance.descriptor.internal_name == *self.internal_name {
          for snapped_to_package in snapped_to_packages {
            if instance.descriptor.package.borrow().name == snapped_to_package.borrow().name {
              return Some(Rc::clone(&instance.descriptor.specifier));
            }
          }
        }
      }
    }
    None
  }

  /// Returns an iterator of each included instance
  pub fn get_instances(&self) -> impl Iterator<Item = &Rc<Instance>> {
    self.instances.iter().filter(|instance| instance.descriptor.matches_cli_filter)
  }

  /// Returns an iterator of each included instance, sorted by:
  /// - Valid instances first
  /// - Highest version first
  /// - Package name A-Z when version is equal
  pub fn get_sorted_instances(&self) -> impl Iterator<Item = &Rc<Instance>> {
    self.get_instances().sorted_by(|a, b| {
      if a.is_valid() && !b.is_valid() {
        return Ordering::Less;
      }
      if b.is_valid() && !a.is_valid() {
        return Ordering::Greater;
      }
      if a.has_missing_specifier() {
        return Ordering::Greater;
      }
      if b.has_missing_specifier() {
        return Ordering::Less;
      }
      let specifier_order = b.descriptor.specifier.cmp(&a.descriptor.specifier);
      if matches!(specifier_order, Ordering::Equal) {
        a.descriptor.package.borrow().name.cmp(&b.descriptor.package.borrow().name)
      } else {
        specifier_order
      }
    })
  }
}
