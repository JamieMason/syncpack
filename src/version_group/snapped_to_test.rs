use {
  crate::{
    context::Context,
    instance::{
      FixableInstance::*, InstanceState, SemverGroupAndVersionConflict::*, SuspectInstance::*, UnfixableInstance, ValidInstance::*,
    },
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

#[tokio::test]
async fn snaps_follower_to_targets_corrected_version_not_its_raw_version() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({ "name": "leader",   "dependencies": { "foo": "1.0.0" } }),
      json!({ "name": "other",    "dependencies": { "foo": "2.0.0" } }),
      json!({ "name": "follower", "dependencies": { "foo": "1.0.0" } }),
    ])
    .with_version_group(json!({
      "dependencies": ["foo"], "packages": ["follower"], "snapTo": ["leader"]
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "1.0.0",
      expected: Some("2.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of other",
      actual: "2.0.0",
      expected: Some("2.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "1.0.0",
      expected: Some("2.0.0"),
      overridden: None,
      severity: None,
    },
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
      dependency_name: "other",
      id: "other in /version of other",
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
  ]);
}

/// Chain: `app` snaps to `mid`, `mid` snaps to `leader` (2.0.0). `app` must
/// resolve to the corrected 2.0.0, not `mid`'s raw 1.0.0, despite being
/// declared first.
#[tokio::test]
async fn snaps_follower_through_a_chain_to_the_corrected_version() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({ "name": "leader", "dependencies": { "foo": "2.0.0" } }),
      json!({ "name": "mid",    "dependencies": { "foo": "1.0.0" } }),
      json!({ "name": "app",    "dependencies": { "foo": "1.0.0" } }),
    ])
    .with_version_groups(vec![
      json!({ "dependencies": ["foo"], "packages": ["app"], "snapTo": ["mid"] }),
      json!({ "dependencies": ["foo"], "packages": ["mid"], "snapTo": ["leader"] }),
    ])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "2.0.0",
      expected: Some("2.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of mid",
      actual: "1.0.0",
      expected: Some("2.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of app",
      actual: "1.0.0",
      expected: Some("2.0.0"),
      overridden: None,
      severity: None,
    },
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
      dependency_name: "mid",
      id: "mid in /version of mid",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "app",
      id: "app in /version of app",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Circular snap `a`↔`b` with differing versions: both `CircularSnapToMismatch`.
#[tokio::test]
async fn circular_snap_with_mismatched_versions_is_unfixable() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({ "name": "a", "dependencies": { "foo": "1.0.0" } }),
      json!({ "name": "b", "dependencies": { "foo": "2.0.0" } }),
    ])
    .with_version_groups(vec![
      json!({ "dependencies": ["foo"], "packages": ["a"], "snapTo": ["b"] }),
      json!({ "dependencies": ["foo"], "packages": ["b"], "snapTo": ["a"] }),
    ])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::unfixable(UnfixableInstance::CircularSnapToMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(UnfixableInstance::CircularSnapToMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of b",
      actual: "2.0.0",
      expected: Some("2.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "a",
      id: "a in /version of a",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "b",
      id: "b in /version of b",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Circular snap `a`↔`b` with identical versions stays valid `IsIdenticalToSnapTarget`.
#[tokio::test]
async fn circular_snap_with_identical_versions_is_valid() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({ "name": "a", "dependencies": { "foo": "1.0.0" } }),
      json!({ "name": "b", "dependencies": { "foo": "1.0.0" } }),
    ])
    .with_version_groups(vec![
      json!({ "dependencies": ["foo"], "packages": ["a"], "snapTo": ["b"] }),
      json!({ "dependencies": ["foo"], "packages": ["b"], "snapTo": ["a"] }),
    ])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of b",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "a",
      id: "a in /version of a",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "b",
      id: "b in /version of b",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Circular snap `a`↔`b`, same version number but differing ranges that each
/// satisfy: both stay valid `SatisfiesSnapTarget`.
#[tokio::test]
async fn circular_snap_with_satisfying_versions_is_valid() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({ "name": "a", "dependencies": { "foo": "~1.0.0" } }),
      json!({ "name": "b", "dependencies": { "foo": "^1.0.0" } }),
    ])
    .with_semver_group(json!({ "packages": ["a"], "range": "~" }))
    .with_semver_group(json!({ "packages": ["b"], "range": "^" }))
    .with_version_groups(vec![
      json!({ "dependencies": ["foo"], "packages": ["a"], "snapTo": ["b"] }),
      json!({ "dependencies": ["foo"], "packages": ["b"], "snapTo": ["a"] }),
    ])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of a",
      actual: "~1.0.0",
      expected: Some("~1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of b",
      actual: "^1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "a",
      id: "a in /version of a",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "b",
      id: "b in /version of b",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Three-member cycle `a`→`b`→`c`→`a` with mismatched versions: all three
/// `CircularSnapToMismatch` (detection is not limited to pairs).
#[tokio::test]
async fn circular_snap_of_three_with_mismatch_is_unfixable() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({ "name": "a", "dependencies": { "foo": "1.0.0" } }),
      json!({ "name": "b", "dependencies": { "foo": "2.0.0" } }),
      json!({ "name": "c", "dependencies": { "foo": "3.0.0" } }),
    ])
    .with_version_groups(vec![
      json!({ "dependencies": ["foo"], "packages": ["a"], "snapTo": ["b"] }),
      json!({ "dependencies": ["foo"], "packages": ["b"], "snapTo": ["c"] }),
      json!({ "dependencies": ["foo"], "packages": ["c"], "snapTo": ["a"] }),
    ])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::unfixable(UnfixableInstance::CircularSnapToMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(UnfixableInstance::CircularSnapToMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of b",
      actual: "2.0.0",
      expected: Some("2.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(UnfixableInstance::CircularSnapToMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of c",
      actual: "3.0.0",
      expected: Some("3.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "a",
      id: "a in /version of a",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "b",
      id: "b in /version of b",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "c",
      id: "c in /version of c",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
  ]);
}

/// A banned target's expected specifier is empty (it is to be removed), which
/// must not leak into its followers: they snap to the target's raw version.
#[tokio::test]
async fn snaps_follower_to_raw_version_when_target_is_banned() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({ "name": "leader",   "dependencies": { "foo": "1.0.0" } }),
      json!({ "name": "follower", "dependencies": { "foo": "1.0.0" } }),
    ])
    .with_version_groups(vec![
      json!({ "dependencies": ["foo"], "packages": ["leader"], "isBanned": true }),
      json!({ "dependencies": ["foo"], "packages": ["follower"], "snapTo": ["leader"] }),
    ])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::fixable(IsBanned),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "1.0.0",
      expected: Some(""),
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
  ]);
}

