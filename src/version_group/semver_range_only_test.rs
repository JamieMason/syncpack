use {
  crate::{
    instance::{FixableInstance::*, InstanceState, ValidInstance::*},
    test::{
      builder::TestBuilder,
      expect::{ExpectedInstance, expect},
    },
  },
  serde_json::json,
};

/// No semver group → no range rule applies → MatchesSemverGroup.
#[tokio::test]
async fn no_semver_group_marks_instance_as_matches_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({"name": "package-a", "dependencies": {"foo": "1.2.3"}}),
      json!({"name": "package-b", "dependencies": {"foo": "4.5.6"}}),
    ])
    .with_version_groups(vec![
      json!({"dependencyTypes": ["local"], "isIgnored": true}),
      json!({"dependencies": ["foo"], "policy": "semverRangeOnly"}),
    ])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
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
      state: InstanceState::valid(MatchesSemverGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: "1.2.3",
      expected: Some("1.2.3"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(MatchesSemverGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "4.5.6",
      expected: Some("4.5.6"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Specifier range matches semver group's range → MatchesSemverGroup.
#[tokio::test]
async fn matching_semver_group_range_is_valid() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "package-a",
      "dependencies": {"foo": "^1.2.3"}
    })])
    .with_semver_group(json!({"packages": ["package-a"], "range": "^"}))
    .with_version_groups(vec![
      json!({"dependencyTypes": ["local"], "isIgnored": true}),
      json!({"dependencies": ["foo"], "policy": "semverRangeOnly"}),
    ])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(MatchesSemverGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: "^1.2.3",
      expected: Some("^1.2.3"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Specifier range mismatches semver group → SemverRangeMismatch, fix
/// rewrites the range prefix.
#[tokio::test]
async fn mismatching_semver_group_range_is_fixable() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "package-a",
      "dependencies": {"foo": "^1.2.3"}
    })])
    .with_semver_group(json!({"packages": ["package-a"], "range": "~"}))
    .with_version_groups(vec![
      json!({"dependencyTypes": ["local"], "isIgnored": true}),
      json!({"dependencies": ["foo"], "policy": "semverRangeOnly"}),
    ])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SemverRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: "^1.2.3",
      expected: Some("~1.2.3"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Cross-instance versions are NOT reconciled — two packages with different
/// versions both pass when their ranges match the semver group.
#[tokio::test]
async fn instances_with_different_versions_are_independent() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({"name": "package-a", "dependencies": {"foo": "^1.0.0"}}),
      json!({"name": "package-b", "dependencies": {"foo": "^2.5.0"}}),
    ])
    .with_semver_group(json!({"range": "^"}))
    .with_version_groups(vec![
      json!({"dependencyTypes": ["local"], "isIgnored": true}),
      json!({"dependencies": ["foo"], "policy": "semverRangeOnly"}),
    ])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
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
      state: InstanceState::valid(MatchesSemverGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: "^1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(MatchesSemverGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "^2.5.0",
      expected: Some("^2.5.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Local instance with a valid exact version → IsLocalAndValid.
#[tokio::test]
async fn local_instance_with_valid_version_is_local_and_valid() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "package-a",
      "version": "1.0.0"
    })])
    .with_version_group(json!({
      "dependencies": ["package-a"],
      "policy": "semverRangeOnly"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![ExpectedInstance {
    state: InstanceState::valid(IsLocalAndValid),
    dependency_name: "package-a",
    id: "package-a in /version of package-a",
    actual: "1.0.0",
    expected: Some("1.0.0"),
    overridden: None,
    severity: None,
  }]);
}

/// Local instance with a non-exact version specifier is IsLocalAndValid
/// because the built-in exact-local semver group has `range: None` (it
/// doesn't enforce a range), and SemverRangeOnly does not special-case
/// invalid locals the way PreferredSemver does.
#[tokio::test]
async fn local_instance_with_range_prefix_is_local_and_valid() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "package-a",
      "version": "^1.0.0"
    })])
    .with_version_group(json!({
      "dependencies": ["package-a"],
      "policy": "semverRangeOnly"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![ExpectedInstance {
    state: InstanceState::valid(IsLocalAndValid),
    dependency_name: "package-a",
    id: "package-a in /version of package-a",
    actual: "^1.0.0",
    expected: Some("^1.0.0"),
    overridden: None,
    severity: None,
  }]);
}

/// workspace:* has no semver version → range fix is skipped, instance
/// is MatchesSemverGroup. Mirrors HighestSemver, which filters non-semver
/// specifiers out of its range-fix candidates.
#[tokio::test]
async fn workspace_protocol_specifier_is_valid() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({"name": "package-a", "version": "1.0.0"}),
      json!({
        "name": "package-b",
        "dependencies": {"package-a": "workspace:*"}
      }),
    ])
    .with_semver_group(json!({"range": "^"}))
    .with_version_groups(vec![
      json!({"dependencyTypes": ["local"], "isIgnored": true}),
      json!({"dependencies": ["package-a"], "policy": "semverRangeOnly"}),
    ])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
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
      dependency_name: "package-b",
      id: "package-b in /version of package-b",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(MatchesSemverGroup),
      dependency_name: "package-a",
      id: "package-a in /dependencies of package-b",
      actual: "workspace:*",
      expected: Some("workspace:*"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// A tag like `next` or `*` has no semver range to enforce → MatchesSemverGroup.
#[tokio::test]
async fn tag_and_latest_specifiers_are_valid() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({"name": "package-a", "dependencies": {"foo": "next"}}),
      json!({"name": "package-b", "dependencies": {"foo": "*"}}),
    ])
    .with_semver_group(json!({"range": "^"}))
    .with_version_groups(vec![
      json!({"dependencyTypes": ["local"], "isIgnored": true}),
      json!({"dependencies": ["foo"], "policy": "semverRangeOnly"}),
    ])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
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
      state: InstanceState::valid(MatchesSemverGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: "next",
      expected: Some("next"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(MatchesSemverGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "*",
      expected: Some("*"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Registry update available → DiffersToNpmRegistry, fix applies the
/// semver group's preferred range to the higher version.
#[tokio::test]
async fn registry_update_is_fixable_as_differs_to_npm_registry() {
  let ctx = TestBuilder::new()
    .with_package(json!({
      "name": "package-a",
      "version": "0.0.0",
      "dependencies": {"foo": "^1.2.3"}
    }))
    .with_semver_group(json!({"packages": ["package-a"], "range": "^"}))
    .with_version_groups(vec![
      json!({"dependencyTypes": ["local"], "isIgnored": true}),
      json!({"dependencies": ["foo"], "policy": "semverRangeOnly"}),
    ])
    .with_registry_updates(json!({"foo": ["1.2.3", "1.3.0", "2.0.0"]}))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToNpmRegistry),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: "^1.2.3",
      expected: Some("^2.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Registry update available AND specifier range mismatches semver group →
/// DiffersToNpmRegistry wins (registry fix carries the preferred range).
#[tokio::test]
async fn registry_update_overrides_range_mismatch() {
  let ctx = TestBuilder::new()
    .with_package(json!({
      "name": "package-a",
      "version": "0.0.0",
      "dependencies": {"foo": "~1.2.3"}
    }))
    .with_semver_group(json!({"packages": ["package-a"], "range": "^"}))
    .with_version_groups(vec![
      json!({"dependencyTypes": ["local"], "isIgnored": true}),
      json!({"dependencies": ["foo"], "policy": "semverRangeOnly"}),
    ])
    .with_registry_updates(json!({"foo": ["1.2.3", "1.3.0", "2.0.0"]}))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToNpmRegistry),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: "~1.2.3",
      expected: Some("^2.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// UpdatePolicy::Skip short-circuits to IsIgnored — matches HighestSemver.
#[tokio::test]
async fn update_policy_skip_short_circuits_to_is_ignored() {
  let ctx = TestBuilder::new()
    .with_package(json!({
      "name": "package-a",
      "version": "0.0.0",
      "dependencies": {"foo": "~1.2.3"}
    }))
    .with_semver_group(json!({"packages": ["package-a"], "range": "^"}))
    .with_update_group(json!({"dependencies": ["foo"], "isIgnored": true}))
    .with_version_groups(vec![
      json!({"dependencyTypes": ["local"], "isIgnored": true}),
      json!({"dependencies": ["foo"], "policy": "semverRangeOnly"}),
    ])
    .with_registry_updates(json!({"foo": ["1.2.3", "2.0.0"]}))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: "~1.2.3",
      expected: Some("~1.2.3"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Severity tests — opt out of auto-fix per status (issue #216).
/// SemverRangeOnly permits only `SemverRangeMismatch`.
mod severity {
  use {super::*, crate::instance::Severity};

  #[tokio::test]
  async fn semver_range_mismatch_warn() {
    let ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "package-a",
        "dependencies": {"foo": "^1.2.3"}
      })])
      .with_semver_group(json!({"packages": ["package-a"], "range": "~"}))
      .with_version_groups(vec![
        json!({"dependencyTypes": ["local"], "isIgnored": true}),
        json!({
          "dependencies": ["foo"],
          "policy": "semverRangeOnly",
          "severity": {"SemverRangeMismatch": "warn"}
        }),
      ])
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsIgnored),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(SemverRangeMismatch),
        dependency_name: "foo",
        id: "foo in /dependencies of package-a",
        actual: "^1.2.3",
        expected: Some("~1.2.3"),
        overridden: None,
        severity: Some(Severity::Warn),
      },
    ]);
  }

  #[tokio::test]
  async fn semver_range_mismatch_error() {
    let ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "package-a",
        "dependencies": {"foo": "^1.2.3"}
      })])
      .with_semver_group(json!({"packages": ["package-a"], "range": "~"}))
      .with_version_groups(vec![
        json!({"dependencyTypes": ["local"], "isIgnored": true}),
        json!({
          "dependencies": ["foo"],
          "policy": "semverRangeOnly",
          "severity": {"SemverRangeMismatch": "error"}
        }),
      ])
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsIgnored),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(SemverRangeMismatch),
        dependency_name: "foo",
        id: "foo in /dependencies of package-a",
        actual: "^1.2.3",
        expected: Some("~1.2.3"),
        overridden: None,
        severity: Some(Severity::Error),
      },
    ]);
  }

  /// SemverRangeOnly only permits `SemverRangeMismatch`. `DiffersToLocal`
  /// is a PreferredSemver key → `InvalidSeverityKey`.
  #[tokio::test]
  #[should_panic(expected = "InvalidSeverityKey")]
  async fn rejects_invalid_severity_key() {
    let _ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "package-a",
        "dependencies": {"foo": "^1.2.3"}
      })])
      .with_version_group(json!({
        "dependencies": ["foo"],
        "policy": "semverRangeOnly",
        "severity": {"DiffersToLocal": "warn"}
      }))
      .run()
      .await;
  }
}
