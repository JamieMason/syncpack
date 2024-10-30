use {
  crate::{context::Context, instance::Instance, instance_state::InstanceState},
  log::error,
};

#[derive(Debug)]
pub struct ExpectedInstance {
  /// The original version on disk
  pub actual: &'static str,
  /// eg "react-dom"
  pub dependency_name: &'static str,
  /// The specifier syncpack determined the instance should have
  pub expected: Option<&'static str>,
  /// The instance id
  pub id: &'static str,
  /// In the case of a semver group being overridden
  pub overridden: Option<&'static str>,
  /// The error or valid state syncpack determined the instance is in
  pub state: InstanceState,
}

#[derive(Debug)]
pub struct ActualInstance {
  /// The original version on disk
  pub actual: String,
  /// eg "react-dom"
  pub dependency_name: String,
  /// The specifier syncpack determined the instance should have
  pub expected: Option<String>,
  /// The instance id
  pub id: String,
  /// In the case of a semver group being overridden
  pub overridden: Option<String>,
  /// The error or valid state syncpack determined the instance is in
  pub state: InstanceState,
}

impl ActualInstance {
  pub fn new(instance: &Instance) -> Self {
    Self {
      actual: instance.actual_specifier.unwrap(),
      dependency_name: instance.name.clone(),
      expected: instance.expected_specifier.borrow().clone().map(|expected| expected.unwrap()),
      id: instance.id.clone(),
      overridden: instance
        .get_specifier_with_preferred_semver_range()
        .clone()
        .map(|expected| expected.unwrap()),
      state: instance.state.borrow().clone(),
    }
  }
}

pub fn expect(ctx: &Context) -> Expects {
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
    let actual_instances = &self
      .ctx
      .instances
      .iter()
      .map(|instance| ActualInstance::new(instance))
      .collect::<Vec<ActualInstance>>();
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
