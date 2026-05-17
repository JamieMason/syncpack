use {
  crate::{
    instance::{FixableInstance::*, InstanceState, SemverGroupAndVersionConflict::*, SuspectInstance::*, ValidInstance::*},
    test::{
      builder::TestBuilder,
      expect::{ExpectedInstance, expect},
    },
  },
  serde_json::json,
};

#[tokio::test]
async fn instance_identical_to_snapped_to_and_has_no_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "leader",
        "dependencies": {
          "foo": "1.0.0"
        }
      }),
      json!({
        "name": "follower",
        "dependencies": {
          "foo": "1.0.0"
        }
      }),
    ])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn instance_has_different_version_to_snapped_to_and_has_no_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "leader",
        "dependencies": {
          "foo": "1.0.0"
        }
      }),
      json!({
        "name": "follower",
        "dependencies": {
          "foo": "1.1.0"
        }
      }),
    ])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "1.1.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn instance_has_same_version_number_as_snapped_to_but_a_different_range_and_has_no_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "leader",
        "dependencies": {
          "foo": "^1.0.0"
        }
      }),
      json!({
        "name": "follower",
        "devDependencies": {
          "foo": "~1.0.0"
        }
      }),
    ])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "^1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToSnapTarget),
      dependency_name: "foo",
      id: "foo in /devDependencies of follower",
      actual: "~1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn instance_has_same_version_number_as_snapped_to_but_matches_a_different_but_compatible_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "leader",
        "dependencies": {
          "foo": "^1.0.0"
        }
      }),
      json!({
        "name": "follower",
        "dependencies": {
          "foo": "~1.0.0"
        }
      }),
    ])
    .with_semver_group(json!({"packages": ["follower"], "range": "~"}))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "~1.0.0",
      expected: Some("~1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "^1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn instance_has_same_version_number_as_snapped_to_but_mismatches_a_different_but_compatible_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "leader",
        "dependencies": {
          "foo": ">=1.0.0"
        }
      }),
      json!({
        "name": "follower",
        "dependencies": {
          "foo": "~1.0.0"
        }
      }),
    ])
    .with_semver_group(json!({"packages": ["follower"], "range": "^"}))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: ">=1.0.0",
      expected: Some(">=1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SemverRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "~1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn instance_has_same_version_number_as_snapped_to_but_matches_a_different_but_incompatible_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "leader",
        "dependencies": {
          "foo": "1.0.0"
        }
      }),
      json!({
        "name": "follower",
        "dependencies": {
          "foo": "<1.0.0"
        }
      }),
    ])
    .with_semver_group(json!({"packages": ["follower"], "range": "<"}))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::conflict(MatchConflictsWithSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "<1.0.0",
      expected: Some("<1.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn instance_has_same_version_number_as_snapped_to_but_mismatches_a_different_but_incompatible_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "leader",
        "dependencies": {
          "foo": "~1.0.0"
        }
      }),
      json!({
        "name": "follower",
        "dependencies": {
          "foo": "1.0.0"
        }
      }),
    ])
    .with_semver_group(json!({"packages": ["follower"], "range": "<"}))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "~1.0.0",
      expected: Some("~1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::conflict(MismatchConflictsWithSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn instance_cannot_find_a_snapped_to_version() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "leader",
        "version": "1.0.0"
      }),
      json!({
        "name": "follower",
        "version": "0.1.0",
        "dependencies": {
          "foo": "1.0.0"
        }
      }),
    ])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "0.1.0",
      expected: Some("0.1.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(DependsOnMissingSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn instance_is_in_a_snapped_to_group_and_is_itself_a_snapped_to_target() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "leader",
        "dependencies": {
          "foo": "1.0.0"
        }
      }),
      json!({
        "name": "follower",
        "dependencies": {
          "foo": "1.0.0"
        }
      }),
    ])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "snapTo": ["leader"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn refuses_to_snap_local_version_to_another_target() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "version": "1.1.0"
      }),
      json!({
        "name": "package-b",
        "version": "0.1.0",
        "dependencies": {
          "package-a": "0.0.1"
        }
      }),
    ])
    .with_version_group(json!({
      "snapTo": ["package-b"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(RefuseToSnapLocal),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.1.0",
      expected: Some("0.0.1"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToSnapTarget),
      dependency_name: "package-b",
      id: "package-b in /version of package-b",
      actual: "0.1.0",
      expected: Some("0.1.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToSnapTarget),
      dependency_name: "package-a",
      id: "package-a in /dependencies of package-b",
      actual: "0.0.1",
      expected: Some("0.0.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn workspace_star_identical_to_snapped_to_target() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "leader",
        "dependencies": {
          "localpkg": "workspace:*"
        }
      }),
      json!({
        "name": "follower",
        "peerDependencies": {
          "localpkg": "workspace:*"
        }
      }),
    ])
    .with_version_group(json!({
      "dependencies": ["localpkg"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsNonSemverButIdentical),
      dependency_name: "localpkg",
      id: "localpkg in /dependencies of leader",
      actual: "workspace:*",
      expected: Some("workspace:*"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToSnapTarget),
      dependency_name: "localpkg",
      id: "localpkg in /peerDependencies of follower",
      actual: "workspace:*",
      expected: Some("workspace:*"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn workspace_star_differs_from_workspace_with_embedded_version() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "leader",
        "dependencies": {
          "localpkg": "workspace:^1.0.0"
        }
      }),
      json!({
        "name": "follower",
        "peerDependencies": {
          "localpkg": "workspace:*"
        }
      }),
    ])
    .with_version_group(json!({
      "dependencies": ["localpkg"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "localpkg",
      id: "localpkg in /dependencies of leader",
      actual: "workspace:^1.0.0",
      expected: Some("workspace:^1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToSnapTarget),
      dependency_name: "localpkg",
      id: "localpkg in /peerDependencies of follower",
      actual: "workspace:*",
      expected: Some("workspace:^1.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn workspace_caret_identical_to_snapped_to_target() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "leader",
        "dependencies": {
          "localpkg": "workspace:^"
        }
      }),
      json!({
        "name": "follower",
        "dependencies": {
          "localpkg": "workspace:^"
        }
      }),
    ])
    .with_version_group(json!({
      "dependencies": ["localpkg"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsNonSemverButIdentical),
      dependency_name: "localpkg",
      id: "localpkg in /dependencies of leader",
      actual: "workspace:^",
      expected: Some("workspace:^"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToSnapTarget),
      dependency_name: "localpkg",
      id: "localpkg in /dependencies of follower",
      actual: "workspace:^",
      expected: Some("workspace:^"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn workspace_tilde_identical_to_snapped_to_target() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "leader",
        "dependencies": {
          "localpkg": "workspace:~"
        }
      }),
      json!({
        "name": "follower",
        "dependencies": {
          "localpkg": "workspace:~"
        }
      }),
    ])
    .with_version_group(json!({
      "dependencies": ["localpkg"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsNonSemverButIdentical),
      dependency_name: "localpkg",
      id: "localpkg in /dependencies of leader",
      actual: "workspace:~",
      expected: Some("workspace:~"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToSnapTarget),
      dependency_name: "localpkg",
      id: "localpkg in /dependencies of follower",
      actual: "workspace:~",
      expected: Some("workspace:~"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Same bug as preferred_semver: when version differs from snap target AND a
/// semver group exists, the fix target should apply the semver group's range
/// instead of inheriting the snap target's range.
#[tokio::test]
async fn differs_to_snap_target_should_apply_semver_group_range() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "leader",
        "dependencies": {
          "foo": "^2.0.0"
        }
      }),
      json!({
        "name": "follower",
        "dependencies": {
          "foo": "1.0.0"
        }
      }),
    ])
    .with_semver_group(json!({"packages": ["follower"], "range": ""}))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "^2.0.0",
      expected: Some("^2.0.0"),
      overridden: None,
      severity: None,
    },
    // BUG: currently suggests ^2.0.0 (inherits ^ from snap target)
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "1.0.0",
      expected: Some("2.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Severity tests — opt out of auto-fix per status (issue #216), opt into
/// snapping local (mirrors issue #325 for pin). SnappedTo permits
/// `DiffersToSnapTarget`, `SemverRangeMismatch`, `RefuseToSnapLocal`.
mod severity {
  use {super::*, crate::instance::Severity};

  /// Scenario: leader foo 1.0.0, follower foo 1.1.0, follower snaps to
  /// leader. follower's foo → DiffersToSnapTarget Fixable. severity
  /// downgrades to `Warn`.
  #[tokio::test]
  async fn differs_to_snap_target_warn() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({"name": "leader", "dependencies": {"foo": "1.0.0"}}),
        json!({"name": "follower", "dependencies": {"foo": "1.1.0"}}),
      ])
      .with_version_group(json!({
        "dependencies": ["foo"],
        "packages": ["follower"],
        "snapTo": ["leader"],
        "severity": {"DiffersToSnapTarget": "warn"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "leader",
        id: "leader in /version of leader",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "follower",
        id: "follower in /version of follower",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of leader",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToSnapTarget),
        dependency_name: "foo",
        id: "foo in /dependencies of follower",
        actual: "1.1.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::Warn),
      },
    ]);
  }

  /// `severity: { DiffersToSnapTarget: "error" }` → `Error`.
  #[tokio::test]
  async fn differs_to_snap_target_error() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({"name": "leader", "dependencies": {"foo": "1.0.0"}}),
        json!({"name": "follower", "dependencies": {"foo": "1.1.0"}}),
      ])
      .with_version_group(json!({
        "dependencies": ["foo"],
        "packages": ["follower"],
        "snapTo": ["leader"],
        "severity": {"DiffersToSnapTarget": "error"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "leader",
        id: "leader in /version of leader",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "follower",
        id: "follower in /version of follower",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of leader",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToSnapTarget),
        dependency_name: "foo",
        id: "foo in /dependencies of follower",
        actual: "1.1.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::Error),
      },
    ]);
  }

  /// Scenario: follower's foo `~1.0.0` mismatches its caret semver group but
  /// satisfies the snap target's `>=1.0.0` → `SemverRangeMismatch` (range
  /// fix to `^`). severity downgrades to `Warn`.
  #[tokio::test]
  async fn semver_range_mismatch_warn() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({"name": "leader", "dependencies": {"foo": ">=1.0.0"}}),
        json!({"name": "follower", "dependencies": {"foo": "~1.0.0"}}),
      ])
      .with_semver_group(json!({"packages": ["follower"], "range": "^"}))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "packages": ["follower"],
        "snapTo": ["leader"],
        "severity": {"SemverRangeMismatch": "warn"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "leader",
        id: "leader in /version of leader",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "follower",
        id: "follower in /version of follower",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of leader",
        actual: ">=1.0.0",
        expected: Some(">=1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(SemverRangeMismatch),
        dependency_name: "foo",
        id: "foo in /dependencies of follower",
        actual: "~1.0.0",
        expected: Some("^1.0.0"),
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
        json!({"name": "leader", "dependencies": {"foo": ">=1.0.0"}}),
        json!({"name": "follower", "dependencies": {"foo": "~1.0.0"}}),
      ])
      .with_semver_group(json!({"packages": ["follower"], "range": "^"}))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "packages": ["follower"],
        "snapTo": ["leader"],
        "severity": {"SemverRangeMismatch": "error"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "leader",
        id: "leader in /version of leader",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "follower",
        id: "follower in /version of follower",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of leader",
        actual: ">=1.0.0",
        expected: Some(">=1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(SemverRangeMismatch),
        dependency_name: "foo",
        id: "foo in /dependencies of follower",
        actual: "~1.0.0",
        expected: Some("^1.0.0"),
        overridden: None,
        severity: Some(Severity::Error),
      },
    ]);
  }

  /// Default `RefuseToSnapLocal` under `!strict` resolves to `Warn`. This is
  /// the BREAKING visibility change for Suspects under `!strict`.
  #[tokio::test]
  async fn refuse_to_snap_local_defaults_to_warn_under_non_strict() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({"name": "package-a", "version": "1.1.0"}),
        json!({
          "name": "package-b",
          "version": "0.1.0",
          "dependencies": {"package-a": "0.0.1"}
        }),
      ])
      .with_version_group(json!({"snapTo": ["package-b"]}))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(RefuseToSnapLocal),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.1.0",
        expected: Some("0.0.1"),
        overridden: None,
        severity: Some(Severity::Warn),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsIdenticalToSnapTarget),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "0.1.0",
        expected: Some("0.1.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsIdenticalToSnapTarget),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "0.0.1",
        expected: Some("0.0.1"),
        overridden: None,
        severity: Some(Severity::None),
      },
    ]);
  }

  /// Under `strict: true`, unconfigured `RefuseToSnapLocal` resolves to
  /// `Error`. Mirrors RefuseToPinLocal strict default.
  #[tokio::test]
  async fn refuse_to_snap_local_under_strict_is_error() {
    let ctx = TestBuilder::new()
      .with_strict(true)
      .with_packages(vec![
        json!({"name": "package-a", "version": "1.1.0"}),
        json!({
          "name": "package-b",
          "version": "0.1.0",
          "dependencies": {"package-a": "0.0.1"}
        }),
      ])
      .with_version_group(json!({"snapTo": ["package-b"]}))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(RefuseToSnapLocal),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.1.0",
        expected: Some("0.0.1"),
        overridden: None,
        severity: Some(Severity::Error),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsIdenticalToSnapTarget),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "0.1.0",
        expected: Some("0.1.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsIdenticalToSnapTarget),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "0.0.1",
        expected: Some("0.0.1"),
        overridden: None,
        severity: Some(Severity::None),
      },
    ]);
  }

  /// `strict: true` + explicit `severity: { RefuseToSnapLocal: "warn" }`:
  /// explicit user value wins over the strict default of Error.
  #[tokio::test]
  async fn refuse_to_snap_local_explicit_severity_wins_over_strict_default() {
    let ctx = TestBuilder::new()
      .with_strict(true)
      .with_packages(vec![
        json!({"name": "package-a", "version": "1.1.0"}),
        json!({
          "name": "package-b",
          "version": "0.1.0",
          "dependencies": {"package-a": "0.0.1"}
        }),
      ])
      .with_version_group(json!({
        "snapTo": ["package-b"],
        "severity": {"RefuseToSnapLocal": "warn"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(RefuseToSnapLocal),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.1.0",
        expected: Some("0.0.1"),
        overridden: None,
        severity: Some(Severity::Warn),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsIdenticalToSnapTarget),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "0.1.0",
        expected: Some("0.1.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsIdenticalToSnapTarget),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "0.0.1",
        expected: Some("0.0.1"),
        overridden: None,
        severity: Some(Severity::None),
      },
    ]);
  }

  /// `severity: { RefuseToSnapLocal: "fix" }` opts in to rewriting the
  /// local pkg's /version to the snap target's value. Resolved severity is
  /// `Fix`; plan §3.7 routes the write through `copy_expected_specifier_json`.
  /// Symmetric with RefuseToPinLocal: fix.
  #[tokio::test]
  async fn refuse_to_snap_local_fix_routes_through_fix_action() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({"name": "package-a", "version": "1.1.0"}),
        json!({
          "name": "package-b",
          "version": "0.1.0",
          "dependencies": {"package-a": "0.0.1"}
        }),
      ])
      .with_version_group(json!({
        "snapTo": ["package-b"],
        "severity": {"RefuseToSnapLocal": "fix"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(RefuseToSnapLocal),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.1.0",
        expected: Some("0.0.1"),
        overridden: None,
        severity: Some(Severity::Fix),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsIdenticalToSnapTarget),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "0.1.0",
        expected: Some("0.1.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsIdenticalToSnapTarget),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "0.0.1",
        expected: Some("0.0.1"),
        overridden: None,
        severity: Some(Severity::None),
      },
    ]);
  }

  /// SnappedTo permits `DiffersToSnapTarget`, `SemverRangeMismatch`,
  /// `RefuseToSnapLocal`. `IsBanned` is a Banned-only key →
  /// `InvalidSeverityKey`.
  #[tokio::test]
  #[should_panic(expected = "InvalidSeverityKey")]
  async fn rejects_invalid_severity_key() {
    let _ctx = TestBuilder::new()
      .with_packages(vec![
        json!({"name": "leader", "dependencies": {"foo": "1.0.0"}}),
        json!({"name": "follower", "dependencies": {"foo": "1.1.0"}}),
      ])
      .with_version_group(json!({
        "dependencies": ["foo"],
        "packages": ["follower"],
        "snapTo": ["leader"],
        "severity": {"IsBanned": "warn"}
      }))
      .run()
      .await;
  }
}
