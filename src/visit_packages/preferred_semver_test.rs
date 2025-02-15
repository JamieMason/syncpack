use {
  crate::{
    cli::UpdateTarget,
    instance_state::{
      FixableInstance::*, InstanceState, SemverGroupAndVersionConflict::*, SuspectInstance::*, UnfixableInstance::*, ValidInstance::*,
    },
    test::{
      self,
      expect::{expect, ExpectedInstance},
    },
    visit_packages::visit_packages,
    Context,
  },
  serde_json::json,
};

mod local {
  use super::*;

  #[test]
  fn instance_depends_on_local_version_which_is_missing() {
    let config = test::mock::config();
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a"
      }),
      json!({
        "name": "package-b",
        "devDependencies": {
          "package-a": "0.1.0"
        }
      }),
    ]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(DependsOnInvalidLocalPackage),
        dependency_name: "package-a",
        id: "package-a in /devDependencies of package-b",
        actual: "0.1.0",
        expected: Some("0.1.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_depends_on_local_version_which_is_not_exact_semver() {
    let config = test::mock::config();
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "version": "^1.0.0"
      }),
      json!({
        "name": "package-b",
        "devDependencies": {
          "package-a": "0.1.0"
        }
      }),
    ]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "^1.0.0",
        expected: Some("^1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(DependsOnInvalidLocalPackage),
        dependency_name: "package-a",
        id: "package-a in /devDependencies of package-b",
        actual: "0.1.0",
        expected: Some("0.1.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_has_higher_version_than_local_package_and_has_no_semver_group() {
    let config = test::mock::config();
    let packages = test::mock::packages_from_mocks(vec![
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
    ]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToLocal),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "1.1.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_identical_to_local_package_and_has_no_semver_group() {
    let config = test::mock::config();
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "version": "1.0.0"
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "package-a": "1.0.0"
        }
      }),
    ]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsIdenticalToLocal),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_has_different_version_to_local_package_and_has_no_semver_group() {
    let config = test::mock::config();
    let packages = test::mock::packages_from_mocks(vec![
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
    ]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToLocal),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "1.1.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_has_same_version_number_as_local_package_but_a_different_range_and_has_no_semver_group() {
    let config = test::mock::config();
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "version": "1.0.0"
      }),
      json!({
        "name": "package-b",
        "devDependencies": {
          "package-a": "~1.0.0"
        }
      }),
    ]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToLocal),
        dependency_name: "package-a",
        id: "package-a in /devDependencies of package-b",
        actual: "~1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_has_same_version_number_as_local_package_but_matches_a_different_but_compatible_semver_group() {
    let config = test::mock::config_from_mock(json!({
      "semverGroups": [{
        "range": "^"
      }]
    }));
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "version": "1.0.0"
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "package-a": "^1.0.0"
        }
      }),
    ]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesLocal),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "^1.0.0",
        expected: Some("^1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_has_same_version_number_as_local_package_but_mismatches_a_different_but_compatible_semver_group() {
    let config = test::mock::config_from_mock(json!({
      "semverGroups": [{
        "range": "^"
      }]
    }));
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "version": "1.0.0"
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "package-a": "~1.0.0"
        }
      }),
    ]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(SemverRangeMismatch),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "~1.0.0",
        expected: Some("^1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_has_same_version_number_as_local_package_but_matches_a_different_but_incompatible_semver_group() {
    let config = test::mock::config_from_mock(json!({
      "semverGroups": [{
        "range": "<"
      }]
    }));
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "version": "1.0.0"
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "package-a": "<1.0.0"
        }
      }),
    ]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::conflict(MatchConflictsWithLocal),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "<1.0.0",
        expected: Some("<1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_has_same_version_number_as_local_package_but_mismatches_a_different_but_incompatible_semver_group() {
    let config = test::mock::config_from_mock(json!({
      "semverGroups": [{
        "range": ">"
      }]
    }));
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "version": "1.0.0"
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "package-a": "~1.0.0"
        }
      }),
    ]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::conflict(MismatchConflictsWithLocal),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "~1.0.0",
        expected: Some("~1.0.0"),
        overridden: None,
      },
    ]);
  }
}

