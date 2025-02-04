use {
  crate::{
    dependency_type::{DependencyType, Strategy},
    instance_state::{
      FixableInstance, InstanceState, InvalidInstance, SemverGroupAndVersionConflict, SuspectInstance, UnfixableInstance, ValidInstance,
    },
    package_json::PackageJson,
    semver_group::SemverGroup,
    specifier::{semver::Semver, semver_range::SemverRange, Specifier},
  },
  log::debug,
  serde_json::Value,
  std::{cell::RefCell, path::PathBuf, rc::Rc},
};

pub type InstanceId = String;

#[derive(Debug)]
pub struct Instance {
  /// The original version specifier, which should never be mutated.
  /// eg. `Specifier::Exact("16.8.0")`, `Specifier::Range("^16.8.0")`
  pub actual_specifier: Specifier,
  /// The dependency type to use to read/write this instance
  pub dependency_type: DependencyType,
  /// The version specifier which syncpack has determined this instance should
  /// be set to, if it was not possible to determine without user intervention,
  /// this will be a `None`.
  pub expected_specifier: RefCell<Option<Specifier>>,
  /// The file path of the package.json file this instance belongs to
  pub file_path: PathBuf,
  /// A unique identifier for this instance
  pub id: InstanceId,
  /// Whether this is a package developed in this repo
  pub is_local: bool,
  /// Does this instance match the filter options provided via the CLI?
  pub matches_cli_filter: RefCell<bool>,
  /// The dependency name eg. "react", "react-dom"
  pub name: String,
  /// The `.name` of the package.json this file is in
  pub package: Rc<RefCell<PackageJson>>,
  /// If this instance belongs to a `WithRange` semver group, this is the range.
  /// This is used by Version Groups while determining the preferred version,
  /// to try to also satisfy any applicable semver group ranges
  pub preferred_semver_range: RefCell<Option<SemverRange>>,
  /// The state of whether this instance has not been processed yet
  /// (InstanceState::Unknown) or when it has, what it was found to be
  pub state: RefCell<InstanceState>,
}

impl Instance {
  pub fn new(
    name: String,
    // The initial, unwrapped specifier (eg. "1.1.0") from the package.json file
    raw_specifier: String,
    dependency_type: &DependencyType,
    package: Rc<RefCell<PackageJson>>,
  ) -> Instance {
    let package_name = package.borrow().get_name_unsafe();
    let specifier = Specifier::new(&raw_specifier);
    Instance {
      actual_specifier: specifier.clone(),
      dependency_type: dependency_type.clone(),
      expected_specifier: RefCell::new(None),
      file_path: package.borrow().file_path.clone(),
      id: format!("{} in {} of {}", name, &dependency_type.path, package_name),
      is_local: dependency_type.path == "/version",
      matches_cli_filter: RefCell::new(false),
      name,
      package: Rc::clone(&package),
      preferred_semver_range: RefCell::new(None),
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
    self.set_state(InstanceState::Suspect(state), &self.actual_specifier)
  }

  /// Mark this instance as having a mismatch which can be auto-fixed
  pub fn mark_fixable(&self, state: FixableInstance, expected_specifier: &Specifier) -> &Self {
    self.set_state(InstanceState::Invalid(InvalidInstance::Fixable(state)), expected_specifier)
  }

  /// Mark this instance as a mismatch which can't be auto-fixed, its semver
  /// group and version group config are in conflict with one another, asking
  /// for mutually exclusive versions
  pub fn mark_conflict(&self, state: SemverGroupAndVersionConflict) -> &Self {
    self.set_state(InstanceState::Invalid(InvalidInstance::Conflict(state)), &self.actual_specifier)
  }

  /// Mark this instance as a mismatch which can't be auto-fixed without user
  /// input
  pub fn mark_unfixable(&self, state: UnfixableInstance) -> &Self {
    self.set_state(InstanceState::Invalid(InvalidInstance::Unfixable(state)), &self.actual_specifier)
  }

  /// If this instance should use a preferred semver range, store it
  pub fn set_semver_group(&self, group: &SemverGroup) {
    if let Some(range) = &group.range {
      *self.preferred_semver_range.borrow_mut() = Some(range.clone());
    }
  }

  /// Does this instance's actual specifier match the expected specifier?
  pub fn already_equals(&self, expected: &Specifier) -> bool {
    self.actual_specifier == *expected
  }

  /// Does this instance belong to a `WithRange` semver group?
  pub fn must_match_preferred_semver_range(&self) -> bool {
    self.preferred_semver_range.borrow().is_some()
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
      .is_some_and(|range_of_other_specifier| self.must_match_preferred_semver_range_which_is_not(&range_of_other_specifier))
  }

  /// Is the given semver range the preferred semver range for this instance?
  pub fn preferred_semver_range_is(&self, range: &SemverRange) -> bool {
    self.preferred_semver_range.borrow().as_ref().map(|r| r == range).unwrap_or(false)
  }

  /// Does this instance belong to a `WithRange` semver group and also have a
  /// specifier which matches its preferred semver range?
  pub fn matches_preferred_semver_range(&self) -> bool {
    self
      .preferred_semver_range
      .borrow()
      .as_ref()
      .map(|preferred_semver_range| self.actual_specifier.has_semver_range_of(preferred_semver_range))
      .unwrap_or(false)
  }

  /// Get the expected version specifier for this instance with the semver
  /// group's preferred range applied
  pub fn get_specifier_with_preferred_semver_range(&self) -> Option<Specifier> {
    self.preferred_semver_range.borrow().as_ref().and_then(|preferred_semver_range| {
      self
        .actual_specifier
        .get_simple_semver()
        .map(|actual| Specifier::Semver(Semver::Simple(actual.with_range(preferred_semver_range))))
    })
  }

  /// Does this instance's specifier match the specifier of every one of the
  /// given instances?
  pub fn already_satisfies_all(&self, instances: &[Rc<Instance>]) -> bool {
    !matches!(self.actual_specifier, Specifier::None)
      && self
        .actual_specifier
        .satisfies_all(instances.iter().map(|i| &i.actual_specifier).collect())
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
    match self.dependency_type.strategy {
      Strategy::NameAndVersionProps => {
        debug!("@TODO: remove instance for NameAndVersionProps");
      }
      Strategy::NamedVersionString => {
        debug!("@TODO: remove instance for NamedVersionString");
      }
      Strategy::UnnamedVersionString => {
        debug!("@TODO: remove instance for UnnamedVersionString");
      }
      Strategy::VersionsByName => {
        let path_to_obj = &self.dependency_type.path;
        let name = &self.name;
        if let Some(Value::Object(obj)) = self.package.borrow_mut().contents.borrow_mut().pointer_mut(path_to_obj) {
          obj.remove(name);
        }
      }
      Strategy::InvalidConfig => {
        panic!("unrecognised strategy");
      }
    };
  }
}
