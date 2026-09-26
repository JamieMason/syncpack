use {
  crate::{context::Inspected, specifier::Specifier},
  log::error,
  serde_json::Value,
  syncpack_model::instance::{Instance, InstanceIdx, InstanceState},
  syncpack_rules::inspection::{FormatMismatchVariant, Inspection},
  syncpack_vocab::severity::Severity,
};

#[derive(Debug)]
pub struct ExpectedFormatMismatch {
  /// The formatting rule that was broken.
  pub variant: FormatMismatchVariant,
  /// JSON pointer to the property that was linted.
  pub property_path: &'static str,
  /// The formatted value. Compared including key order, because
  /// `serde_json` with `preserve_order` ignores order in `Value` equality.
  pub expected: Value,
}

/// Are these two values identical including key order? `Value ==` ignores
/// order under `preserve_order`.
#[allow(clippy::cmp_owned)]
fn is_identical(a: &Value, b: &Value) -> bool {
  a.to_string() == b.to_string()
}

#[derive(Debug)]
pub struct ActualFormatMismatch {
  pub variant: FormatMismatchVariant,
  pub property_path: String,
  pub expected: Value,
}

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
  pub fn new(instance: &Instance, idx: InstanceIdx, inspection: &Inspection) -> Self {
    Self {
      actual: instance.descriptor.specifier.get_raw().to_string(),
      dependency_name: instance.descriptor.internal_name.clone(),
      expected: inspection.expected(idx).map(|expected| expected.get_raw().to_string()),
      id: instance.id.clone(),
      overridden: instance
        .get_specifier_with_preferred_semver_range()
        .map(|expected| expected.get_raw().to_string()),
      severity: inspection.severity(idx),
      state: inspection.state(idx).clone(),
    }
  }
}

/// One instance paired with its inspection results. Stands in for the
/// pre-refactor `instance.state` / `instance.expected_specifier` reads.
pub struct InstanceView<'a> {
  pub instance: &'a Instance,
  pub state: &'a InstanceState,
  pub expected: Option<&'a Specifier>,
}

impl Inspected {
  /// Every instance in discovery order with its inspected state and expected
  /// specifier.
  pub fn instance_views(&self) -> impl Iterator<Item = InstanceView<'_>> {
    self.data.instances.iter().enumerate().map(|(i, instance)| {
      let idx = InstanceIdx(i);
      InstanceView {
        instance,
        state: self.inspection.state(idx),
        expected: self.inspection.expected(idx).map(|expected| expected.as_ref()),
      }
    })
  }
}

pub fn expect(inspected: &Inspected) -> Expects<'_> {
  Expects::new(inspected)
}

pub struct Expects<'a> {
  pub inspected: &'a Inspected,
}

impl<'a> Expects<'a> {
  pub fn new(inspected: &'a Inspected) -> Self {
    Self { inspected }
  }

  pub fn to_have_instances(&self, expected_instances: Vec<ExpectedInstance>) -> &Self {
    let inspection = &self.inspected.inspection;
    let actual_instances = &self
      .inspected
      .data
      .instances
      .iter()
      .enumerate()
      .map(|(i, instance)| ActualInstance::new(instance, InstanceIdx(i), inspection))
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

  /// Assert the formatting mismatches found in the named package's
  /// `package.json`. An empty `expected_mismatches` asserts there are none.
  pub fn to_have_format_mismatches(&self, package_name: &str, expected_mismatches: Vec<ExpectedFormatMismatch>) -> &Self {
    let source_idx = self
      .inspected
      .data
      .sources
      .find_package(package_name)
      .unwrap_or_else(|| panic!("package not found: {package_name}"));
    let actual_mismatches = self.inspected.inspection.formatting[source_idx.0]
      .iter()
      .map(|mismatch| ActualFormatMismatch {
        variant: mismatch.variant.clone(),
        property_path: mismatch.property_path.clone(),
        expected: mismatch.expected.clone(),
      })
      .collect::<Vec<ActualFormatMismatch>>();
    let actual_len = actual_mismatches.len();
    let expected_len = expected_mismatches.len();
    if actual_len != expected_len {
      error!("expected {expected_len} format mismatches in {package_name} but found {actual_len}");
      error!("expected mismatches: {expected_mismatches:#?}");
      error!("actual mismatches: {actual_mismatches:#?}");
      panic!("");
    }

    'expected: for expected in &expected_mismatches {
      for actual in actual_mismatches.iter() {
        if actual.variant == expected.variant
          && actual.property_path == expected.property_path
          && is_identical(&actual.expected, &expected.expected)
        {
          continue 'expected;
        }
      }
      error!("expected a format mismatch {expected:#?} but it was not found");
      error!("actual mismatches: {actual_mismatches:#?}");
      panic!("");
    }
    self
  }
}