/// `(package, state, expected)` of every `foo` instance in package order, so the cycle
/// tests need not list each package's local `version` instance.
fn foo_states(ctx: &Context) -> Vec<(String, InstanceState, Option<String>)> {
  let mut states: Vec<_> = ctx
    .instances
    .iter()
    .filter(|instance| instance.descriptor.internal_name == "foo")
    .map(|instance| {
      (
        instance.id.rsplit(' ').next().unwrap().to_string(),
        instance.state.borrow().clone(),
        instance
          .expected_specifier
          .borrow()
          .as_ref()
          .map(|expected| expected.get_raw().to_string()),
      )
    })
    .collect();
  states.sort_by(|a, b| a.0.cmp(&b.0));
  states
}

fn state(package: &str, state: InstanceState, expected: &str) -> (String, InstanceState, Option<String>) {
  (package.to_string(), state, Some(expected.to_string()))
}

fn group(package: &str, snap_to: &str) -> serde_json::Value {
  json!({ "dependencies": ["foo"], "packages": [package], "snapTo": [snap_to] })
}

/// Packages which snap into a mismatched cycle have no source of truth to follow, so
/// they are refused instead of rewritten: `a` → `b` ↔ `c` ← `d`.
#[tokio::test]
async fn followers_of_a_mismatched_cycle_are_refused_not_rewritten() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({ "name": "a", "dependencies": { "foo": "3.0.0" } }),
      json!({ "name": "b", "dependencies": { "foo": "1.0.0" } }),
      json!({ "name": "c", "dependencies": { "foo": "2.0.0" } }),
      json!({ "name": "d", "dependencies": { "foo": "4.0.0" } }),
    ])
    .with_version_groups(vec![group("a", "b"), group("b", "c"), group("c", "b"), group("d", "b")])
    .run()
    .await;
  let refused = InstanceState::unfixable(UnfixableInstance::DependsOnCircularSnapToMismatch);
  let cycle = InstanceState::unfixable(UnfixableInstance::CircularSnapToMismatch);
  assert_eq!(
    foo_states(&ctx),
    vec![
      state("a", refused.clone(), "3.0.0"),
      state("b", cycle.clone(), "1.0.0"),
      state("c", cycle, "2.0.0"),
      state("d", refused, "4.0.0"),
    ]
  );
}

