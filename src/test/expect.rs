use {
  crate::{
    context::Context,
    instance::{Instance, InstanceState, Severity},
  },
  log::error,
};

#[derive(Debug)]
pub struct ExpectedInstance {
  /// Original specifier on disk.
  pub actual: &'static str,
  pub dependency_name: &'static str,
  /// Specifier syncpack determined the instance should have.
  pub expected: Option<&'static str>,
  pub id: &'static str,
  /// Set when a semver group overrode the version group's choice.
  pub overridden: Option<&'static str>,
  /// `None` means do not assert severity (lets existing tests stay green
  /// without per-assertion churn). `Some(s)` asserts the resolved severity
  /// equals `s`. See `.plans/severity.md` §2.
  pub severity: Option<Severity>,
  pub state: InstanceState,
}

#[derive(Debug)]
pub struct ActualInstance {
  /// Original specifier on disk.
  pub actual: String,
  pub dependency_name: String,
  /// Specifier syncpack determined the instance should have.
  pub expected: Option<String>,
  pub id: String,
  /// Set when a semver group overrode the version group's choice.
  pub overridden: Option<String>,
  pub severity: Option<Severity>,
  pub state: InstanceState,
}

impl ActualInstance {
  pub fn new(instance: &Instance) -> Self {
    Self {
      actual: instance.descriptor.specifier.get_raw().to_string(),
      dependency_name: instance.descriptor.internal_name.clone(),
      expected: instance
        .expected_specifier
        .borrow()
        .clone()
        .map(|expected| expected.get_raw().to_string()),
      id: instance.id.clone(),
      overridden: instance
        .get_specifier_with_preferred_semver_range()
        .map(|expected| expected.get_raw().to_string()),
      severity: *instance.severity.borrow(),
      state: instance.state.borrow().clone(),
    }
  }
}

pub fn expect(ctx: &Context) -> Expects<'_> {
  Expects::new(ctx)
}

pub struct Expects<'a> {
  pub ctx: &'a Context,
}

impl<'a> Expects<'a> {
  pub fn new(ctx: &'a Context) -> Self {
    Self { ctx }
  }

  pub fn to_have_instances(&self, expected_instances: Vec<ExpectedInstance>) -> &Self {
    let actual_instances = &self.ctx.instances.iter().map(ActualInstance::new).collect::<Vec<ActualInstance>>();
    let actual_len = actual_instances.len();
    let expected_len = expected_instances.len();
    if actual_len != expected_len {
      error!("expected {expected_len} instances but found {actual_len}");
      error!("expected instances: {expected_instances:#?}");
      error!("actual instances: {actual_instances:#?}");
      panic!("");
    }

    'expected: for expected in &expected_instances {
      let actual_specifier = expected.actual.to_string();
      let dependency_name = expected.dependency_name.to_string();
      let expected_specifier = expected.expected.map(|expected| expected.to_string());
      let overridden_specifier = expected.overridden.map(|overridden| overridden.to_string());
      let id = expected.id.to_string();
      let state = expected.state.clone();
      for actual in actual_instances.iter() {
        if actual.actual == actual_specifier
          && actual.dependency_name == dependency_name
          && actual.expected == expected_specifier
          && actual.id == id
          && actual.state == state
          && (expected.overridden.is_none() || actual.overridden == overridden_specifier)
          && (expected.severity.is_none() || actual.severity == expected.severity)
        {
          continue 'expected;
        }
      }
      error!("expected an instance {expected:#?} but it was not found");
      error!("actual instances: {actual_instances:#?}");
      panic!("");
    }
    self
  }
}
