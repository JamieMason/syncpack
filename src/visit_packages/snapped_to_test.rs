use {
  crate::{
    instance_state::{FixableInstance::*, InstanceState, SemverGroupAndVersionConflict::*, SuspectInstance::*, ValidInstance::*},
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
fn instance_identical_to_snapped_to_and_has_no_semver_group() {
  let config = test::mock::config_from_mock(json!({
    "versionGroups": [{
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }]
  }));
  let packages = test::mock::packages_from_mocks(vec![
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
  ]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);
  let ctx = visit_packages(ctx);
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_has_different_version_to_snapped_to_and_has_no_semver_group() {
  let config = test::mock::config_from_mock(json!({
    "versionGroups": [{
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }]
  }));
  let packages = test::mock::packages_from_mocks(vec![
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
  ]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);
  let ctx = visit_packages(ctx);
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "1.1.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_has_same_version_number_as_snapped_to_but_a_different_range_and_has_no_semver_group() {
  let config = test::mock::config_from_mock(json!({
    "versionGroups": [{
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }]
  }));
  let packages = test::mock::packages_from_mocks(vec![
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
  ]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);
  let ctx = visit_packages(ctx);
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "^1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToSnapTarget),
      dependency_name: "foo",
      id: "foo in /devDependencies of follower",
      actual: "~1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_has_same_version_number_as_snapped_to_but_matches_a_different_but_compatible_semver_group() {
  let config = test::mock::config_from_mock(json!({
    "semverGroups": [{
      "packages": ["follower"],
      "range": "~"
    }],
    "versionGroups": [{
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }]
  }));
  let packages = test::mock::packages_from_mocks(vec![
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
  ]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);
  let ctx = visit_packages(ctx);
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "~1.0.0",
      expected: Some("~1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "^1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_has_same_version_number_as_snapped_to_but_mismatches_a_different_but_compatible_semver_group() {
  let config = test::mock::config_from_mock(json!({
    "semverGroups": [{
      "packages": ["follower"],
      "range": "^"
    }],
    "versionGroups": [{
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }]
  }));
  let packages = test::mock::packages_from_mocks(vec![
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
  ]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);
  let ctx = visit_packages(ctx);
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: ">=1.0.0",
      expected: Some(">=1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(SemverRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "~1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_has_same_version_number_as_snapped_to_but_matches_a_different_but_incompatible_semver_group() {
  let config = test::mock::config_from_mock(json!({
    "semverGroups": [{
      "packages": ["follower"],
      "range": "<"
    }],
    "versionGroups": [{
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }]
  }));
  let packages = test::mock::packages_from_mocks(vec![
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
  ]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);
  let ctx = visit_packages(ctx);
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::conflict(MatchConflictsWithSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "<1.0.0",
      expected: Some("<1.0.0"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_has_same_version_number_as_snapped_to_but_mismatches_a_different_but_incompatible_semver_group() {
  let config = test::mock::config_from_mock(json!({
    "semverGroups": [{
      "packages": ["follower"],
      "range": "<"
    }],
    "versionGroups": [{
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }]
  }));
  let packages = test::mock::packages_from_mocks(vec![
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
  ]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);
  let ctx = visit_packages(ctx);
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "~1.0.0",
      expected: Some("~1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::conflict(MismatchConflictsWithSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_cannot_find_a_snapped_to_version() {
  let config = test::mock::config_from_mock(json!({
    "versionGroups": [{
      "dependencies": ["foo"],
      "packages": ["follower"],
      "snapTo": ["leader"]
    }]
  }));
  let packages = test::mock::packages_from_mocks(vec![
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
  ]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);
  let ctx = visit_packages(ctx);
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "0.1.0",
      expected: Some("0.1.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(DependsOnMissingSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
  ]);
}

#[test]
fn instance_is_in_a_snapped_to_group_and_is_itself_a_snapped_to_target() {
  let config = test::mock::config_from_mock(json!({
    "versionGroups": [{
      "dependencies": ["foo"],
      "snapTo": ["leader"]
    }]
  }));
  let packages = test::mock::packages_from_mocks(vec![
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
  ]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);
  let ctx = visit_packages(ctx);
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "leader",
      id: "leader in /version of leader",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "follower",
      id: "follower in /version of follower",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of leader",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToSnapTarget),
      dependency_name: "foo",
      id: "foo in /dependencies of follower",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
  ]);
}

#[test]
fn refuses_to_snap_local_version_to_another_target() {
  let config = test::mock::config_from_mock(json!({
    "versionGroups": [{
      "snapTo": ["package-b"]
    }]
  }));
  let packages = test::mock::packages_from_mocks(vec![
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
  ]);
  let registry_client = None;
  let ctx = Context::create(config, packages, registry_client);
  let ctx = visit_packages(ctx);
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(RefuseToSnapLocal),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.1.0",
      expected: Some("1.1.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToSnapTarget),
      dependency_name: "package-b",
      id: "package-b in /version of package-b",
      actual: "0.1.0",
      expected: Some("0.1.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToSnapTarget),
      dependency_name: "package-a",
      id: "package-a in /dependencies of package-b",
      actual: "0.0.1",
      expected: Some("0.0.1"),
      overridden: None,
    },
  ]);
}