mod highest_or_lowest {
  use super::*;

  #[test]
  fn reports_one_highest_version_mismatch_in_one_file() {
    let config = test::mock::config();
    let packages = test::mock::packages_from_mocks(vec![json!({
      "name": "package-a",
      "dependencies": {
        "wat": "1.0.0"
      },
      "devDependencies": {
        "wat": "2.0.0"
      }
    })]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "wat",
        id: "wat in /devDependencies of package-a",
        actual: "2.0.0",
        expected: Some("2.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToHighestOrLowestSemver),
        dependency_name: "wat",
        id: "wat in /dependencies of package-a",
        actual: "1.0.0",
        expected: Some("2.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn reports_many_highest_version_mismatches_in_one_file() {
    let config = test::mock::config();
    let packages = test::mock::packages_from_mocks(vec![json!({
      "name": "package-a",
      "dependencies": {
        "wat": "0.1.0"
      },
      "devDependencies": {
        "wat": "0.3.0"
      },
      "peerDependencies": {
        "wat": "0.2.0"
      }
    })]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "wat",
        id: "wat in /devDependencies of package-a",
        actual: "0.3.0",
        expected: Some("0.3.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToHighestOrLowestSemver),
        dependency_name: "wat",
        id: "wat in /dependencies of package-a",
        actual: "0.1.0",
        expected: Some("0.3.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToHighestOrLowestSemver),
        dependency_name: "wat",
        id: "wat in /peerDependencies of package-a",
        actual: "0.2.0",
        expected: Some("0.3.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn reports_highest_version_mismatches_in_many_files() {
    let config = test::mock::config();
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "wat": "1.0.0"
        }
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "wat": "2.0.0"
        }
      }),
    ]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "wat",
        id: "wat in /dependencies of package-b",
        actual: "2.0.0",
        expected: Some("2.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToHighestOrLowestSemver),
        dependency_name: "wat",
        id: "wat in /dependencies of package-a",
        actual: "1.0.0",
        expected: Some("2.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn does_not_report_highest_version_mismatches_when_in_different_version_groups() {
    let config = test::mock::config_from_mock(json!({
      "versionGroups": [
        { "packages": ["package-a"] },
        { "packages": ["package-b"] }
      ]
    }));
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "good": "1.0.0"
        }
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "good": "2.0.0"
        }
      }),
    ]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "good",
        id: "good in /dependencies of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "good",
        id: "good in /dependencies of package-b",
        actual: "2.0.0",
        expected: Some("2.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn does_not_confuse_highest_version_matches_and_mismatches_of_the_same_dependency() {
    let config = test::mock::config();
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "version": "0.0.0",
        "dependencies": {
          "mix": "0.3.0"
        },
        "devDependencies": {
          "mix": "0.1.0"
        },
        "peerDependencies": {
          "mix": "0.2.0"
        }
      }),
      json!({
        "name": "package-b",
        "version": "0.0.0",
        "devDependencies": {
          "mix": "0.3.0"
        }
      }),
    ]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "mix",
        id: "mix in /dependencies of package-a",
        actual: "0.3.0",
        expected: Some("0.3.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "mix",
        id: "mix in /devDependencies of package-b",
        actual: "0.3.0",
        expected: Some("0.3.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToHighestOrLowestSemver),
        dependency_name: "mix",
        id: "mix in /devDependencies of package-a",
        actual: "0.1.0",
        expected: Some("0.3.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToHighestOrLowestSemver),
        dependency_name: "mix",
        id: "mix in /peerDependencies of package-a",
        actual: "0.2.0",
        expected: Some("0.3.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_identical_to_highest_semver_and_has_no_semver_group() {
    let config = test::mock::config();
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": "1.0.0"
        }
      }),
      json!({
        "name": "package-b",
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
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of package-b",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_has_different_version_to_highest_semver_and_has_no_semver_group() {
    let config = test::mock::config();
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": "1.0.0"
        }
      }),
      json!({
        "name": "package-b",
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
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of package-b",
        actual: "1.1.0",
        expected: Some("1.1.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of package-a",
        actual: "1.0.0",
        expected: Some("1.1.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_has_same_version_number_as_highest_semver_but_a_different_range_and_has_no_semver_group() {
    let config = test::mock::config();
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": "^1.0.0"
        }
      }),
      json!({
        "name": "package-b",
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
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of package-a",
        actual: "^1.0.0",
        expected: Some("^1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /devDependencies of package-b",
        actual: "~1.0.0",
        expected: Some("^1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_has_same_version_number_as_highest_semver_but_matches_a_different_but_compatible_semver_group() {
    let config = test::mock::config_from_mock(json!({
      "semverGroups": [{
        "packages": ["package-b"],
        "range": "~"
      }]
    }));
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": "^1.0.0"
        }
      }),
      json!({
        "name": "package-b",
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
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of package-b",
        actual: "~1.0.0",
        expected: Some("~1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of package-a",
        actual: "^1.0.0",
        expected: Some("^1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_has_same_version_number_as_highest_semver_but_mismatches_a_different_but_compatible_semver_group() {
    let config = test::mock::config_from_mock(json!({
      "semverGroups": [{
        "packages": ["package-b"],
        "range": "^"
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
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
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
        actual: "~1.0.0",
        expected: Some("^1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_has_same_version_number_as_highest_semver_but_matches_a_different_but_incompatible_semver_group() {
    let config = test::mock::config_from_mock(json!({
      "semverGroups": [{
        "packages": ["package-b"],
        "range": "<"
      }]
    }));
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": "1.0.0"
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
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::conflict(MatchConflictsWithHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of package-b",
        actual: "<1.0.0",
        expected: Some("<1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_has_same_version_number_as_highest_semver_but_mismatches_a_different_but_incompatible_semver_group() {
    let config = test::mock::config_from_mock(json!({
      "semverGroups": [{
        "packages": ["package-b"],
        "range": "<"
      }]
    }));
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": "~1.0.0"
        }
      }),
      json!({
        "name": "package-b",
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
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of package-a",
        actual: "~1.0.0",
        expected: Some("~1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::conflict(MismatchConflictsWithHighestOrLowestSemver),
        dependency_name: "foo",
        id: "foo in /dependencies of package-b",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
    ]);
  }
}

mod non_semver {
  use super::*;

  #[test]
  fn no_instances_are_semver_but_all_are_identical() {
    let config = test::mock::config();
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": "workspace:*"
        }
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "foo": "workspace:*"
        }
      }),
    ]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsNonSemverButIdentical),
        dependency_name: "foo",
        id: "foo in /dependencies of package-a",
        actual: "workspace:*",
        expected: Some("workspace:*"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsNonSemverButIdentical),
        dependency_name: "foo",
        id: "foo in /dependencies of package-b",
        actual: "workspace:*",
        expected: Some("workspace:*"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn no_instances_are_semver_and_they_differ() {
    let config = test::mock::config();
    let packages = test::mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "foo": "workspace:*"
        }
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "foo": "workspace:^"
        }
      }),
    ]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(NonSemverMismatch),
        dependency_name: "foo",
        id: "foo in /dependencies of package-a",
        actual: "workspace:*",
        expected: Some("workspace:*"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(NonSemverMismatch),
        dependency_name: "foo",
        id: "foo in /dependencies of package-b",
        actual: "workspace:^",
        expected: Some("workspace:^"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn workspace_protocol_version_differs_to_local_version_is_invalid_in_strict_mode() {
    let config = test::mock::config_from_mock(json!({
      "strict": true
    }));
    let packages = test::mock::packages_from_mocks(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "devDependencies": {
        "package-a": "workspace:*"
      }
    })]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
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
        state: InstanceState::fixable(DiffersToLocal),
        dependency_name: "package-a",
        id: "package-a in /devDependencies of package-a",
        actual: "workspace:*",
        expected: Some("1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn workspace_protocol_version_differs_to_local_version_is_valid_by_default() {
    let config = test::mock::config_from_mock(json!({}));
    let packages = test::mock::packages_from_mocks(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "devDependencies": {
        "package-a": "workspace:*"
      }
    })]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
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
        state: InstanceState::valid(SatisfiesLocal),
        dependency_name: "package-a",
        id: "package-a in /devDependencies of package-a",
        actual: "workspace:*",
        expected: Some("workspace:*"),
        overridden: None,
      },
    ]);
  }
}

