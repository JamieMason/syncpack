#[cfg(test)]
#[path = "instance_test.rs"]
mod instance_test;

use {
  crate::{
    dependency::UpdateUrl,
    dependency_type::{DependencyType, Strategy},
    instance_state::{
      FixableInstance, InstanceState, InvalidInstance, SemverGroupAndVersionConflict, SuspectInstance, UnfixableInstance, ValidInstance,
    },
    package_json::PackageJson,
    specifier::{semver_range::SemverRange, Specifier},
  },
  log::debug,
  serde_json::Value,
  std::{cell::RefCell, rc::Rc},
};

pub type InstanceId = String;

#[derive(Debug)]
pub struct InstanceDescriptor {
  /// The dependency type to use to read/write this instance
  pub dependency_type: DependencyType,
  /// When a dependency group is used, its alias_name is used here in place of
  /// the actual dependency name (eg. "@aws-sdk/**" instead of "@aws-sdk/core"
  /// and "@aws-sdk/middleware-logger" etc.), otherwise the actual name is used.
  ///
  /// This aliased name is only used when allocating an `Instance` to a
  /// `Dependency`, the original name is otherwise preserved
  pub internal_name: String,
  /// Does this instance match the filter options provided via the CLI?
  pub matches_cli_filter: bool,
  /// The dependency name eg. "react", "react-dom"
  pub name: String,
  /// The package.json this instance belongs to
  pub package: Rc<RefCell<PackageJson>>,
  /// The original version specifier, which should never be mutated.
  /// eg. `Specifier::Exact("16.8.0")`, `Specifier::Range("^16.8.0")`
  pub specifier: Specifier,
}

#[derive(Debug)]
pub struct Instance {
  /// The original data this Instance is derived from
  pub descriptor: InstanceDescriptor,
  /// The version specifier which syncpack has determined this instance should
  /// be set to, if it was not possible to determine without user intervention,
  /// this will be a `None`.
  pub expected_specifier: RefCell<Option<Specifier>>,
  /// A unique identifier for this instance
  pub id: InstanceId,
  /// Whether this is a package developed in this repo
  pub is_local: bool,
  /// If this instance belongs to a `WithRange` semver group, this is the range.
  /// This is used by Version Groups while determining the preferred version,
  /// to try to also satisfy any applicable semver group ranges
  pub preferred_semver_range: Option<SemverRange>,
  /// The state of whether this instance has not been processed yet
  /// (InstanceState::Unknown) or when it has, what it was found to be
  pub state: RefCell<InstanceState>,
}

impl Instance {
  pub fn new(descriptor: InstanceDescriptor, preferred_semver_range: Option<SemverRange>) -> Instance {
    let dependency_type_name = &descriptor.dependency_type.path;
    let package_name = descriptor.package.borrow().name.clone();
    let id = format!("{} in {} of {}", &descriptor.name, dependency_type_name, package_name);
    let is_local = dependency_type_name == "/version";
    Instance {
      descriptor,
      expected_specifier: RefCell::new(None),
      id,
      is_local,
      preferred_semver_range,
      state: RefCell::new(InstanceState::Unknown),
    }
  }

  /// Record what syncpack has determined the state of this instance is and what
  /// its expected specifier should be
  fn set_state(&self, state: InstanceState, expected_specifier: &Specifier) -> &Self {
    *self.state.borrow_mut() = state;
    *self.expected_specifier.borrow_mut() = Some(expected_specifier.clone());
    self
  }

  /// Mark this instance as already having a valid specifier
  pub fn mark_valid(&self, state: ValidInstance, expected_specifier: &Specifier) -> &Self {
    self.set_state(InstanceState::Valid(state), expected_specifier)
  }

  /// Mark this instance as having something which doesn't look quite right, but
  /// for the moment is not yet resulting in an issue
  pub fn mark_suspect(&self, state: SuspectInstance) -> &Self {
    self.set_state(InstanceState::Suspect(state), &self.descriptor.specifier)
  }

  /// Mark this instance as having a mismatch which can be auto-fixed
  pub fn mark_fixable(&self, state: FixableInstance, expected_specifier: &Specifier) -> &Self {
    self.set_state(InstanceState::Invalid(InvalidInstance::Fixable(state)), expected_specifier)
  }

