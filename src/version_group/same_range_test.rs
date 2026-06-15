use {
  crate::{
    instance::{FixableInstance::*, InstanceState, UnfixableInstance::*, ValidInstance::*},
    test::{
      builder::TestBuilder,
      expect::{ExpectedInstance, expect},
    },
  },
  serde_json::json,
};

#[tokio::test]
async fn instance_in_a_same_range_group_satisfies_every_other_and_there_are_no_semver_groups() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": ">=1.0.0"
        }
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "foo": "<=2.0.0"
        }
      }),
    ])
    .with_version_groups(vec![
      json!({
        "dependencyTypes": ["local"],
        "isIgnored": true
      }),
      json!({
        "dependencies": ["foo"],
        "policy": "sameRange"
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
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: ">=1.0.0",
      expected: Some(">=1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "<=2.0.0",
      expected: Some("<=2.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn instance_in_a_same_range_group_satisfies_every_other_and_matches_its_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": ">=1.0.0"
        }
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "foo": "^1.2.3"
        }
      }),
    ])
    .with_semver_group(json!({
      "packages": ["package-b"],
      "range": "^"
    }))
    .with_version_groups(vec![
      json!({
        "dependencyTypes": ["local"],
        "isIgnored": true
      }),
      json!({
        "dependencies": ["foo"],
        "policy": "sameRange"
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
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: ">=1.0.0",
      expected: Some(">=1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "^1.2.3",
      expected: Some("^1.2.3"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn instance_in_a_same_range_group_satisfies_every_other_but_mismatches_its_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": ">=1.0.0"
        }
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "foo": "^1.2.3"
        }
      }),
    ])
    .with_semver_group(json!({
      "packages": ["package-b"],
      "range": "~"
    }))
    .with_version_groups(vec![
      json!({
        "dependencyTypes": ["local"],
        "isIgnored": true
      }),
      json!({
        "dependencies": ["foo"],
        "policy": "sameRange"
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
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: ">=1.0.0",
      expected: Some(">=1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SemverRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "^1.2.3",
      expected: Some("~1.2.3"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn instance_in_a_same_range_group_does_not_satisfy_another() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": ">=1.0.0"
        }
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "foo": "<1.0.0"
        }
      }),
    ])
    .with_version_groups(vec![
      json!({
        "dependencyTypes": ["local"],
        "isIgnored": true
      }),
      json!({
        "dependencies": ["foo"],
        "policy": "sameRange"
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
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: ">=1.0.0",
      expected: Some(">=1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "<1.0.0",
      expected: Some("<1.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn caret_and_tilde_ranges_overlap_when_tilde_is_within_caret() {
  // ^1.0.0 covers 1.0.0-1.x.x, ~1.4.2 covers 1.4.2-1.4.x -- they overlap
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": "^1.0.0"
        }
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "foo": "~1.4.2"
        }
      }),
    ])
    .with_version_groups(vec![
      json!({
        "dependencyTypes": ["local"],
        "isIgnored": true
      }),
      json!({
        "dependencies": ["foo"],
        "policy": "sameRange"
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
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: "^1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "~1.4.2",
      expected: Some("~1.4.2"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn tilde_ranges_do_not_overlap_when_minor_versions_differ() {
  // ~1.0.0 covers 1.0.x, ~1.4.2 covers 1.4.x -- no overlap
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": "~1.0.0"
        }
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "foo": "~1.4.2"
        }
      }),
    ])
    .with_version_groups(vec![
      json!({
        "dependencyTypes": ["local"],
        "isIgnored": true
      }),
      json!({
        "dependencies": ["foo"],
        "policy": "sameRange"
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
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: "~1.0.0",
      expected: Some("~1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "~1.4.2",
      expected: Some("~1.4.2"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn exact_pinned_versions_that_differ_do_not_overlap() {
  // 1.0.0 and 1.0.1 are each a single point -- they don't intersect
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": "1.0.0"
        }
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "foo": "1.0.1"
        }
      }),
    ])
    .with_version_groups(vec![
      json!({
        "dependencyTypes": ["local"],
        "isIgnored": true
      }),
      json!({
        "dependencies": ["foo"],
        "policy": "sameRange"
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
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "1.0.1",
      expected: Some("1.0.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn three_ranges_all_overlap_pairwise() {
  // >=1.0.0, ^1.2.3, <=2.0.0 -- all three pairwise intersect
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": ">=1.0.0"
        }
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "foo": "^1.2.3"
        }
      }),
      json!({
        "name": "package-c",
        "dependencies": {
          "foo": "<=2.0.0"
        }
      }),
    ])
    .with_version_groups(vec![
      json!({
        "dependencyTypes": ["local"],
        "isIgnored": true
      }),
      json!({
        "dependencies": ["foo"],
        "policy": "sameRange"
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
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-c",
      id: "package-c in /version of package-c",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: ">=1.0.0",
      expected: Some(">=1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "^1.2.3",
      expected: Some("^1.2.3"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-c",
      actual: "<=2.0.0",
      expected: Some("<=2.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn three_ranges_where_one_does_not_overlap_with_others() {
  // ^1.0.0 and ~1.2.0 overlap, but <1.0.0 overlaps with neither
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": "^1.0.0"
        }
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "foo": "~1.2.0"
        }
      }),
      json!({
        "name": "package-c",
        "dependencies": {
          "foo": "<1.0.0"
        }
      }),
    ])
    .with_version_groups(vec![
      json!({
        "dependencyTypes": ["local"],
        "isIgnored": true
      }),
      json!({
        "dependencies": ["foo"],
        "policy": "sameRange"
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
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-c",
      id: "package-c in /version of package-c",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: "^1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "~1.2.0",
      expected: Some("~1.2.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-c",
      actual: "<1.0.0",
      expected: Some("<1.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Regression test for issue #298: when a sameRange version group matches
/// multiple *different* dependency names (e.g. `@nx/*` and `nx`) and each
/// appears only once (single package.json), each dep's DependencyCore contains
/// exactly one instance. Previously `already_satisfies_all` received a
/// one-element list that trivially passes — every instance satisfied only
/// itself, so the group reported "no issues" even when the exact pinned
/// versions diverged.
///
/// The fix: check each instance against ALL instances across ALL deps in the
/// group, not just against its own dep-name's sibling instances.
#[tokio::test]
async fn cross_dep_exact_versions_that_differ_are_flagged_issue_298() {
  // Mirrors the minimal repro from issue #298:
  //   npm pkg set dependencies.nx=21.3.0 dependencies.@nx/js=21.3.1 dependencies.@nx/eslint=21.3.2
  //   syncpack lint  →  was: "✓ No issues found"  (wrong)
  //                 →  want: SameRangeMismatch on every instance  (correct)
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "repro",
      "dependencies": {
        "nx":         "21.3.0",
        "@nx/js":     "21.3.1",
        "@nx/eslint": "21.3.2"
      }
    })])
    .with_version_groups(vec![
      json!({
        "dependencyTypes": ["local"],
        "isIgnored": true
      }),
      json!({
        "dependencies": ["nx", "@nx/*"],
        "policy": "sameRange"
      }),
    ])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "repro",
      id: "repro in /version of repro",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "@nx/eslint",
      id: "@nx/eslint in /dependencies of repro",
      actual: "21.3.2",
      expected: Some("21.3.2"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "@nx/js",
      id: "@nx/js in /dependencies of repro",
      actual: "21.3.1",
      expected: Some("21.3.1"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "nx",
      id: "nx in /dependencies of repro",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// When all cross-dep versions in the same sameRange group are identical
/// pinned versions they should be valid (regression guard for the fix).
#[tokio::test]
async fn cross_dep_exact_versions_that_match_are_valid_issue_298() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "repro",
      "dependencies": {
        "nx":         "21.3.0",
        "@nx/js":     "21.3.0",
        "@nx/eslint": "21.3.0"
      }
    })])
    .with_version_groups(vec![
      json!({
        "dependencyTypes": ["local"],
        "isIgnored": true
      }),
      json!({
        "dependencies": ["nx", "@nx/*"],
        "policy": "sameRange"
      }),
    ])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "repro",
      id: "repro in /version of repro",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "@nx/eslint",
      id: "@nx/eslint in /dependencies of repro",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "@nx/js",
      id: "@nx/js in /dependencies of repro",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "nx",
      id: "nx in /dependencies of repro",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Severity tests — opt out of auto-fix per status (issue #216).
/// SameRange only permits `SemverRangeMismatch` (the only Fixable variant it
/// produces). `SameRangeMismatch` itself is Unfixable and not user-tunable.
mod severity {
  use {super::*, crate::instance::Severity};

  /// Scenario: package-a `foo: ">=1.0.0"` is `SatisfiesSameRangeGroup`;
  /// package-b `foo: "^1.2.3"` matches the same range but mismatches the
  /// semver group's tilde range → `SemverRangeMismatch` Fixable. With
  /// `severity: { SemverRangeMismatch: "warn" }`, severity downgrades to
  /// `Warn`.
  #[tokio::test]
  async fn semver_range_mismatch_warn() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({"name": "package-a", "dependencies": {"foo": ">=1.0.0"}}),
        json!({"name": "package-b", "dependencies": {"foo": "^1.2.3"}}),
      ])
      .with_semver_group(json!({"packages": ["package-b"], "range": "~"}))
      .with_version_groups(vec![
        json!({"dependencyTypes": ["local"], "isIgnored": true}),
        json!({
          "dependencies": ["foo"],
          "policy": "sameRange",
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
        state: InstanceState::valid(IsIgnored),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesSameRangeGroup),
        dependency_name: "foo",
        id: "foo in /dependencies of package-a",
        actual: ">=1.0.0",
        expected: Some(">=1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(SemverRangeMismatch),
        dependency_name: "foo",
        id: "foo in /dependencies of package-b",
        actual: "^1.2.3",
        expected: Some("~1.2.3"),
        overridden: None,
        severity: Some(Severity::Warn),
      },
    ]);
  }

  /// `severity: { SemverRangeMismatch: "error" }` → `Error`.
  #[tokio::test]
  async fn semver_range_mismatch_error() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({"name": "package-a", "dependencies": {"foo": ">=1.0.0"}}),
        json!({"name": "package-b", "dependencies": {"foo": "^1.2.3"}}),
      ])
      .with_semver_group(json!({"packages": ["package-b"], "range": "~"}))
      .with_version_groups(vec![
        json!({"dependencyTypes": ["local"], "isIgnored": true}),
        json!({
          "dependencies": ["foo"],
          "policy": "sameRange",
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
        state: InstanceState::valid(IsIgnored),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesSameRangeGroup),
        dependency_name: "foo",
        id: "foo in /dependencies of package-a",
        actual: ">=1.0.0",
        expected: Some(">=1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(SemverRangeMismatch),
        dependency_name: "foo",
        id: "foo in /dependencies of package-b",
        actual: "^1.2.3",
        expected: Some("~1.2.3"),
        overridden: None,
        severity: Some(Severity::Error),
      },
    ]);
  }

  /// SameRange only permits `SemverRangeMismatch`. `DiffersToLocal` is a
  /// PreferredSemver key; using it on a SameRange group → `InvalidSeverityKey`.
  #[tokio::test]
  #[should_panic(expected = "InvalidSeverityKey")]
  async fn rejects_invalid_severity_key() {
    let _ctx = TestBuilder::new()
      .with_packages(vec![
        json!({"name": "package-a", "dependencies": {"foo": ">=1.0.0"}}),
        json!({"name": "package-b", "dependencies": {"foo": "~1.2.3"}}),
      ])
      .with_version_group(json!({
        "dependencies": ["foo"],
        "policy": "sameRange",
        "severity": {"DiffersToLocal": "warn"}
      }))
      .run()
      .await;
  }
}
