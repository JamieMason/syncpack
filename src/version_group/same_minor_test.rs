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

// ═══════════════════════════════════════════════════════════════════════
// Non-semver gate
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn non_semver_all_identical() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "pkg-a",
        "version": "1.0.0",
        "dependencies": { "foo": "alpha" }
      }),
      json!({
        "name": "pkg-b",
        "version": "1.0.0",
        "dependencies": { "foo": "alpha" }
      }),
    ])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-b",
      id: "pkg-b in /version of pkg-b",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsNonSemverButIdentical),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-a",
      actual: "alpha",
      expected: Some("alpha"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsNonSemverButIdentical),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-b",
      actual: "alpha",
      expected: Some("alpha"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn non_semver_differing() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "pkg-a",
        "version": "1.0.0",
        "dependencies": { "foo": "alpha" }
      }),
      json!({
        "name": "pkg-b",
        "version": "1.0.0",
        "dependencies": { "foo": "beta" }
      }),
    ])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-b",
      id: "pkg-b in /version of pkg-b",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(NonSemverMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-a",
      actual: "alpha",
      expected: Some("alpha"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(NonSemverMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-b",
      actual: "beta",
      expected: Some("beta"),
      overridden: None,
      severity: None,
    },
  ]);
}

// ═══════════════════════════════════════════════════════════════════════
// Major mismatch gate
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn major_mismatch_marks_all_unfixable() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "1.2.0" },
      "devDependencies": { "foo": "2.1.0" }
    })])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameMinorHasMajorMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "1.2.0",
      expected: Some("1.2.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameMinorHasMajorMismatch),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "2.1.0",
      expected: Some("2.1.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn major_mismatch_with_ranges_marks_all_unfixable() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "21.3.0" },
      "devDependencies": { "foo": "^22.3.1" }
    })])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameMinorHasMajorMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameMinorHasMajorMismatch),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "^22.3.1",
      expected: Some("^22.3.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn major_mismatch_even_with_prefer_version() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "1.2.0" },
      "devDependencies": { "foo": "2.1.0" }
    })])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "highestSemver"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameMinorHasMajorMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "1.2.0",
      expected: Some("1.2.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameMinorHasMajorMismatch),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "2.1.0",
      expected: Some("2.1.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

// ═══════════════════════════════════════════════════════════════════════
// All same MAJOR.MINOR — no semver group
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn satisfies_every_other_and_has_no_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "21.3.0" },
      "devDependencies": { "foo": "21.3.1" }
    })])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "21.3.1",
      expected: Some("21.3.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn has_same_minor_and_compatible_semver_range_and_no_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "21.3.0" },
      "devDependencies": { "foo": "~21.3.1" }
    })])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "~21.3.1",
      expected: Some("~21.3.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn same_minor_no_semver_group_unsafe_caret_range_on_disk() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "21.3.0" },
      "devDependencies": { "foo": "^21.3.1" }
    })])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SameMinorOverridesSemverRange),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "^21.3.1",
      expected: Some("~21.3.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

// ═══════════════════════════════════════════════════════════════════════
// All same MAJOR.MINOR — with semver group — safe preferred range
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn satisfies_every_other_and_matches_compatible_tilde_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "21.3.0" },
      "peerDependencies": { "foo": "~21.3.5" }
    })])
    .with_semver_group(json!({
      "dependencyTypes": ["peer"],
      "range": "~"
    }))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /peerDependencies of my-project",
      actual: "~21.3.5",
      expected: Some("~21.3.5"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn satisfies_every_other_but_mismatches_compatible_tilde_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "21.3.0" },
      "peerDependencies": { "foo": "21.3.5" }
    })])
    .with_semver_group(json!({
      "dependencyTypes": ["peer"],
      "range": "~"
    }))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SemverRangeMismatch),
      dependency_name: "foo",
      id: "foo in /peerDependencies of my-project",
      actual: "21.3.5",
      expected: Some("~21.3.5"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn satisfies_every_other_and_matches_compatible_exact_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "~21.3.0" },
      "peerDependencies": { "foo": "21.3.5" }
    })])
    .with_semver_group(json!({
      "dependencyTypes": ["peer"],
      "range": ""
    }))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "~21.3.0",
      expected: Some("~21.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /peerDependencies of my-project",
      actual: "21.3.5",
      expected: Some("21.3.5"),
      overridden: None,
      severity: None,
    },
  ]);
}