  /// Mark this instance as a mismatch which can't be auto-fixed, its semver
  /// group and version group config are in conflict with one another, asking
  /// for mutually exclusive versions
  pub fn mark_conflict(&self, state: SemverGroupAndVersionConflict) -> &Self {
    self.set_state(InstanceState::Invalid(InvalidInstance::Conflict(state)), &self.descriptor.specifier)
  }

  /// Mark this instance as a mismatch which can't be auto-fixed without user
  /// input
  pub fn mark_unfixable(&self, state: UnfixableInstance) -> &Self {
    self.set_state(
      InstanceState::Invalid(InvalidInstance::Unfixable(state)),
      &self.descriptor.specifier,
    )
  }

  pub fn is_valid(&self) -> bool {
    self.state.borrow().is_valid()
  }

  pub fn is_invalid(&self) -> bool {
    self.state.borrow().is_invalid()
  }

  pub fn is_suspect(&self) -> bool {
    self.state.borrow().is_suspect()
  }

  pub fn is_fixable(&self) -> bool {
    self.state.borrow().is_fixable()
  }

  pub fn is_banned(&self) -> bool {
    self.state.borrow().is_banned()
  }

  pub fn is_unfixable(&self) -> bool {
    self.state.borrow().is_unfixable()
  }

  pub fn is_outdated(&self) -> bool {
    self.state.borrow().is_outdated()
  }

  pub fn has_missing_specifier(&self) -> bool {
    matches!(self.descriptor.specifier, Specifier::None)
  }

  /// Does this instance's actual specifier match the expected specifier?
  pub fn already_equals(&self, expected: &Specifier) -> bool {
    self.descriptor.specifier.get_raw() == *expected.get_raw()
  }

  /// Does this instance belong to a `WithRange` semver group?
  pub fn must_match_preferred_semver_range(&self) -> bool {
    self.preferred_semver_range.is_some()
  }

  /// Does this instance belong to a `WithRange` semver group and which prefers
  /// a semver range other than the given range?
  ///
  /// This is a convenience method for the common case where a preferred semver
  /// range only matters if what is preferred is not the same as the expected
  /// version of a dependency which you are trying to synchronise to
  pub fn must_match_preferred_semver_range_which_is_not(&self, needed_range: &SemverRange) -> bool {
    self.must_match_preferred_semver_range() && !self.preferred_semver_range_is(needed_range)
  }

  /// Does this instance belong to a `WithRange` semver group and which prefers
  /// a semver range other than that used by the given specifier?
  pub fn must_match_preferred_semver_range_which_differs_to(&self, other_specifier: &Specifier) -> bool {
    other_specifier
      .get_semver_range()
      .is_some_and(|range_of_other_specifier| self.must_match_preferred_semver_range_which_is_not(range_of_other_specifier))
  }

  /// Is the given semver range the preferred semver range for this instance?
  pub fn preferred_semver_range_is(&self, range: &SemverRange) -> bool {
    self.preferred_semver_range.as_ref().map(|r| r == range).unwrap_or(false)
  }

  /// Does this instance belong to a `WithRange` semver group and also have a
  /// specifier which matches its preferred semver range?
  pub fn matches_preferred_semver_range(&self) -> bool {
    self
      .preferred_semver_range
      .as_ref()
      .map(|preferred_semver_range| self.descriptor.specifier.has_semver_range_of(preferred_semver_range))
      .unwrap_or(false)
  }

  /// Get the expected version specifier for this instance with the semver
  /// group's preferred range applied
  pub fn get_specifier_with_preferred_semver_range(&self) -> Option<Specifier> {
    self
      .preferred_semver_range
      .as_ref()
      .map(|preferred_semver_range| self.descriptor.specifier.clone().with_range(preferred_semver_range))
  }