mod dependency_groups {
  use super::*;

  #[test]
  fn dependency_group_of_dependency_and_its_types_that_can_be_relied_on_to_be_same_version() {
    let config = test::mock::config_from_mock(json!({
      "dependencyGroups": [{
        "aliasName": "foo-group",
        "dependencies": ["@types/foo", "foo"]
      }]
    }));
    let packages = test::mock::packages_from_mocks(vec![json!({
      "name": "package-a",
      "version": "0.1.0",
      "dependencies": {
        "foo": "4.1.0"
      },
      "devDependencies": {
        "@types/foo": "4.0.5"
      }
    })]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "0.1.0",
        expected: Some("0.1.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToHighestOrLowestSemver),
        dependency_name: "foo-group",
        id: "@types/foo in /devDependencies of package-a",
        actual: "4.0.5",
        expected: Some("4.1.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "foo-group",
        id: "foo in /dependencies of package-a",
        actual: "4.1.0",
        expected: Some("4.1.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn dependency_group_of_dependency_and_its_types_that_track_the_same_major_version() {
    let config = test::mock::config_from_mock(json!({
      "dependencyGroups": [{
        "aliasName": "foo-group",
        "dependencies": ["@types/foo", "foo"]
      }],
      "versionGroups": [{
        "dependencies": ["foo-group"],
        "policy": "sameRange"
      }]
    }));
    let packages = test::mock::packages_from_mocks(vec![json!({
      "name": "package-a",
      "version": "0.1.0",
      "dependencies": {
        "foo": "4.1.0"
      },
      "devDependencies": {
        "@types/foo": "^4.0.5"
      }
    })]);
    let registry_client = None;
    let ctx = Context::create(config, packages, registry_client);
    let ctx = visit_packages(ctx);
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "0.1.0",
        expected: Some("0.1.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesSameRangeGroup),
        dependency_name: "foo-group",
        id: "@types/foo in /devDependencies of package-a",
        actual: "^4.0.5",
        expected: Some("^4.0.5"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesSameRangeGroup),
        dependency_name: "foo-group",
        id: "foo in /dependencies of package-a",
        actual: "4.1.0",
        expected: Some("4.1.0"),
        overridden: None,
      },
    ]);
  }
}

mod registry_updates {
  use super::*;

  mod latest {
    use super::*;

    #[tokio::test]
    async fn reports_one_latest_exact_semver_update_in_one_file() {
      let config = test::mock::config();
      let packages = test::mock::packages_from_mocks(vec![json!({
        "name": "package-a",
        "dependencies": {
          "wat": "1.2.3"
        }
      })]);
      let ctx = test::mock::context_with_registry_updates(
        config,
        packages,
        json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }),
      )
      .await;
      let ctx = visit_packages(ctx);
      expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
          state: InstanceState::suspect(InvalidLocalVersion),
          dependency_name: "package-a",
          id: "package-a in /version of package-a",
          actual: "",
          expected: Some(""),
          overridden: None,
        },
        ExpectedInstance {
          state: InstanceState::fixable(DiffersToNpmRegistry),
          dependency_name: "wat",
          id: "wat in /dependencies of package-a",
          actual: "1.2.3",
          expected: Some("2.0.0"),
          overridden: None,
        },
      ]);
    }

    #[tokio::test]
    async fn reports_one_latest_update_with_loose_semver_range_in_one_file() {
      let config = test::mock::config();
      let packages = test::mock::packages_from_mocks(vec![json!({
        "name": "package-a",
        "dependencies": {
          "wat": "^1.2.3"
        }
      })]);
      let ctx = test::mock::context_with_registry_updates(
        config,
        packages,
        json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }),
      )
      .await;
      let ctx = visit_packages(ctx);
      expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
          state: InstanceState::suspect(InvalidLocalVersion),
          dependency_name: "package-a",
          id: "package-a in /version of package-a",
          actual: "",
          expected: Some(""),
          overridden: None,
        },
        ExpectedInstance {
          state: InstanceState::fixable(DiffersToNpmRegistry),
          dependency_name: "wat",
          id: "wat in /dependencies of package-a",
          actual: "^1.2.3",
          expected: Some("^2.0.0"),
          overridden: None,
        },
      ]);
    }

    #[tokio::test]
    async fn reports_one_latest_update_with_semver_range_mismatch_in_one_file() {
      let config = test::mock::config_from_mock(json!({
        "semverGroups": [{
          "dependencies": ["wat"],
          "range": "~"
        }]
      }));
      let packages = test::mock::packages_from_mocks(vec![json!({
        "name": "package-a",
        "dependencies": {
          "wat": "^1.2.3"
        }
      })]);
      let ctx = test::mock::context_with_registry_updates(
        config,
        packages,
        json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }),
      )
      .await;
      let ctx = visit_packages(ctx);
      expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
          state: InstanceState::suspect(InvalidLocalVersion),
          dependency_name: "package-a",
          id: "package-a in /version of package-a",
          actual: "",
          expected: Some(""),
          overridden: None,
        },
        ExpectedInstance {
          state: InstanceState::fixable(DiffersToNpmRegistry),
          dependency_name: "wat",
          id: "wat in /dependencies of package-a",
          actual: "^1.2.3",
          expected: Some("~2.0.0"),
          overridden: None,
        },
      ]);
    }
  }

  mod minor {
    use super::*;

    #[tokio::test]
    async fn reports_one_minor_exact_semver_update_in_one_file() {
      let mut config = test::mock::config();
      config.cli.target = UpdateTarget::Minor;
      let packages = test::mock::packages_from_mocks(vec![json!({
        "name": "package-a",
        "dependencies": {
          "wat": "1.2.3"
        }
      })]);
      let ctx = test::mock::context_with_registry_updates(
        config,
        packages,
        json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }),
      )
      .await;
      let ctx = visit_packages(ctx);
      expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
          state: InstanceState::suspect(InvalidLocalVersion),
          dependency_name: "package-a",
          id: "package-a in /version of package-a",
          actual: "",
          expected: Some(""),
          overridden: None,
        },
        ExpectedInstance {
          state: InstanceState::fixable(DiffersToNpmRegistry),
          dependency_name: "wat",
          id: "wat in /dependencies of package-a",
          actual: "1.2.3",
          expected: Some("1.3.4"),
          overridden: None,
        },
      ]);
    }

    #[tokio::test]
    async fn reports_one_minor_update_with_loose_semver_range_in_one_file() {
      let mut config = test::mock::config();
      config.cli.target = UpdateTarget::Minor;
      let packages = test::mock::packages_from_mocks(vec![json!({
        "name": "package-a",
        "dependencies": {
          "wat": "^1.2.3"
        }
      })]);
      let ctx = test::mock::context_with_registry_updates(
        config,
        packages,
        json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }),
      )
      .await;
      let ctx = visit_packages(ctx);
      expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
          state: InstanceState::suspect(InvalidLocalVersion),
          dependency_name: "package-a",
          id: "package-a in /version of package-a",
          actual: "",
          expected: Some(""),
          overridden: None,
        },
        ExpectedInstance {
          state: InstanceState::fixable(DiffersToNpmRegistry),
          dependency_name: "wat",
          id: "wat in /dependencies of package-a",
          actual: "^1.2.3",
          expected: Some("^1.3.4"),
          overridden: None,
        },
      ]);
    }

    #[tokio::test]
    async fn reports_one_minor_update_with_semver_range_mismatch_in_one_file() {
      let mut config = test::mock::config_from_mock(json!({
        "semverGroups": [{
          "dependencies": ["wat"],
          "range": "~"
        }]
      }));
      config.cli.target = UpdateTarget::Minor;
      let packages = test::mock::packages_from_mocks(vec![json!({
        "name": "package-a",
        "dependencies": {
          "wat": "^1.2.3"
        }
      })]);
      let ctx = test::mock::context_with_registry_updates(
        config,
        packages,
        json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }),
      )
      .await;
      let ctx = visit_packages(ctx);
      expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
          state: InstanceState::suspect(InvalidLocalVersion),
          dependency_name: "package-a",
          id: "package-a in /version of package-a",
          actual: "",
          expected: Some(""),
          overridden: None,
        },
        ExpectedInstance {
          state: InstanceState::fixable(DiffersToNpmRegistry),
          dependency_name: "wat",
          id: "wat in /dependencies of package-a",
          actual: "^1.2.3",
          expected: Some("~1.3.4"),
          overridden: None,
        },
      ]);
    }
  }

  mod patch {
    use super::*;

    #[tokio::test]
    async fn reports_one_patch_exact_semver_update_in_one_file() {
      let mut config = test::mock::config();
      config.cli.target = UpdateTarget::Patch;
      let packages = test::mock::packages_from_mocks(vec![json!({
        "name": "package-a",
        "dependencies": {
          "wat": "1.2.3"
        }
      })]);
      let ctx = test::mock::context_with_registry_updates(
        config,
        packages,
        json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }),
      )
      .await;
      let ctx = visit_packages(ctx);
      expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
          state: InstanceState::suspect(InvalidLocalVersion),
          dependency_name: "package-a",
          id: "package-a in /version of package-a",
          actual: "",
          expected: Some(""),
          overridden: None,
        },
        ExpectedInstance {
          state: InstanceState::fixable(DiffersToNpmRegistry),
          dependency_name: "wat",
          id: "wat in /dependencies of package-a",
          actual: "1.2.3",
          expected: Some("1.2.4"),
          overridden: None,
        },
      ]);
    }

    #[tokio::test]
    async fn reports_one_patch_update_with_loose_semver_range_in_one_file() {
      let mut config = test::mock::config();
      config.cli.target = UpdateTarget::Patch;
      let packages = test::mock::packages_from_mocks(vec![json!({
        "name": "package-a",
        "dependencies": {
          "wat": "^1.2.3"
        }
      })]);
      let ctx = test::mock::context_with_registry_updates(
        config,
        packages,
        json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }),
      )
      .await;
      let ctx = visit_packages(ctx);
      expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
          state: InstanceState::suspect(InvalidLocalVersion),
          dependency_name: "package-a",
          id: "package-a in /version of package-a",
          actual: "",
          expected: Some(""),
          overridden: None,
        },
        ExpectedInstance {
          state: InstanceState::fixable(DiffersToNpmRegistry),
          dependency_name: "wat",
          id: "wat in /dependencies of package-a",
          actual: "^1.2.3",
          expected: Some("^1.2.4"),
          overridden: None,
        },
      ]);
    }

    #[tokio::test]
    async fn reports_one_patch_update_with_semver_range_mismatch_in_one_file() {
      let mut config = test::mock::config_from_mock(json!({
        "semverGroups": [{
          "dependencies": ["wat"],
          "range": "~"
        }]
      }));
      config.cli.target = UpdateTarget::Patch;
      let packages = test::mock::packages_from_mocks(vec![json!({
        "name": "package-a",
        "dependencies": {
          "wat": "^1.2.3"
        }
      })]);
      let ctx = test::mock::context_with_registry_updates(
        config,
        packages,
        json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }),
      )
      .await;
      let ctx = visit_packages(ctx);
      expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
          state: InstanceState::suspect(InvalidLocalVersion),
          dependency_name: "package-a",
          id: "package-a in /version of package-a",
          actual: "",
          expected: Some(""),
          overridden: None,
        },
        ExpectedInstance {
          state: InstanceState::fixable(DiffersToNpmRegistry),
          dependency_name: "wat",
          id: "wat in /dependencies of package-a",
          actual: "^1.2.3",
          expected: Some("~1.2.4"),
          overridden: None,
        },
      ]);
    }
  }
}