// ═══════════════════════════════════════════════════════════════════════
// All same MAJOR.MINOR — with semver group — unsafe preferred range
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn has_semver_number_which_satisfies_every_other_but_range_matches_incompatible_caret_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "21.3.0" },
      "peerDependencies": { "foo": "^21.3.5" }
    })])
    .with_semver_group(json!({
      "dependencyTypes": ["peer"],
      "range": "^"
    }))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SameMinorOverridesSemverRange),
      dependency_name: "foo",
      id: "foo in /peerDependencies of my-project",
      actual: "^21.3.5",
      expected: Some("~21.3.5"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn has_semver_number_which_satisfies_every_other_but_range_mismatches_incompatible_caret_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "21.3.0" },
      "peerDependencies": { "foo": "21.3.5" }
    })])
    .with_semver_group(json!({
      "dependencyTypes": ["peer"],
      "range": "^"
    }))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SameMinorOverridesSemverRangeMismatch),
      dependency_name: "foo",
      id: "foo in /peerDependencies of my-project",
      actual: "21.3.5",
      expected: Some("~21.3.5"),
      overridden: Some("^21.3.5"),
      severity: None,
    },
  ]);
}

// ═══════════════════════════════════════════════════════════════════════
// Minor mismatch — preferVersion NOT set
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn has_different_minor_and_no_prefer_version() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "21.3.0" },
      "devDependencies": { "foo": "21.4.0" }
    })])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameMinorMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameMinorMismatch),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "21.4.0",
      expected: Some("21.4.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

// ═══════════════════════════════════════════════════════════════════════
// Minor mismatch — preferVersion: highestSemver — no semver group
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn minor_mismatch_highest_no_semver_group_safe_range() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "21.3.0" },
      "devDependencies": { "foo": "21.4.1" }
    })])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "highestSemver"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.4.1"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "21.4.1",
      expected: Some("21.4.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn minor_mismatch_highest_no_semver_group_tilde_range_preserved() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "~21.3.0" },
      "devDependencies": { "foo": "21.4.1" }
    })])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "highestSemver"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "~21.3.0",
      expected: Some("~21.4.1"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "21.4.1",
      expected: Some("21.4.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn minor_mismatch_highest_no_semver_group_unsafe_caret_range_forced_to_tilde() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "^21.3.0" },
      "devDependencies": { "foo": "21.4.1" }
    })])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "highestSemver"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "^21.3.0",
      expected: Some("~21.4.1"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "21.4.1",
      expected: Some("21.4.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

// ═══════════════════════════════════════════════════════════════════════
// Minor mismatch — preferVersion: highestSemver — with semver group
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn minor_mismatch_highest_safe_tilde_semver_group_applied_to_fix_target() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "21.3.0" },
      "peerDependencies": { "foo": "21.4.1" }
    })])
    .with_semver_group(json!({
      "dependencyTypes": ["dev"],
      "range": "~"
    }))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "highestSemver"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.4.1"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /peerDependencies of my-project",
      actual: "21.4.1",
      expected: Some("21.4.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn minor_mismatch_highest_safe_tilde_semver_group_on_below_target_instance() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "pkg-a",
        "version": "1.0.0",
        "dependencies": { "foo": "21.3.0" }
      }),
      json!({
        "name": "pkg-b",
        "version": "1.0.0",
        "dependencies": { "foo": "21.4.1" }
      }),
    ])
    .with_semver_group(json!({
      "packages": ["pkg-a"],
      "range": "~"
    }))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "highestSemver"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-b",
      id: "pkg-b in /version of pkg-b",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-a",
      actual: "21.3.0",
      expected: Some("~21.4.1"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-b",
      actual: "21.4.1",
      expected: Some("21.4.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn minor_mismatch_highest_unsafe_caret_semver_group_forced_to_tilde_on_fix_target() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "pkg-a",
        "version": "1.0.0",
        "dependencies": { "foo": "21.3.0" }
      }),
      json!({
        "name": "pkg-b",
        "version": "1.0.0",
        "dependencies": { "foo": "21.4.1" }
      }),
    ])
    .with_semver_group(json!({
      "packages": ["pkg-a"],
      "range": "^"
    }))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "highestSemver"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-b",
      id: "pkg-b in /version of pkg-b",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-a",
      actual: "21.3.0",
      expected: Some("~21.4.1"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-b",
      actual: "21.4.1",
      expected: Some("21.4.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

// ═══════════════════════════════════════════════════════════════════════
// Minor mismatch — preferVersion: lowestSemver
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn minor_mismatch_lowest_no_semver_group_safe_range() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "21.3.0" },
      "devDependencies": { "foo": "21.4.1" }
    })])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "lowestSemver"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "21.4.1",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn minor_mismatch_lowest_no_semver_group_unsafe_range_forced_to_tilde() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "21.3.0" },
      "devDependencies": { "foo": "^21.4.1" }
    })])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "lowestSemver"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "^21.4.1",
      expected: Some("~21.3.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

// ═══════════════════════════════════════════════════════════════════════
// Minor mismatch — preferVersion: highestSemver — at target minor,
// semver group interaction (same subtree as "all same MAJOR.MINOR")
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn minor_mismatch_highest_at_target_no_semver_group_safe() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": { "foo": "21.3.0" },
      "devDependencies": { "foo": "21.4.1" }
    })])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "highestSemver"
    }))
    .run()
    .await;
  // Instance at 21.4.1 is at target minor — should be valid
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.4.1"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "21.4.1",
      expected: Some("21.4.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn minor_mismatch_highest_at_target_unsafe_range_overridden() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "pkg-a",
        "version": "1.0.0",
        "dependencies": { "foo": "21.3.0" }
      }),
      json!({
        "name": "pkg-b",
        "version": "1.0.0",
        "dependencies": { "foo": "^21.4.1" }
      }),
    ])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "highestSemver"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-b",
      id: "pkg-b in /version of pkg-b",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-a",
      actual: "21.3.0",
      expected: Some("21.4.1"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SameMinorOverridesSemverRange),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-b",
      actual: "^21.4.1",
      expected: Some("~21.4.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn minor_mismatch_highest_at_target_matches_safe_tilde_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "pkg-a",
        "version": "1.0.0",
        "dependencies": { "foo": "21.3.0" }
      }),
      json!({
        "name": "pkg-b",
        "version": "1.0.0",
        "peerDependencies": { "foo": "~21.4.1" }
      }),
    ])
    .with_semver_group(json!({
      "dependencyTypes": ["peer"],
      "range": "~"
    }))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "highestSemver"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-b",
      id: "pkg-b in /version of pkg-b",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-a",
      actual: "21.3.0",
      expected: Some("21.4.1"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /peerDependencies of pkg-b",
      actual: "~21.4.1",
      expected: Some("~21.4.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn minor_mismatch_highest_at_target_mismatches_safe_tilde_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "pkg-a",
        "version": "1.0.0",
        "dependencies": { "foo": "21.3.0" }
      }),
      json!({
        "name": "pkg-b",
        "version": "1.0.0",
        "peerDependencies": { "foo": "21.4.1" }
      }),
    ])
    .with_semver_group(json!({
      "dependencyTypes": ["peer"],
      "range": "~"
    }))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "highestSemver"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-b",
      id: "pkg-b in /version of pkg-b",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-a",
      actual: "21.3.0",
      expected: Some("21.4.1"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SemverRangeMismatch),
      dependency_name: "foo",
      id: "foo in /peerDependencies of pkg-b",
      actual: "21.4.1",
      expected: Some("~21.4.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn minor_mismatch_highest_at_target_matches_unsafe_caret_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "pkg-a",
        "version": "1.0.0",
        "dependencies": { "foo": "21.3.0" }
      }),
      json!({
        "name": "pkg-b",
        "version": "1.0.0",
        "peerDependencies": { "foo": "^21.4.1" }
      }),
    ])
    .with_semver_group(json!({
      "dependencyTypes": ["peer"],
      "range": "^"
    }))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "highestSemver"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-b",
      id: "pkg-b in /version of pkg-b",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-a",
      actual: "21.3.0",
      expected: Some("21.4.1"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SameMinorOverridesSemverRange),
      dependency_name: "foo",
      id: "foo in /peerDependencies of pkg-b",
      actual: "^21.4.1",
      expected: Some("~21.4.1"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn minor_mismatch_highest_at_target_mismatches_unsafe_caret_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "pkg-a",
        "version": "1.0.0",
        "dependencies": { "foo": "21.3.0" }
      }),
      json!({
        "name": "pkg-b",
        "version": "1.0.0",
        "peerDependencies": { "foo": "21.4.1" }
      }),
    ])
    .with_semver_group(json!({
      "dependencyTypes": ["peer"],
      "range": "^"
    }))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "highestSemver"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-b",
      id: "pkg-b in /version of pkg-b",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-a",
      actual: "21.3.0",
      expected: Some("21.4.1"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SameMinorOverridesSemverRangeMismatch),
      dependency_name: "foo",
      id: "foo in /peerDependencies of pkg-b",
      actual: "21.4.1",
      expected: Some("~21.4.1"),
      overridden: Some("^21.4.1"),
      severity: None,
    },
  ]);
}