  pub fn get_update_url(&self) -> Option<UpdateUrl> {
    if self.descriptor.matches_cli_filter && !self.is_local {
      let internal_name = &self.descriptor.internal_name;
      let actual_name = &self.descriptor.name;
      let raw = self.descriptor.specifier.get_raw();
      match &self.descriptor.specifier {
        Specifier::Alias(alias) => {
          if let Some(aliased_name) = alias.extract_package_name() {
            if aliased_name.starts_with("@jsr/") {
              Some(UpdateUrl {
                internal_name: internal_name.clone(),
                url: format!("https://npm.jsr.io/{}", aliased_name),
              })
            } else if &aliased_name == actual_name {
              Some(UpdateUrl {
                internal_name: internal_name.clone(),
                url: format!("https://registry.npmjs.org/{}", actual_name),
              })
            } else {
              debug!("'{aliased_name}' in '{raw}' does not equal the instance name '{actual_name}', skipping update as this might create mismatches");
              None
            }
          } else {
            None
          }
        }
        Specifier::BasicSemver(_) => {
          if actual_name.starts_with("@jsr/") {
            Some(UpdateUrl {
              internal_name: internal_name.clone(),
              url: format!("https://npm.jsr.io/{}", actual_name),
            })
          } else {
            Some(UpdateUrl {
              internal_name: internal_name.clone(),
              url: format!("https://registry.npmjs.org/{}", actual_name),
            })
          }
        }
        _ => None,
      }
    } else {
      None
    }
  }

  /// Does this instance's specifier match the specifier of every one of the
  /// given instances?
  pub fn already_satisfies_all(&self, instances: &[Rc<Instance>]) -> bool {
    !matches!(self.descriptor.specifier, Specifier::None)
      && self
        .descriptor
        .specifier
        .satisfies_all(instances.iter().map(|i| &i.descriptor.specifier).collect())
  }

  /// Will this instance's specifier, once fixed to match its semver group,
  /// satisfy the given specifier?
  pub fn specifier_with_preferred_semver_range_will_satisfy(&self, other: &Specifier) -> bool {
    self
      .get_specifier_with_preferred_semver_range()
      .map(|specifier| specifier.satisfies(other))
      .unwrap_or(false)
  }

  /// Delete from the package.json
  pub fn remove(&self) {
    match self.descriptor.dependency_type.strategy {
      Strategy::NameAndVersionProps => {
        let path_to_prop = &self.descriptor.dependency_type.path;
        if let Some(parent_path) = path_to_prop.rfind('/') {
          let parent_path = &path_to_prop[..parent_path];
          let prop_name = &path_to_prop[parent_path.len() + 1..];
          if let Some(Value::Object(obj)) = self.descriptor.package.borrow_mut().contents.borrow_mut().pointer_mut(parent_path) {
            obj.remove(prop_name);
          }
        } else if path_to_prop == "/" {
          debug!("Cannot remove root property for NameAndVersionProps");
        }
      }
      Strategy::NamedVersionString => {
        let path_to_prop = &self.descriptor.dependency_type.path;
        if let Some(parent_path) = path_to_prop.rfind('/') {
          let parent_path = &path_to_prop[..parent_path];
          let prop_name = &path_to_prop[parent_path.len() + 1..];
          if let Some(Value::Object(obj)) = self.descriptor.package.borrow_mut().contents.borrow_mut().pointer_mut(parent_path) {
            obj.remove(prop_name);
          }
        } else if path_to_prop == "/" {
          debug!("Cannot remove root property for NamedVersionString");
        }
      }
      Strategy::UnnamedVersionString => {
        let path_to_prop = &self.descriptor.dependency_type.path;
        if let Some(parent_path) = path_to_prop.rfind('/') {
          let parent_path = &path_to_prop[..parent_path];
          let prop_name = &path_to_prop[parent_path.len() + 1..];
          if let Some(Value::Object(obj)) = self.descriptor.package.borrow_mut().contents.borrow_mut().pointer_mut(parent_path) {
            obj.remove(prop_name);
          }
        } else if path_to_prop == "/" {
          debug!("Cannot remove root property for UnnamedVersionString");
        }
      }
      Strategy::VersionsByName => {
        let path_to_obj = &self.descriptor.dependency_type.path;
        let name = &self.descriptor.name;
        if let Some(Value::Object(obj)) = self.descriptor.package.borrow_mut().contents.borrow_mut().pointer_mut(path_to_obj) {
          obj.remove(name);
        }
      }
      Strategy::InvalidConfig => {
        panic!("unrecognised strategy");
      }
    };
  }
}
