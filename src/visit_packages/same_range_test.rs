use {
  crate::{
    instance_state::{FixableInstance::*, InstanceState, UnfixableInstance::*, ValidInstance::*},
    test::{
      self,
      expect::{expect, ExpectedInstance},
    },
    visit_packages::visit_packages,
    Context,
  },
  serde_json::json,
};

#[test]
fn instance_in_a_same_range_group_satisfies_every_other_and_there_are_no_semver_groups() {
  let config = test::mock::config_from_mock(json!({
    "versionGroups": [{
      "dependencyTypes": ["local"],
      "isIgnored": true
    }, {
      "dependencies": ["foo"],
      "policy": "sameRange"
    }]
  }));
  let packages = test::mock::packages_from_mocks(vec![
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
  ]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);
  let ctx = visit_packages(ctx);
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-b",
      id: "package-b in /version of package-b",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: ">=1.0.0",
      expected: Some(">=1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "<=2.0.0",
      expected: Some("<=2.0.0"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_in_a_same_range_group_satisfies_every_other_and_matches_its_semver_group() {
  let config = test::mock::config_from_mock(json!({
    "semverGroups": [{
      "packages": ["package-b"],
      "range": "^"
    }],
    "versionGroups": [{
      "dependencyTypes": ["local"],
      "isIgnored": true
    }, {
      "dependencies": ["foo"],
      "policy": "sameRange"
    }]
  }));
  let packages = test::mock::packages_from_mocks(vec![
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
  ]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);
  let ctx = visit_packages(ctx);
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-b",
      id: "package-b in /version of package-b",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: ">=1.0.0",
      expected: Some(">=1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "^1.2.3",
      expected: Some("^1.2.3"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_in_a_same_range_group_satisfies_every_other_but_mismatches_its_semver_group() {
  let config = test::mock::config_from_mock(json!({
    "semverGroups": [{
      "packages": ["package-b"],
      "range": "~"
    }],
    "versionGroups": [{
      "dependencyTypes": ["local"],
      "isIgnored": true
    }, {
      "dependencies": ["foo"],
      "policy": "sameRange"
    }]
  }));
  let packages = test::mock::packages_from_mocks(vec![
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
  ]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);
  let ctx = visit_packages(ctx);
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-b",
      id: "package-b in /version of package-b",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: ">=1.0.0",
      expected: Some(">=1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SemverRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "^1.2.3",
      expected: Some("~1.2.3"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_in_a_same_range_group_does_not_satisfy_another() {
  let config = test::mock::config_from_mock(json!({
    "versionGroups": [{
      "dependencyTypes": ["local"],
      "isIgnored": true
    }, {
      "dependencies": ["foo"],
      "policy": "sameRange"
    }]
  }));
  let packages = test::mock::packages_from_mocks(vec![
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
  ]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);
  let ctx = visit_packages(ctx);
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-b",
      id: "package-b in /version of package-b",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: ">=1.0.0",
      expected: Some(">=1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "<1.0.0",
      expected: Some("<1.0.0"),
      overridden: None,
    },
  ]);
}