// ═══════════════════════════════════════════════════════════════════════
// Three instances — one at target, two below — verifies multiple
// instances all get DiffersToHighestOrLowestSemverMinor
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn minor_mismatch_highest_multiple_below_target() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "pkg-a",
        "version": "1.0.0",
        "dependencies": { "foo": "5.1.0" }
      }),
      json!({
        "name": "pkg-b",
        "version": "1.0.0",
        "dependencies": { "foo": "5.2.0" }
      }),
      json!({
        "name": "pkg-c",
        "version": "1.0.0",
        "dependencies": { "foo": "5.3.0" }
      }),
    ])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor",
      "preferVersion": "highestSemver"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-b",
      id: "pkg-b in /version of pkg-b",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-c",
      id: "pkg-c in /version of pkg-c",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-a",
      actual: "5.1.0",
      expected: Some("5.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-b",
      actual: "5.2.0",
      expected: Some("5.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-c",
      actual: "5.3.0",
      expected: Some("5.3.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

// ═══════════════════════════════════════════════════════════════════════
// All same minor — all valid — all patches differ (patch has no policy
// meaning)
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn all_same_minor_different_patches_all_valid() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "pkg-a",
        "version": "1.0.0",
        "dependencies": { "foo": "5.3.0" }
      }),
      json!({
        "name": "pkg-b",
        "version": "1.0.0",
        "dependencies": { "foo": "5.3.7" }
      }),
      json!({
        "name": "pkg-c",
        "version": "1.0.0",
        "dependencies": { "foo": "5.3.99" }
      }),
    ])
    .with_version_group(json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-b",
      id: "pkg-b in /version of pkg-b",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-c",
      id: "pkg-c in /version of pkg-c",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-a",
      actual: "5.3.0",
      expected: Some("5.3.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-b",
      actual: "5.3.7",
      expected: Some("5.3.7"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of pkg-c",
      actual: "5.3.99",
      expected: Some("5.3.99"),
      overridden: None,
      severity: None,
    },
  ]);
}

