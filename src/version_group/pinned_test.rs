use {
  crate::{
    instance::{FixableInstance::*, InstanceState, SuspectInstance::*, ValidInstance::*},
    test::{
      builder::TestBuilder,
      expect::{ExpectedInstance, expect},
    },
  },
  serde_json::json,
};

mod local {
  use super::*;

  #[tokio::test]
  async fn refuses_to_pin_local_version() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({
          "name": "package-a",
          "version": "1.0.0"
        }),
        json!({
          "name": "package-b",
          "dependencies": {
            "package-a": "1.1.0"
          }
        }),
      ])
      .with_version_group(json!({
        "dependencies": ["package-a"],
        "pinVersion": "1.2.0"
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(RefuseToPinLocal),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.2.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToPin),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "1.1.0",
        expected: Some("1.2.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }
}

mod normal {
  use super::*;

  #[tokio::test]
  async fn a_pinned_version_will_replace_anything_different() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0",
        "devDependencies": {
          "foo": "workspace:*"
        }
      }))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "pinVersion": "1.2.0"
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToPin),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: "workspace:*",
        expected: Some("1.2.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn pin_version_will_override_instance_with_same_version_number_as_pinned_but_matching_a_semver_group() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0",
        "devDependencies": {
          "foo": "^1.0.0"
        }
      }))
      .with_semver_group(json!({
        "range": "^",
        "dependencies": ["foo"]
      }))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "pinVersion": "1.0.0"
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::fixable(PinOverridesSemverRange),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: "^1.0.0",
        expected: Some("1.0.0"),
        overridden: Some("^1.0.0"),
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn pin_version_will_override_instance_with_same_version_number_as_pinned_but_mismatching_a_semver_group() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0",
        "devDependencies": {
          "foo": ">=1.0.0"
        }
      }))
      .with_semver_group(json!({
        "range": "^",
        "dependencies": ["foo"]
      }))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "pinVersion": "1.0.0"
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::fixable(PinOverridesSemverRangeMismatch),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: ">=1.0.0",
        expected: Some("1.0.0"),
        overridden: Some("^1.0.0"),
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn pin_version_will_override_instance_with_same_version_number_as_pinned_but_a_different_range_and_no_semver_group() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0",
        "devDependencies": {
          "foo": "^1.0.0"
        }
      }))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "pinVersion": "1.0.0"
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToPin),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: "^1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn an_already_pinned_version_is_valid() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0",
        "devDependencies": {
          "foo": "1.2.0"
        }
      }))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "pinVersion": "1.2.0"
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsIdenticalToPin),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: "1.2.0",
        expected: Some("1.2.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn an_already_pinned_workspace_protocol_version_is_valid() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0",
        "devDependencies": {
          "package-a": "workspace:*"
        }
      }))
      .with_version_group(json!({
        "dependencies": ["package-a"],
        "dependencyTypes": ["dev"],
        "pinVersion": "workspace:*"
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsIdenticalToPin),
        dependency_name: "package-a",
        id: "package-a in /devDependencies of package-a",
        actual: "workspace:*",
        expected: Some("workspace:*"),
        overridden: None,
        severity: None,
      },
    ]);
  }
}

/// Severity tests — opt out of auto-fix per status (issue #216), opt into
/// pinning local (issue #325). Pinned permits `DiffersToPin`,
/// `PinOverridesSemverRange`, `PinOverridesSemverRangeMismatch`,
/// `RefuseToPinLocal`.
mod severity {
  use {
    super::*,
    crate::{commands::json::instance_to_json, instance::Severity},
  };

