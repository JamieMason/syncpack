use {
  crate::{
    instance::{InstanceState, ValidInstance::*},
    test::{
      builder::TestBuilder,
      expect::{ExpectedInstance, expect},
    },
  },
  serde_json::json,
};

#[tokio::test]
async fn all_instances_are_ignored() {
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
    .with_version_group(json!({"isIgnored": true}))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-b",
      id: "package-b in /version of package-b",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-a",
      id: "package-a in /dependencies of package-b",
      actual: "1.1.0",
      expected: Some("1.1.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Severity tests — `severity` on an Ignored group is silently discarded
/// (the group produces no statuses to tune). No error, no effect.
mod severity {
  use {super::*, crate::instance::Severity};

  /// Any `severity` map on an Ignored group is accepted (not rejected as an
  /// unknown field) and has no observable effect — instances stay
  /// `IsIgnored` (Valid) with resolved severity `None`, regardless of what
  /// keys the user wrote.
  #[tokio::test]
  async fn severity_on_ignored_group_is_silently_discarded() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({"name": "package-a", "version": "1.0.0"}),
        json!({"name": "package-b", "dependencies": {"package-a": "1.1.0"}}),
      ])
      .with_version_group(json!({
        "isIgnored": true,
        "severity": {"IsBanned": "error"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsIgnored),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsIgnored),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsIgnored),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "1.1.0",
        expected: Some("1.1.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
    ]);
  }
}