/// Severity tests — opt out of auto-fix per status (issue #216).
/// SameMinor permits `DiffersToHighestOrLowestSemverMinor`,
/// `SemverRangeMismatch`, `SameMinorOverridesSemverRange`,
/// `SameMinorOverridesSemverRangeMismatch`.
mod severity {
  use {super::*, crate::instance::Severity};

  /// Scenario: dependencies foo 21.3.0 vs devDependencies foo 21.4.1, same
  /// major different minor, preferVersion highestSemver. `21.3.0` →
  /// `DiffersToHighestOrLowestSemverMinor` Fixable, severity downgrades to
  /// `Warn`. `21.4.1` is `SatisfiesSameMinorGroup`.
  #[tokio::test]
  async fn differs_to_highest_or_lowest_semver_minor_warn() {
    let ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "my-project",
        "version": "1.0.0",
        "dependencies": { "foo": "21.3.0" },
        "devDependencies": { "foo": "21.4.1" }
      })])
      .with_version_group(json!({
        "dependencies": ["foo"],
        "policy": "sameMinor",
        "preferVersion": "highestSemver",
        "severity": {"DiffersToHighestOrLowestSemverMinor": "warn"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "my-project",
        id: "my-project in /version of my-project",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
        dependency_name: "foo",
        id: "foo in /dependencies of my-project",
        actual: "21.3.0",
        expected: Some("21.4.1"),
        overridden: None,
        severity: Some(Severity::Warn),
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesSameMinorGroup),
        dependency_name: "foo",
        id: "foo in /devDependencies of my-project",
        actual: "21.4.1",
        expected: Some("21.4.1"),
        overridden: None,
        severity: Some(Severity::None),
      },
    ]);
  }

  /// `severity: { DiffersToHighestOrLowestSemverMinor: "error" }` → `Error`.
  #[tokio::test]
  async fn differs_to_highest_or_lowest_semver_minor_error() {
    let ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "my-project",
        "version": "1.0.0",
        "dependencies": { "foo": "21.3.0" },
        "devDependencies": { "foo": "21.4.1" }
      })])
      .with_version_group(json!({
        "dependencies": ["foo"],
        "policy": "sameMinor",
        "preferVersion": "highestSemver",
        "severity": {"DiffersToHighestOrLowestSemverMinor": "error"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "my-project",
        id: "my-project in /version of my-project",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToHighestOrLowestSemverMinor),
        dependency_name: "foo",
        id: "foo in /dependencies of my-project",
        actual: "21.3.0",
        expected: Some("21.4.1"),
        overridden: None,
        severity: Some(Severity::Error),
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesSameMinorGroup),
        dependency_name: "foo",
        id: "foo in /devDependencies of my-project",
        actual: "21.4.1",
        expected: Some("21.4.1"),
        overridden: None,
        severity: Some(Severity::None),
      },
    ]);
  }

  /// Scenario: dependencies and peer both `21.3.5`; semver group on peer
  /// requires tilde. peer becomes `SemverRangeMismatch` (range fix). severity
  /// downgrades to `Warn`.
  #[tokio::test]
  async fn semver_range_mismatch_warn() {
    let ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "my-project",
        "version": "1.0.0",
        "dependencies": { "foo": "21.3.5" },
        "peerDependencies": { "foo": "21.3.5" }
      })])
      .with_semver_group(json!({"dependencyTypes": ["peer"], "range": "~"}))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "policy": "sameMinor",
        "severity": {"SemverRangeMismatch": "warn"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "my-project",
        id: "my-project in /version of my-project",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesSameMinorGroup),
        dependency_name: "foo",
        id: "foo in /dependencies of my-project",
        actual: "21.3.5",
        expected: Some("21.3.5"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(SemverRangeMismatch),
        dependency_name: "foo",
        id: "foo in /peerDependencies of my-project",
        actual: "21.3.5",
        expected: Some("~21.3.5"),
        overridden: None,
        severity: Some(Severity::Warn),
      },
    ]);
  }

  /// `severity: { SemverRangeMismatch: "error" }` → `Error`.
  #[tokio::test]
  async fn semver_range_mismatch_error() {
    let ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "my-project",
        "version": "1.0.0",
        "dependencies": { "foo": "21.3.5" },
        "peerDependencies": { "foo": "21.3.5" }
      })])
      .with_semver_group(json!({"dependencyTypes": ["peer"], "range": "~"}))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "policy": "sameMinor",
        "severity": {"SemverRangeMismatch": "error"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "my-project",
        id: "my-project in /version of my-project",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesSameMinorGroup),
        dependency_name: "foo",
        id: "foo in /dependencies of my-project",
        actual: "21.3.5",
        expected: Some("21.3.5"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(SemverRangeMismatch),
        dependency_name: "foo",
        id: "foo in /peerDependencies of my-project",
        actual: "21.3.5",
        expected: Some("~21.3.5"),
        overridden: None,
        severity: Some(Severity::Error),
      },
    ]);
  }

  /// Scenario: caret on disk is unsafe under sameMinor policy → no semver
  /// group, on-disk range `^21.3.1` is replaced with `~21.3.1`. State is
  /// `SameMinorOverridesSemverRange`. severity downgrades to `Warn`.
  #[tokio::test]
  async fn same_minor_overrides_semver_range_warn() {
    let ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "my-project",
        "version": "1.0.0",
        "dependencies": { "foo": "21.3.0" },
        "devDependencies": { "foo": "^21.3.1" }
      })])
      .with_version_group(json!({
        "dependencies": ["foo"],
        "policy": "sameMinor",
        "severity": {"SameMinorOverridesSemverRange": "warn"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "my-project",
        id: "my-project in /version of my-project",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesSameMinorGroup),
        dependency_name: "foo",
        id: "foo in /dependencies of my-project",
        actual: "21.3.0",
        expected: Some("21.3.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(SameMinorOverridesSemverRange),
        dependency_name: "foo",
        id: "foo in /devDependencies of my-project",
        actual: "^21.3.1",
        expected: Some("~21.3.1"),
        overridden: None,
        severity: Some(Severity::Warn),
      },
    ]);
  }

  /// `severity: { SameMinorOverridesSemverRange: "error" }` → `Error`.
  #[tokio::test]
  async fn same_minor_overrides_semver_range_error() {
    let ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "my-project",
        "version": "1.0.0",
        "dependencies": { "foo": "21.3.0" },
        "devDependencies": { "foo": "^21.3.1" }
      })])
      .with_version_group(json!({
        "dependencies": ["foo"],
        "policy": "sameMinor",
        "severity": {"SameMinorOverridesSemverRange": "error"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "my-project",
        id: "my-project in /version of my-project",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesSameMinorGroup),
        dependency_name: "foo",
        id: "foo in /dependencies of my-project",
        actual: "21.3.0",
        expected: Some("21.3.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(SameMinorOverridesSemverRange),
        dependency_name: "foo",
        id: "foo in /devDependencies of my-project",
        actual: "^21.3.1",
        expected: Some("~21.3.1"),
        overridden: None,
        severity: Some(Severity::Error),
      },
    ]);
  }

  /// Scenario: exact `21.3.5` in peer mismatches caret semver group; under
  /// sameMinor policy `~` wins (~21.3.5) over caret. State is
  /// `SameMinorOverridesSemverRangeMismatch`. severity downgrades to `Warn`.
  #[tokio::test]
  async fn same_minor_overrides_semver_range_mismatch_warn() {
    let ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "my-project",
        "version": "1.0.0",
        "dependencies": { "foo": "21.3.0" },
        "peerDependencies": { "foo": "21.3.5" }
      })])
      .with_semver_group(json!({"dependencyTypes": ["peer"], "range": "^"}))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "policy": "sameMinor",
        "severity": {"SameMinorOverridesSemverRangeMismatch": "warn"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "my-project",
        id: "my-project in /version of my-project",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesSameMinorGroup),
        dependency_name: "foo",
        id: "foo in /dependencies of my-project",
        actual: "21.3.0",
        expected: Some("21.3.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(SameMinorOverridesSemverRangeMismatch),
        dependency_name: "foo",
        id: "foo in /peerDependencies of my-project",
        actual: "21.3.5",
        expected: Some("~21.3.5"),
        overridden: Some("^21.3.5"),
        severity: Some(Severity::Warn),
      },
    ]);
  }

  /// `severity: { SameMinorOverridesSemverRangeMismatch: "error" }` → `Error`.
  #[tokio::test]
  async fn same_minor_overrides_semver_range_mismatch_error() {
    let ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "my-project",
        "version": "1.0.0",
        "dependencies": { "foo": "21.3.0" },
        "peerDependencies": { "foo": "21.3.5" }
      })])
      .with_semver_group(json!({"dependencyTypes": ["peer"], "range": "^"}))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "policy": "sameMinor",
        "severity": {"SameMinorOverridesSemverRangeMismatch": "error"}
      }))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "my-project",
        id: "my-project in /version of my-project",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesSameMinorGroup),
        dependency_name: "foo",
        id: "foo in /dependencies of my-project",
        actual: "21.3.0",
        expected: Some("21.3.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(SameMinorOverridesSemverRangeMismatch),
        dependency_name: "foo",
        id: "foo in /peerDependencies of my-project",
        actual: "21.3.5",
        expected: Some("~21.3.5"),
        overridden: Some("^21.3.5"),
        severity: Some(Severity::Error),
      },
    ]);
  }

  /// `IsBanned` is a Banned-only key; using it on a SameMinor group →
  /// `InvalidSeverityKey`.
  #[tokio::test]
  #[should_panic(expected = "InvalidSeverityKey")]
  async fn rejects_invalid_severity_key() {
    let _ctx = TestBuilder::new()
      .with_package(json!({
        "name": "my-project",
        "version": "1.0.0",
        "dependencies": {"foo": "1.0.0"}
      }))
      .with_version_group(json!({
        "dependencies": ["foo"],
        "policy": "sameMinor",
        "severity": {"IsBanned": "warn"}
      }))
      .run()
      .await;
  }
}