  /// Default Fixable severity is `Fix`. Pins existing behaviour so it does
  /// not regress under the new resolver.
  #[tokio::test]
  async fn differs_to_pin_default_severity_is_fix() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0",
        "devDependencies": {"foo": "1.0.0"}
      }))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "pinVersion": "1.2.0"
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToPin),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: "1.0.0",
        expected: Some("1.2.0"),
        overridden: None,
        severity: Some(Severity::Fix),
      },
    ]);
  }

  /// `severity: { DiffersToPin: "warn" }` resolves the Fixable to `Warn`.
  #[tokio::test]
  async fn differs_to_pin_warn() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0",
        "devDependencies": {"foo": "1.0.0"}
      }))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "pinVersion": "1.2.0",
        "severity": {"DiffersToPin": "warn"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToPin),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: "1.0.0",
        expected: Some("1.2.0"),
        overridden: None,
        severity: Some(Severity::Warn),
      },
    ]);
  }

  /// `severity: { DiffersToPin: "error" }` resolves to `Error`.
  #[tokio::test]
  async fn differs_to_pin_error() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0",
        "devDependencies": {"foo": "1.0.0"}
      }))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "pinVersion": "1.2.0",
        "severity": {"DiffersToPin": "error"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToPin),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: "1.0.0",
        expected: Some("1.2.0"),
        overridden: None,
        severity: Some(Severity::Error),
      },
    ]);
  }

  /// Default `RefuseToPinLocal` under `!strict` resolves to `Warn`. This is
  /// the BREAKING visibility change — today the Suspect is hidden from lint
  /// under !strict, planned behaviour surfaces it as a warning.
  #[tokio::test]
  async fn refuse_to_pin_local_defaults_to_warn_under_non_strict() {
    let ctx = TestBuilder::new()
      .with_package(json!({"name": "package-a", "version": "1.0.0"}))
      .with_version_group(json!({
        "dependencies": ["package-a"],
        "pinVersion": "1.2.0"
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![ExpectedInstance {
      state: InstanceState::suspect(RefuseToPinLocal),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.2.0"),
      overridden: None,
      severity: Some(Severity::Warn),
    }]);
  }

  /// Under `strict: true`, an unconfigured `RefuseToPinLocal` resolves to
  /// `Error`. Strict only changes the default for unconfigured Suspect — it
  /// does not affect Fixable defaults.
  #[tokio::test]
  async fn refuse_to_pin_local_under_strict_is_error() {
    let ctx = TestBuilder::new()
      .with_strict(true)
      .with_package(json!({"name": "package-a", "version": "1.0.0"}))
      .with_version_group(json!({
        "dependencies": ["package-a"],
        "pinVersion": "1.2.0"
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![ExpectedInstance {
      state: InstanceState::suspect(RefuseToPinLocal),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.2.0"),
      overridden: None,
      severity: Some(Severity::Error),
    }]);
  }

  /// `strict: true` + explicit `severity: { RefuseToPinLocal: "warn" }`:
  /// explicit user value wins over the strict default of Error.
  #[tokio::test]
  async fn refuse_to_pin_local_explicit_severity_wins_over_strict_default() {
    let ctx = TestBuilder::new()
      .with_strict(true)
      .with_package(json!({"name": "package-a", "version": "1.0.0"}))
      .with_version_group(json!({
        "dependencies": ["package-a"],
        "pinVersion": "1.2.0",
        "severity": {"RefuseToPinLocal": "warn"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![ExpectedInstance {
      state: InstanceState::suspect(RefuseToPinLocal),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.2.0"),
      overridden: None,
      severity: Some(Severity::Warn),
    }]);
  }

  /// `severity: { RefuseToPinLocal: "fix" }` opts into v13-era greedy pin
  /// behaviour. The Suspect's resolved severity becomes `Fix`; downstream
  /// (in `fix.rs`) this routes a write through `copy_expected_specifier_json`
  /// to rewrite the local pkg's `/version`. Issue #325 + plan §3.7.
  #[tokio::test]
  async fn refuse_to_pin_local_fix_routes_through_fix_action() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({"name": "package-a", "version": "1.0.0"}),
        json!({"name": "package-b", "dependencies": {"package-a": "1.1.0"}}),
      ])
      .with_version_group(json!({
        "dependencies": ["package-a"],
        "pinVersion": "1.2.0",
        "severity": {"RefuseToPinLocal": "fix"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(RefuseToPinLocal),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.2.0"),
        overridden: None,
        severity: Some(Severity::Fix),
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToPin),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "1.1.0",
        expected: Some("1.2.0"),
        overridden: None,
        severity: Some(Severity::Fix),
      },
    ]);
  }

  /// `severity: { PinOverridesSemverRange: "warn" }` — state stays
  /// `PinOverridesSemverRange` (pin wins over semver group at the data
  /// layer); severity downgrades from default `Fix` to `Warn`.
  #[tokio::test]
  async fn pin_overrides_semver_range_warn() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0",
        "devDependencies": {"foo": "^1.0.0"}
      }))
      .with_semver_group(json!({"range": "^", "dependencies": ["foo"]}))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "pinVersion": "1.0.0",
        "severity": {"PinOverridesSemverRange": "warn"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::fixable(PinOverridesSemverRange),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: "^1.0.0",
        expected: Some("1.0.0"),
        overridden: Some("^1.0.0"),
        severity: Some(Severity::Warn),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
    ]);
  }

  /// `severity: { PinOverridesSemverRange: "error" }` resolves to `Error`.
  #[tokio::test]
  async fn pin_overrides_semver_range_error() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0",
        "devDependencies": {"foo": "^1.0.0"}
      }))
      .with_semver_group(json!({"range": "^", "dependencies": ["foo"]}))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "pinVersion": "1.0.0",
        "severity": {"PinOverridesSemverRange": "error"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::fixable(PinOverridesSemverRange),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: "^1.0.0",
        expected: Some("1.0.0"),
        overridden: Some("^1.0.0"),
        severity: Some(Severity::Error),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
    ]);
  }

  /// `severity: { PinOverridesSemverRangeMismatch: "warn" }` resolves to
  /// `Warn`.
  #[tokio::test]
  async fn pin_overrides_semver_range_mismatch_warn() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0",
        "devDependencies": {"foo": ">=1.0.0"}
      }))
      .with_semver_group(json!({"range": "^", "dependencies": ["foo"]}))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "pinVersion": "1.0.0",
        "severity": {"PinOverridesSemverRangeMismatch": "warn"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::fixable(PinOverridesSemverRangeMismatch),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: ">=1.0.0",
        expected: Some("1.0.0"),
        overridden: Some("^1.0.0"),
        severity: Some(Severity::Warn),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
    ]);
  }

  /// `severity: { PinOverridesSemverRangeMismatch: "error" }` → `Error`.
  #[tokio::test]
  async fn pin_overrides_semver_range_mismatch_error() {
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "version": "1.0.0",
        "devDependencies": {"foo": ">=1.0.0"}
      }))
      .with_semver_group(json!({"range": "^", "dependencies": ["foo"]}))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "pinVersion": "1.0.0",
        "severity": {"PinOverridesSemverRangeMismatch": "error"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::fixable(PinOverridesSemverRangeMismatch),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: ">=1.0.0",
        expected: Some("1.0.0"),
        overridden: Some("^1.0.0"),
        severity: Some(Severity::Error),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
    ]);
  }

  /// A Pinned group rejects severity keys outside its permitted set.
  /// `SemverRangeMismatch` is a key for PreferredSemver / SameRange /
  /// SameMinor / SnappedTo, never Pinned — using it here surfaces
  /// `InvalidSeverityKey` with the offending key, the resolved group type,
  /// and the permitted set.
  #[tokio::test]
  #[should_panic(expected = "InvalidSeverityKey")]
  async fn rejects_invalid_severity_key() {
    let _ctx = TestBuilder::new()
      .with_package(json!({"name": "package-a", "version": "1.0.0"}))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "pinVersion": "1.0.0",
        "severity": {"SemverRangeMismatch": "warn"}
      }))
      .run()
      .await;
  }

  /// `DiffersToNpmRegistry` is owned by `updateGroups`, not severity.
  /// Writing it as a severity key on any group → `InvalidSeverityKey`.
  #[tokio::test]
  #[should_panic(expected = "InvalidSeverityKey")]
  async fn rejects_differs_to_npm_registry_severity_key() {
    let _ctx = TestBuilder::new()
      .with_package(json!({"name": "package-a", "version": "1.0.0"}))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "pinVersion": "1.0.0",
        "severity": {"DiffersToNpmRegistry": "warn"}
      }))
      .run()
      .await;
  }

  /// JSON output: every instance carries a `severity` field. Two
  /// resolved-severity cases coexist in one scenario:
  /// - Valid local pkg → `"none"`
  /// - Pinned-fixable consumer → `"fix"` (default for Fixable)
  ///
  /// Walks `ctx.version_groups` like `json::run` does and asserts the
  /// `severity` field on each emitted `instance_to_json` value. Plan §3.9.
  #[tokio::test]
  async fn json_output_carries_severity_field() {
    use crate::version_group::VersionGroupBehavior;
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({"name": "package-a", "version": "1.0.0"}),
        json!({"name": "package-b", "dependencies": {"foo": "1.0.0"}}),
      ])
      .with_version_groups(vec![json!({
        "dependencies": ["foo"],
        "pinVersion": "1.2.0"
      })])
      .run()
      .await;
    let mut by_id: std::collections::HashMap<String, serde_json::Value> = Default::default();
    for group in &ctx.version_groups {
      let label = group.variant_label();
      for dep in group.dependencies().values() {
        for (_, instance) in dep.get_instances(&ctx.instances) {
          let json = instance_to_json(&ctx, instance, label);
          let severity = json
            .get("severity")
            .unwrap_or_else(|| panic!("instance JSON missing 'severity' field: {json}"));
          assert!(
            severity.is_string(),
            "severity must be a string, got: {severity:?} for {}",
            instance.id
          );
          by_id.insert(instance.id.clone(), severity.clone());
        }
      }
    }
    assert_eq!(
      by_id.get("package-a in /version of package-a").and_then(|v| v.as_str()),
      Some("none"),
      "Valid local has severity 'none'",
    );
    assert_eq!(
      by_id.get("foo in /dependencies of package-b").and_then(|v| v.as_str()),
      Some("fix"),
      "Default-fix Fixable has severity 'fix'",
    );
  }
}
