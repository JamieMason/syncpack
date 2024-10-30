use {
  crate::{
    instance::Instance,
    instance_state::InstanceState,
    package_json::PackageJson,
    specifier::{orderable::IsOrderable, semver::Semver, simple_semver::SimpleSemver, Specifier},
    version_group::VersionGroupVariant,
  },
  itertools::Itertools,
  std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{BTreeMap, HashSet},
    rc::Rc,
    vec,
  },
};

#[derive(Debug)]
pub struct Dependency {
  /// The expected version specifier which all instances of this dependency
  /// should be set to, in the event that they should all use the same version.
  pub expected: RefCell<Option<Specifier>>,
  /// Every instance of this dependency in this version group.
  pub instances: RefCell<Vec<Rc<Instance>>>,
  /// If this dependency is a local package, this is the local instance.
  pub local_instance: RefCell<Option<Rc<Instance>>>,
  /// Does every instance match the filter options provided via the CLI?
  pub matches_cli_filter: RefCell<bool>,
  /// The name of the dependency
  pub name: String,
  /// The version to pin all instances to when variant is `Pinned`
  pub pinned_specifier: Option<Specifier>,
  /// package.json files developed in the monorepo when variant is `SnappedTo`
  pub snapped_to_packages: Option<Vec<Rc<RefCell<PackageJson>>>>,
  /// What behaviour has this group been configured to exhibit?
  pub variant: VersionGroupVariant,
}

impl Dependency {
  pub fn new(
    name: String,
    variant: VersionGroupVariant,
    pinned_specifier: Option<Specifier>,
    snapped_to_packages: Option<Vec<Rc<RefCell<PackageJson>>>>,
  ) -> Dependency {
    Dependency {
      expected: RefCell::new(None),
      instances: RefCell::new(vec![]),
      local_instance: RefCell::new(None),
      matches_cli_filter: RefCell::new(false),
      name,
      pinned_specifier,
      snapped_to_packages,
      variant,
    }
  }

  pub fn add_instance(&self, instance: Rc<Instance>) {
    self.instances.borrow_mut().push(Rc::clone(&instance));
    if instance.is_local {
      *self.local_instance.borrow_mut() = Some(Rc::clone(&instance));
    }
  }

  pub fn get_unique_specifiers(&self) -> Vec<Specifier> {
    let set: HashSet<Specifier> = self
      .instances
      .borrow()
      .iter()
      .map(|instance| instance.actual_specifier.clone())
      .collect();
    set.into_iter().collect()
  }

  /// Return the most severe state of all instances in this group
  pub fn get_state(&self) -> InstanceState {
    self
      .instances
      .borrow()
      .iter()
      .fold(InstanceState::Unknown, |acc, instance| acc.max(instance.state.borrow().clone()))
  }

  /// Return every unique instance state which applies to this group
  pub fn get_states(&self) -> Vec<InstanceState> {
    self
      .instances
      .borrow()
      .iter()
      .map(|instance| instance.state.borrow().clone())
      .collect::<Vec<_>>()
  }

  pub fn get_instances_by_specifier(&self) -> BTreeMap<String, Vec<Rc<Instance>>> {
    let mut map = BTreeMap::new();
    for instance in self.instances.borrow().iter() {
      let specifier = instance.actual_specifier.unwrap();
      map.entry(specifier).or_insert_with(Vec::new).push(Rc::clone(instance));
    }
    map
  }

  pub fn set_expected_specifier(&self, specifier: &Specifier) -> &Self {
    *self.expected.borrow_mut() = Some(specifier.clone());
    self
  }

  pub fn get_local_specifier(&self) -> Option<Specifier> {
    self
      .local_instance
      .borrow()
      .as_ref()
      .map(|instance| instance.actual_specifier.clone())
  }

  pub fn has_local_instance(&self) -> bool {
    self.local_instance.borrow().is_some()
  }

  pub fn has_local_instance_with_invalid_specifier(&self) -> bool {
    self.has_local_instance()
      && !matches!(
        self.get_local_specifier().unwrap(),
        Specifier::Semver(Semver::Simple(SimpleSemver::Exact(_)))
      )
  }

  /// Does every instance in this group have a specifier which is exactly the
  /// same?
  pub fn every_specifier_is_already_identical(&self) -> bool {
    if let Some(first_actual) = self.instances.borrow().first().map(|instance| &instance.actual_specifier) {
      self
        .instances
        .borrow()
        .iter()
        .all(|instance| instance.actual_specifier == *first_actual)
    } else {
      false
    }
  }

  /// Get the highest (or lowest) semver specifier in this group.
  pub fn get_highest_or_lowest_specifier(&self) -> Option<Specifier> {
    let prefer_highest = matches!(self.variant, VersionGroupVariant::HighestSemver);
    let preferred_order = if prefer_highest { Ordering::Greater } else { Ordering::Less };
    self
      .instances
      .borrow()
      .iter()
      .filter(|instance| instance.actual_specifier.is_simple_semver())
      .map(|instance| instance.actual_specifier.clone())
      .fold(None, |preferred, specifier| match preferred {
        None => Some(specifier),
        Some(preferred) => {
          let a = specifier.get_orderable();
          let b = preferred.get_orderable();
          if a.cmp(&b) == preferred_order {
            Some(specifier.clone())
          } else {
            Some(preferred)
          }
        }
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
  pub fn get_snapped_to_specifier(&self, every_instance_in_the_project: &[Rc<Instance>]) -> Option<Specifier> {
    if let Some(snapped_to_packages) = &self.snapped_to_packages {
      for instance in every_instance_in_the_project {
        if instance.name == *self.name {
          for snapped_to_package in snapped_to_packages {
            if instance.package.borrow().get_name_unsafe() == snapped_to_package.borrow().get_name_unsafe() {
              return Some(instance.actual_specifier.clone());
            }
          }
        }
      }
    }
    None
  }

  /// Iterate over every instance in this group, sorted by:
  /// - Valid instances first
  /// - Highest version first
  /// - Package name A-Z when version is equal
  pub fn for_each_instance(&self, f: impl Fn(&Rc<Instance>)) {
    self
      .instances
      .borrow()
      .iter()
      .sorted_by(|a, b| {
        if matches!(*a.state.borrow(), InstanceState::Valid(_)) && !matches!(*b.state.borrow(), InstanceState::Valid(_)) {
          return Ordering::Less;
        }
        if matches!(*b.state.borrow(), InstanceState::Valid(_)) && !matches!(*a.state.borrow(), InstanceState::Valid(_)) {
          return Ordering::Greater;
        }
        if matches!(&a.actual_specifier, Specifier::None) {
          return Ordering::Greater;
        }
        if matches!(&b.actual_specifier, Specifier::None) {
          return Ordering::Less;
        }
        let specifier_order = b.actual_specifier.unwrap().cmp(&a.actual_specifier.unwrap());
        if matches!(specifier_order, Ordering::Equal) {
          a.package.borrow().get_name_unsafe().cmp(&b.package.borrow().get_name_unsafe())
        } else {
          specifier_order
        }
      })
      .for_each(f);
  }
}
