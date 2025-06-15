use {
  crate::{
    instance_state::{FixableInstance::*, InstanceState, SuspectInstance::*, ValidInstance::*},
    test::{
      builder::TestBuilder,
      expect::{expect, ExpectedInstance},
    },
  },
  serde_json::json,
};

mod local {
  use super::*;

  #[test]
  fn refuses_to_pin_local_version() {
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
      .build_and_visit();
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(RefuseToPinLocal),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToPin),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "1.1.0",
        expected: Some("1.2.0"),
        overridden: None,
      },
    ]);
  }
}

mod normal {
  use super::*;

  #[test]
  fn a_pinned_version_will_replace_anything_different() {
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
      .build_and_visit();
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToPin),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: "workspace:*",
        expected: Some("1.2.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn pin_version_will_override_instance_with_same_version_number_as_pinned_but_matching_a_semver_group() {
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
      .build_and_visit();
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::fixable(PinOverridesSemverRange),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: "^1.0.0",
        expected: Some("1.0.0"),
        overridden: Some("^1.0.0"),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn pin_version_will_override_instance_with_same_version_number_as_pinned_but_mismatching_a_semver_group() {
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
      .build_and_visit();
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::fixable(PinOverridesSemverRangeMismatch),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: ">=1.0.0",
        expected: Some("1.0.0"),
        overridden: Some("^1.0.0"),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn pin_version_will_override_instance_with_same_version_number_as_pinned_but_a_different_range_and_no_semver_group() {
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
      .build_and_visit();
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToPin),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: "^1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn an_already_pinned_version_is_valid() {
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
      .build_and_visit();
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsIdenticalToPin),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-a",
        actual: "1.2.0",
        expected: Some("1.2.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn an_already_pinned_workspace_protocol_version_is_valid() {
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
      .build_and_visit();
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsIdenticalToPin),
        dependency_name: "package-a",
        id: "package-a in /devDependencies of package-a",
        actual: "workspace:*",
        expected: Some("workspace:*"),
        overridden: None,
      },
    ]);
  }
}
