use {
  crate::{
    instance::{Instance, InstanceIdx, InstanceState, InvalidInstance, SuspectInstance},
    source::FormatMismatch,
    sources::{SourceIdx, Sources},
    specifier::Specifier,
    version_group::{Dependency, DependencyIdx},
  },
  std::rc::Rc,
  syncpack_specifier::update_target::UpdateTarget,
  syncpack_vocab::{
    package_manager::PackageManager,
    severity::{Severity, SeverityMap},
  },
};

/// The read-only view passed to every rule in place of `&Context`. Rules read
/// discovered project data plus the handful of config scalars they need; they
/// never mutate the model.
pub struct RuleInput<'a> {
  pub instances: &'a [Instance],
  pub dependencies: &'a [Dependency],
  pub sources: &'a Sources,
  pub cfg: RuleInputCfg,
}

/// The config scalars rules read, lifted out of `Config` so rules name no
/// rcfile schema type.
#[derive(Clone, Copy, Debug)]
pub struct RuleInputCfg {
  pub target: UpdateTarget,
  pub strict: bool,
  pub package_manager: Option<PackageManager>,
}

/// Every per-entity result produced by inspection, returned as a value.
/// Replaces the interior mutability on `Instance`/`Dependency` and the
/// `visit_formatting` write-back into `Source`.
#[derive(Debug)]
pub struct Inspection {
  /// By `InstanceIdx.0`.
  pub states: Vec<InstanceState>,
  /// By `InstanceIdx.0`.
  pub expected: Vec<Option<Rc<Specifier>>>,
  /// By `InstanceIdx.0`. Resolved at the moment the state is decided.
  pub severities: Vec<Option<Severity>>,
  /// By `DependencyIdx.0`.
  pub dep_expected: Vec<Option<Rc<Specifier>>>,
  /// By `SourceIdx.0`.
  pub formatting: Vec<Vec<FormatMismatch>>,
}

impl Inspection {
  pub fn new(instance_count: usize, dependency_count: usize, source_count: usize) -> Self {
    Self {
      states: (0..instance_count).map(|_| InstanceState::Unknown).collect(),
      expected: (0..instance_count).map(|_| None).collect(),
      severities: (0..instance_count).map(|_| None).collect(),
      dep_expected: (0..dependency_count).map(|_| None).collect(),
      formatting: (0..source_count).map(|_| Vec::new()).collect(),
    }
  }

  /// Record an instance's state, expected specifier and resolved severity in
  /// one call. Severity is resolved here, at the point the state is decided,
  /// so results are never half-populated.
  pub fn classify(
    &mut self,
    idx: InstanceIdx,
    state: InstanceState,
    expected: Option<Rc<Specifier>>,
    severity_map: &SeverityMap,
    strict: bool,
  ) {
    self.severities[idx.0] = severity_of(&state, severity_map, strict);
    self.states[idx.0] = state;
    self.expected[idx.0] = expected;
  }

  pub fn set_dep_expected(&mut self, dep: DependencyIdx, specifier: Rc<Specifier>) {
    self.dep_expected[dep.0] = Some(specifier);
  }

  pub fn add_format_mismatch(&mut self, source: SourceIdx, mismatch: FormatMismatch) {
    self.formatting[source.0].push(mismatch);
  }

  pub fn state(&self, idx: InstanceIdx) -> &InstanceState {
    &self.states[idx.0]
  }

  pub fn expected(&self, idx: InstanceIdx) -> Option<&Rc<Specifier>> {
    self.expected[idx.0].as_ref()
  }

  pub fn severity(&self, idx: InstanceIdx) -> Option<Severity> {
    self.severities[idx.0]
  }
}

/// Resolve the severity for a state against a group's configured severity map
/// and the rcfile's `strict` flag. Pure; called at the moment a rule decides a
/// state. `None` for `Unknown`/`Valid` states.
pub fn severity_of(state: &InstanceState, severity_map: &SeverityMap, strict: bool) -> Option<Severity> {
  match state {
    InstanceState::Unknown | InstanceState::Valid(_) => Some(Severity::None),
    InstanceState::Invalid(InvalidInstance::Unfixable(_)) | InstanceState::Invalid(InvalidInstance::Conflict(_)) => Some(Severity::Error),
    InstanceState::Invalid(InvalidInstance::Fixable(_)) => {
      let key = state.get_name();
      Some(severity_map.get(&key).copied().unwrap_or(Severity::Fix))
    }
    InstanceState::Suspect(SuspectInstance::RefuseToPinLocal) | InstanceState::Suspect(SuspectInstance::RefuseToSnapLocal) => {
      let key = state.get_name();
      let default = if strict { Severity::Error } else { Severity::Warn };
      Some(severity_map.get(&key).copied().unwrap_or(default))
    }
    InstanceState::Suspect(_) => Some(if strict { Severity::Error } else { Severity::Warn }),
  }
}
