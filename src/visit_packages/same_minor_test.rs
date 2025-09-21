use {
  crate::{
    instance_state::{FixableInstance::*, InstanceState, UnfixableInstance::*, ValidInstance::*},
    test::{
      builder::TestBuilder,
      expect::{expect, ExpectedInstance},
    },
  },
  serde_json::json,
};

#[test]
fn instance_in_a_same_minor_group_satisfies_every_other_and_there_are_no_semver_groups() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": {
        "foo": "21.3.0"
      },
      "devDependencies": {
        "foo": "21.3.1"
      }
    })])
    .with_version_groups(vec![json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    })])
    .build_and_visit_packages();
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "21.3.1",
      expected: Some("21.3.1"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_in_a_same_minor_group_has_different_minor_and_there_are_no_semver_groups() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": {
        "foo": "21.3.0"
      },
      "devDependencies": {
        "foo": "22.1.0"
      }
    })])
    .with_version_groups(vec![json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    })])
    .build_and_visit_packages();
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameMinorMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameMinorMismatch),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "22.1.0",
      expected: Some("22.1.0"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_in_a_same_minor_group_has_same_minor_and_a_compatible_semver_range_and_there_are_no_semver_groups() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": {
        "foo": "21.3.0"
      },
      "devDependencies": {
        "foo": "~22.3.1"
      }
    })])
    .with_version_groups(vec![json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    })])
    .build_and_visit_packages();
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "~22.3.1",
      expected: Some("~22.3.1"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_in_a_same_minor_group_has_same_minor_but_an_incompatible_semver_range_and_there_are_no_semver_groups() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": {
        "foo": "21.3.0"
      },
      "devDependencies": {
        "foo": "^22.3.1"
      }
    })])
    .with_version_groups(vec![json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    })])
    .build_and_visit_packages();
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameMinorMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameMinorMismatch),
      dependency_name: "foo",
      id: "foo in /devDependencies of my-project",
      actual: "^22.3.1",
      expected: Some("^22.3.1"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_in_a_same_minor_group_satisfies_every_other_and_matches_its_compatible_tilde_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": {
        "foo": "21.3.0"
      },
      "peerDependencies": {
        "foo": "~21.3.5"
      }
    })])
    .with_semver_group(json!({
      "dependencyTypes": ["peer"],
      "range": "~"
    }))
    .with_version_groups(vec![json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    })])
    .build_and_visit_packages();
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameMinorGroup),
      dependency_name: "foo",
      id: "foo in /peerDependencies of my-project",
      actual: "~21.3.5",
      expected: Some("~21.3.5"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_in_a_same_minor_group_satisfies_every_other_but_mismatches_its_compatible_tilde_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": {
        "foo": "21.3.0"
      },
      "peerDependencies": {
        "foo": "21.3.5"
      }
    })])
    .with_semver_group(json!({
      "dependencyTypes": ["peer"],
      "range": "~"
    }))
    .with_version_groups(vec![json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    })])
    .build_and_visit_packages();
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameMinorMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SemverRangeMismatch),
      dependency_name: "foo",
      id: "foo in /peerDependencies of my-project",
      actual: "21.3.5",
      expected: Some("~21.3.5"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_in_a_same_minor_group_has_semver_number_which_satisfies_every_other_but_range_matches_an_incompatible_caret_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": {
        "foo": "21.3.0"
      },
      "peerDependencies": {
        "foo": "^21.3.5"
      }
    })])
    .with_semver_group(json!({
      "dependencyTypes": ["peer"],
      "range": "^"
    }))
    .with_version_groups(vec![json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    })])
    .build_and_visit_packages();
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameMinorMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SameMinorOverridesSemverRange),
      dependency_name: "foo",
      id: "foo in /peerDependencies of my-project",
      actual: "^21.3.5",
      expected: Some("21.3.5"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_in_a_same_minor_group_satisfies_every_other_but_mismatches_its_incompatible_caret_semver_group() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "my-project",
      "version": "1.0.0",
      "dependencies": {
        "foo": "21.3.0"
      },
      "peerDependencies": {
        "foo": "21.3.5"
      }
    })])
    .with_semver_group(json!({
      "dependencyTypes": ["peer"],
      "range": "^"
    }))
    .with_version_groups(vec![json!({
      "dependencies": ["foo"],
      "policy": "sameMinor"
    })])
    .build_and_visit_packages();
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "my-project",
      id: "my-project in /version of my-project",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameMinorMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of my-project",
      actual: "21.3.0",
      expected: Some("21.3.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SameMinorOverridesSemverRangeMismatch),
      dependency_name: "foo",
      id: "foo in /peerDependencies of my-project",
      actual: "21.3.5",
      expected: Some("21.3.5"),
      overridden: None,
    },
  ]);
}