/// Which version the followers would have been rewritten to used to depend on which of the
/// two cycle groups came first in the config.
#[tokio::test]
async fn followers_of_a_mismatched_cycle_do_not_depend_on_group_order() {
  let packages = vec![
    json!({ "name": "a", "dependencies": { "foo": "3.0.0" } }),
    json!({ "name": "b", "dependencies": { "foo": "1.0.0" } }),
    json!({ "name": "c", "dependencies": { "foo": "2.0.0" } }),
    json!({ "name": "d", "dependencies": { "foo": "4.0.0" } }),
  ];
  let one_way = TestBuilder::new()
    .with_packages(packages.clone())
    .with_version_groups(vec![group("a", "b"), group("b", "c"), group("c", "b"), group("d", "b")])
    .run()
    .await;
  let other_way = TestBuilder::new()
    .with_packages(packages)
    .with_version_groups(vec![group("a", "b"), group("c", "b"), group("b", "c"), group("d", "b")])
    .run()
    .await;
  assert_eq!(foo_states(&one_way), foo_states(&other_way));
}

/// A follower which already has its direct target's version needs no rewrite, so it stays valid.
#[tokio::test]
async fn follower_of_a_mismatched_cycle_which_already_matches_stays_valid() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({ "name": "a", "dependencies": { "foo": "1.0.0" } }),
      json!({ "name": "b", "dependencies": { "foo": "1.0.0" } }),
      json!({ "name": "c", "dependencies": { "foo": "2.0.0" } }),
      json!({ "name": "d", "dependencies": { "foo": "4.0.0" } }),
    ])
    .with_version_groups(vec![group("a", "b"), group("b", "c"), group("c", "b"), group("d", "b")])
    .run()
    .await;
  let cycle = InstanceState::unfixable(UnfixableInstance::CircularSnapToMismatch);
  assert_eq!(
    foo_states(&ctx),
    vec![
      state("a", InstanceState::valid(IsIdenticalToSnapTarget), "1.0.0"),
      state("b", cycle.clone(), "1.0.0"),
      state("c", cycle, "2.0.0"),
      state(
        "d",
        InstanceState::unfixable(UnfixableInstance::DependsOnCircularSnapToMismatch),
        "4.0.0"
      ),
    ]
  );
}

/// The refusal passes down a chain of followers: `e` → `a` → `b` ↔ `c`.
#[tokio::test]
async fn followers_further_down_a_chain_into_a_mismatched_cycle_are_refused() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({ "name": "e", "dependencies": { "foo": "5.0.0" } }),
      json!({ "name": "a", "dependencies": { "foo": "3.0.0" } }),
      json!({ "name": "b", "dependencies": { "foo": "1.0.0" } }),
      json!({ "name": "c", "dependencies": { "foo": "2.0.0" } }),
    ])
    .with_version_groups(vec![group("e", "a"), group("a", "b"), group("b", "c"), group("c", "b")])
    .run()
    .await;
  let refused = InstanceState::unfixable(UnfixableInstance::DependsOnCircularSnapToMismatch);
  let cycle = InstanceState::unfixable(UnfixableInstance::CircularSnapToMismatch);
  assert_eq!(
    foo_states(&ctx),
    vec![
      state("a", refused.clone(), "3.0.0"),
      state("b", cycle.clone(), "1.0.0"),
      state("c", cycle, "2.0.0"),
      state("e", refused, "5.0.0"),
    ]
  );
}

/// A cycle whose members agree has a source of truth, so its followers are fixed as usual.
#[tokio::test]
async fn followers_of_an_agreeing_cycle_are_still_fixed() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({ "name": "a", "dependencies": { "foo": "3.0.0" } }),
      json!({ "name": "b", "dependencies": { "foo": "1.0.0" } }),
      json!({ "name": "c", "dependencies": { "foo": "1.0.0" } }),
    ])
    .with_version_groups(vec![group("a", "b"), group("b", "c"), group("c", "b")])
    .run()
    .await;
  assert_eq!(
    foo_states(&ctx),
    vec![
      state("a", InstanceState::fixable(DiffersToSnapTarget), "1.0.0"),
      state("b", InstanceState::valid(IsIdenticalToSnapTarget), "1.0.0"),
      state("c", InstanceState::valid(IsIdenticalToSnapTarget), "1.0.0"),
    ]
  );
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
