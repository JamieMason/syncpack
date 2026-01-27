use {
  crate::{
    cli::UpdateTarget,
    context::Context,
    instance_state::{
      FixableInstance::*, InstanceState, SemverGroupAndVersionConflict::*, SuspectInstance::*, UnfixableInstance::*, ValidInstance::*,
    },
    test::{
      builder::TestBuilder,
      expect::{expect, ExpectedInstance},
      mock,
    },
    visit_packages,
  },
  serde_json::json,
};

mod local {
  use super::*;

  #[test]
  fn instance_depends_on_local_version_which_is_missing() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({"name": "package-a"}),
        json!({
          "name": "package-b",
          "devDependencies": {
            "package-a": "0.1.0"
          }
        }),
      ])
      .build_and_visit_packages();
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
    let ctx = TestBuilder::new()
      .with_packages(vec![
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
      ])
      .build_and_visit_packages();
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
      .build_and_visit_packages();
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
    let ctx = TestBuilder::new()
      .with_packages(vec![
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
      ])
      .build_and_visit_packages();
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
      .build_and_visit_packages();
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
    let ctx = TestBuilder::new()
      .with_packages(vec![
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
      ])
      .build_and_visit_packages();
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
    let ctx = TestBuilder::new()
      .with_packages(vec![
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
      ])
      .with_semver_group(json!({
        "range": "^"
      }))
      .build_and_visit_packages();
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
  fn instance_is_linked_to_local_package() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({
          "name": "package-a",
          "version": "1.0.0"
        }),
        json!({
          "name": "package-b",
          "version": "2.0.0",
          "dependencies": {
            "package-a": "link:../package-a"
          }
        }),
      ])
      .build_and_visit_packages();
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "2.0.0",
        expected: Some("2.0.0"),
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
        actual: "link:../package-a",
        expected: Some("link:../package-a"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_is_linked_to_wrong_directory() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({
          "name": "package-a",
          "version": "1.0.0"
        }),
        json!({
          "name": "package-b",
          "version": "2.0.0",
          "dependencies": {
            "package-a": "link:../../elsewhere/package-a"
          }
        }),
      ])
      .build_and_visit_packages();
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "2.0.0",
        expected: Some("2.0.0"),
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
        actual: "link:../../elsewhere/package-a",
        expected: Some("1.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn instance_has_same_version_number_as_local_package_but_mismatches_a_different_but_compatible_semver_group() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
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
      ])
      .with_semver_group(json!({
        "range": "^"
      }))
      .build_and_visit_packages();
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
    let ctx = TestBuilder::new()
      .with_packages(vec![
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
      ])
      .with_semver_group(json!({
        "range": "<"
      }))
      .build_and_visit_packages();
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
    let ctx = TestBuilder::new()
      .with_packages(vec![
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
      ])
      .with_semver_group(json!({
        "range": ">"
      }))
      .build_and_visit_packages();
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
    let ctx = TestBuilder::new()
      .with_package(json!({
        "name": "package-a",
        "dependencies": {
          "wat": "1.0.0"
        },
        "devDependencies": {
          "wat": "2.0.0"
        }
      }))
      .build_and_visit_packages();
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
    let ctx = TestBuilder::new()
      .with_package(json!({
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
      }))
      .build_and_visit_packages();
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
    let ctx = TestBuilder::new()
      .with_packages(vec![
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
      ])
      .build_and_visit_packages();
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
    let ctx = TestBuilder::new()
      .with_packages(vec![
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
      ])
      .with_version_groups(vec![json!({"packages": ["package-a"]}), json!({"packages": ["package-b"]})])
      .build_and_visit_packages();
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
    let ctx = TestBuilder::new()
      .with_packages(vec![
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
      ])
      .build_and_visit_packages();
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
            "foo": "1.0.0"
          }
        }),
      ])
      .build_and_visit_packages();
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
    let config = mock::config();
    let packages = mock::packages_from_mocks(vec![
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
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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
    let config = mock::config();
    let packages = mock::packages_from_mocks(vec![
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
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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
    let config = mock::config_from_mock(json!({
      "semverGroups": [{
        "packages": ["package-b"],
        "range": "~"
      }]
    }));
    let packages = mock::packages_from_mocks(vec![
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
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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
    let config = mock::config_from_mock(json!({
      "semverGroups": [{
        "packages": ["package-b"],
        "range": "^"
      }]
    }));
    let packages = mock::packages_from_mocks(vec![
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
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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
    let config = mock::config_from_mock(json!({
      "semverGroups": [{
        "packages": ["package-b"],
        "range": "<"
      }]
    }));
    let packages = mock::packages_from_mocks(vec![
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
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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
    let config = mock::config_from_mock(json!({
      "semverGroups": [{
        "packages": ["package-b"],
        "range": "<"
      }]
    }));
    let packages = mock::packages_from_mocks(vec![
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
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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

  #[test]
  fn highest_semver_with_prerelease_versions_prefers_most_greedy() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({
          "name": "pkg-a",
          "devDependencies": {
            "@typescript/native-preview": "7.0.0-dev.20260117.1"
          }
        }),
        json!({
          "name": "pkg-b",
          "devDependencies": {
            "@typescript/native-preview": "^7.0.0-dev.20260117.1"
          }
        }),
      ])
      .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "pkg-a",
        id: "pkg-a in /version of pkg-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "pkg-b",
        id: "pkg-b in /version of pkg-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToHighestOrLowestSemver),
        dependency_name: "@typescript/native-preview",
        id: "@typescript/native-preview in /devDependencies of pkg-a",
        actual: "7.0.0-dev.20260117.1",
        expected: Some("^7.0.0-dev.20260117.1"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "@typescript/native-preview",
        id: "@typescript/native-preview in /devDependencies of pkg-b",
        actual: "^7.0.0-dev.20260117.1",
        expected: Some("^7.0.0-dev.20260117.1"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn lowest_semver_prefers_least_greedy_range_when_versions_equal() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({
          "name": "pkg-a",
          "devDependencies": {
            "react": "^18.0.0"
          }
        }),
        json!({
          "name": "pkg-b",
          "devDependencies": {
            "react": "18.0.0"
          }
        }),
        json!({
          "name": "pkg-c",
          "devDependencies": {
            "react": "~18.0.0"
          }
        }),
      ])
      .with_version_group(json!({
        "preferVersion": "lowestSemver"
      }))
      .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "pkg-a",
        id: "pkg-a in /version of pkg-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToHighestOrLowestSemver),
        dependency_name: "react",
        id: "react in /devDependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("18.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "pkg-b",
        id: "pkg-b in /version of pkg-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "react",
        id: "react in /devDependencies of pkg-b",
        actual: "18.0.0",
        expected: Some("18.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "pkg-c",
        id: "pkg-c in /version of pkg-c",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToHighestOrLowestSemver),
        dependency_name: "react",
        id: "react in /devDependencies of pkg-c",
        actual: "~18.0.0",
        expected: Some("18.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn regression_test_for_issue_314() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({
          "name": "pkg-a",
          "devDependencies": {
            "lodash": "4.17.21",
            "@typescript/native-preview": "7.0.0-dev.20260117.1"
          }
        }),
        json!({
          "name": "pkg-b",
          "devDependencies": {
            "lodash": "^4.17.21",
            "@typescript/native-preview": "^7.0.0-dev.20260117.1"
          }
        }),
      ])
      .with_semver_group(json!({
        "label": "pin all ranges",
        "range": "",
        "packages": ["**"],
        "dependencies": ["**"]
      }))
      .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "pkg-a",
        id: "pkg-a in /version of pkg-a",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesHighestOrLowestSemver),
        dependency_name: "@typescript/native-preview",
        id: "@typescript/native-preview in /devDependencies of pkg-a",
        actual: "7.0.0-dev.20260117.1",
        expected: Some("7.0.0-dev.20260117.1"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(SatisfiesHighestOrLowestSemver),
        dependency_name: "lodash",
        id: "lodash in /devDependencies of pkg-a",
        actual: "4.17.21",
        expected: Some("4.17.21"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "pkg-b",
        id: "pkg-b in /version of pkg-b",
        actual: "",
        expected: Some(""),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(SemverRangeMismatch),
        dependency_name: "@typescript/native-preview",
        id: "@typescript/native-preview in /devDependencies of pkg-b",
        actual: "^7.0.0-dev.20260117.1",
        expected: Some("7.0.0-dev.20260117.1"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(SemverRangeMismatch),
        dependency_name: "lodash",
        id: "lodash in /devDependencies of pkg-b",
        actual: "^4.17.21",
        expected: Some("4.17.21"),
        overridden: None,
      },
    ]);
  }
}

mod non_semver {
  use super::*;

  #[test]
  fn no_instances_are_semver_but_all_are_identical() {
    let config = mock::config();
    let packages = mock::packages_from_mocks(vec![
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
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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
    let config = mock::config();
    let packages = mock::packages_from_mocks(vec![
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
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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
    let config = mock::config_from_mock(json!({
      "strict": true
    }));
    let packages = mock::packages_from_mocks(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "devDependencies": {
        "package-a": "workspace:*"
      }
    })]);
    let registry_client = None;
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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
    let config = mock::config_from_mock(json!({}));
    let packages = mock::packages_from_mocks(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "devDependencies": {
        "package-a": "workspace:*"
      }
    })]);
    let registry_client = None;
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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

  #[test]
  fn link_specifiers_without_local_instance_are_identical() {
    let config = mock::config();
    let packages = mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "external-package": "link:../../external/external-package"
        }
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "external-package": "link:../../external/external-package"
        }
      }),
    ]);
    let registry_client = None;
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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
        dependency_name: "external-package",
        id: "external-package in /dependencies of package-a",
        actual: "link:../../external/external-package",
        expected: Some("link:../../external/external-package"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsNonSemverButIdentical),
        dependency_name: "external-package",
        id: "external-package in /dependencies of package-b",
        actual: "link:../../external/external-package",
        expected: Some("link:../../external/external-package"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn link_specifiers_without_local_instance_differ() {
    let config = mock::config();
    let packages = mock::packages_from_mocks(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "external-package": "link:../../external/external-package"
        }
      }),
      json!({
        "name": "package-b",
        "dependencies": {
          "external-package": "link:../../elsewhere/external-package"
        }
      }),
    ]);
    let registry_client = None;
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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
        dependency_name: "external-package",
        id: "external-package in /dependencies of package-a",
        actual: "link:../../external/external-package",
        expected: Some("link:../../external/external-package"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(NonSemverMismatch),
        dependency_name: "external-package",
        id: "external-package in /dependencies of package-b",
        actual: "link:../../elsewhere/external-package",
        expected: Some("link:../../elsewhere/external-package"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn non_semver_and_semver_mismatch() {
    let config = mock::config_from_mock(json!({}));
    let packages = mock::packages_from_mocks(vec![json!({
      "name": "package-a",
      "dependencies": {
        "mix": "1.2.3"
      },
      "devDependencies": {
        "mix": "file:./mix"
      }
    })]);
    let registry_client = None;
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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
        dependency_name: "mix",
        id: "mix in /dependencies of package-a",
        actual: "1.2.3",
        expected: Some("1.2.3"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToHighestOrLowestSemver),
        dependency_name: "mix",
        id: "mix in /devDependencies of package-a",
        actual: "file:./mix",
        expected: Some("1.2.3"),
        overridden: None,
      },
    ]);
  }
}

mod dependency_groups {
  use super::*;

  #[test]
  fn dependency_group_of_dependency_and_its_types_that_can_be_relied_on_to_be_same_version() {
    let config = mock::config_from_mock(json!({
      "dependencyGroups": [{
        "aliasName": "foo-group",
        "dependencies": ["@types/foo", "foo"]
      }]
    }));
    let packages = mock::packages_from_mocks(vec![json!({
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
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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
    let config = mock::config_from_mock(json!({
      "dependencyGroups": [{
        "aliasName": "foo-group",
        "dependencies": ["@types/foo", "foo"]
      }],
      "versionGroups": [{
        "dependencies": ["foo-group"],
        "policy": "sameRange"
      }]
    }));
    let packages = mock::packages_from_mocks(vec![json!({
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
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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
      let ctx = TestBuilder::new()
        .with_package(json!({
          "name": "package-a",
          "dependencies": {
            "wat": "1.2.3"
          }
        }))
        .with_registry_updates(json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }))
        .build_with_registry_and_visit()
        .await;
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
      let ctx = TestBuilder::new()
        .with_package(json!({
          "name": "package-a",
          "dependencies": {
            "wat": "^1.2.3"
          }
        }))
        .with_registry_updates(json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }))
        .build_with_registry_and_visit()
        .await;
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
      let ctx = TestBuilder::new()
        .with_package(json!({
          "name": "package-a",
          "dependencies": {
            "wat": "^1.2.3"
          }
        }))
        .with_semver_group(json!({
          "dependencies": ["wat"],
          "range": "~"
        }))
        .with_registry_updates(json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }))
        .build_with_registry_and_visit()
        .await;
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

    #[tokio::test]
    async fn reports_latest_exact_semver_updates_for_npm_aliases() {
      let ctx = TestBuilder::new()
        .with_package(json!({
          "name": "package-a",
          "dependencies": {
            // JSR dependencies (https://jsr.io/docs/npm-compatibility)
            "@jsr/luca__cases": "1.2.3",
            "@luca/cases": "npm:@jsr/luca__cases@1.2.3",
            "@std/fmt": "npm:@jsr/std__fmt@1.2.3",
            // normal npm dependencies, but aliased
            "@lit-labs/ssr": "npm:@lit-labs/ssr@1.2.3",
            "lit": "npm:lit@1.2.3",
          }
        }))
        .with_registry_updates(json!({
          "@jsr/luca__cases": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
          "@lit-labs/ssr": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
          "@luca/cases": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
          "@std/fmt": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
          "lit": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }))
        .build_with_registry_and_visit()
        .await;
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
          dependency_name: "@jsr/luca__cases",
          id: "@jsr/luca__cases in /dependencies of package-a",
          actual: "1.2.3",
          expected: Some("2.0.0"),
          overridden: None,
        },
        ExpectedInstance {
          state: InstanceState::fixable(DiffersToNpmRegistry),
          dependency_name: "@lit-labs/ssr",
          id: "@lit-labs/ssr in /dependencies of package-a",
          actual: "npm:@lit-labs/ssr@1.2.3",
          expected: Some("npm:@lit-labs/ssr@2.0.0"),
          overridden: None,
        },
        ExpectedInstance {
          state: InstanceState::fixable(DiffersToNpmRegistry),
          dependency_name: "@luca/cases",
          id: "@luca/cases in /dependencies of package-a",
          actual: "npm:@jsr/luca__cases@1.2.3",
          expected: Some("npm:@jsr/luca__cases@2.0.0"),
          overridden: None,
        },
        ExpectedInstance {
          state: InstanceState::fixable(DiffersToNpmRegistry),
          dependency_name: "@std/fmt",
          id: "@std/fmt in /dependencies of package-a",
          actual: "npm:@jsr/std__fmt@1.2.3",
          expected: Some("npm:@jsr/std__fmt@2.0.0"),
          overridden: None,
        },
        ExpectedInstance {
          state: InstanceState::fixable(DiffersToNpmRegistry),
          dependency_name: "lit",
          id: "lit in /dependencies of package-a",
          actual: "npm:lit@1.2.3",
          expected: Some("npm:lit@2.0.0"),
          overridden: None,
        },
      ]);
    }
  }

  mod minor {
    use super::*;

    #[tokio::test]
    async fn reports_one_minor_exact_semver_update_in_one_file() {
      let ctx = TestBuilder::new()
        .with_package(json!({
          "name": "package-a",
          "dependencies": {
            "wat": "1.2.3"
          }
        }))
        .with_update_target(UpdateTarget::Minor)
        .with_registry_updates(json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }))
        .build_with_registry_and_visit()
        .await;
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
    async fn reports_two_minor_exact_semver_updates_in_one_file() {
      let ctx = TestBuilder::new()
        .with_package(json!({
          "name": "package-a",
          "dependencies": {
            "wat": "1.0.1"
          },
          "devDependencies": {
            "wat": "2.0.0"
          }
        }))
        .with_update_target(UpdateTarget::Minor)
        .with_registry_updates(json!({
          "wat": ["1.0.1", "1.0.2", "1.1.2", "1.3.3", "2.0.0", "2.2.2"],
        }))
        .build_with_registry_and_visit()
        .await;
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
          actual: "1.0.1",
          expected: Some("1.3.3"),
          overridden: None,
        },
        ExpectedInstance {
          state: InstanceState::fixable(DiffersToNpmRegistry),
          dependency_name: "wat",
          id: "wat in /devDependencies of package-a",
          actual: "2.0.0",
          expected: Some("2.2.2"),
          overridden: None,
        },
      ]);
    }

    #[tokio::test]
    async fn reports_one_minor_update_with_loose_semver_range_in_one_file() {
      let ctx = TestBuilder::new()
        .with_package(json!({
          "name": "package-a",
          "dependencies": {
            "wat": "^1.2.3"
          }
        }))
        .with_update_target(UpdateTarget::Minor)
        .with_registry_updates(json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }))
        .build_with_registry_and_visit()
        .await;
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
      let ctx = TestBuilder::new()
        .with_package(json!({
          "name": "package-a",
          "dependencies": {
            "wat": "^1.2.3"
          }
        }))
        .with_semver_group(json!({
          "dependencies": ["wat"],
          "range": "~"
        }))
        .with_update_target(UpdateTarget::Minor)
        .with_registry_updates(json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }))
        .build_with_registry_and_visit()
        .await;
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
      let ctx = TestBuilder::new()
        .with_package(json!({
          "name": "package-a",
          "dependencies": {
            "wat": "1.2.3"
          }
        }))
        .with_update_target(UpdateTarget::Patch)
        .with_registry_updates(json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }))
        .build_with_registry_and_visit()
        .await;
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
    async fn reports_two_patch_exact_semver_updates_in_one_file() {
      let ctx = TestBuilder::new()
        .with_package(json!({
          "name": "package-a",
          "dependencies": {
            "wat": "1.0.1"
          },
          "devDependencies": {
            "wat": "1.1.1"
          }
        }))
        .with_update_target(UpdateTarget::Patch)
        .with_registry_updates(json!({
          "wat": ["1.0.1", "1.0.2", "1.1.1", "1.1.2", "1.3.3", "2.0.0"],
        }))
        .build_with_registry_and_visit()
        .await;
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
          actual: "1.0.1",
          expected: Some("1.0.2"),
          overridden: None,
        },
        ExpectedInstance {
          state: InstanceState::fixable(DiffersToNpmRegistry),
          dependency_name: "wat",
          id: "wat in /devDependencies of package-a",
          actual: "1.1.1",
          expected: Some("1.1.2"),
          overridden: None,
        },
      ]);
    }

    #[tokio::test]
    async fn reports_one_patch_update_with_loose_semver_range_in_one_file() {
      let ctx = TestBuilder::new()
        .with_package(json!({
          "name": "package-a",
          "dependencies": {
            "wat": "^1.2.3"
          }
        }))
        .with_update_target(UpdateTarget::Patch)
        .with_registry_updates(json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }))
        .build_with_registry_and_visit()
        .await;
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
      let ctx = TestBuilder::new()
        .with_package(json!({
          "name": "package-a",
          "dependencies": {
            "wat": "^1.2.3"
          }
        }))
        .with_semver_group(json!({
          "dependencies": ["wat"],
          "range": "~"
        }))
        .with_update_target(UpdateTarget::Patch)
        .with_registry_updates(json!({
          "wat": ["1.2.2", "1.2.3", "1.2.4", "1.3.4", "2.0.0"],
        }))
        .build_with_registry_and_visit()
        .await;
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

mod custom_types {
  use super::*;

  #[test]
  fn custom_engines_type_visible_in_catch_all_group_without_cli_filter() {
    let ctx = TestBuilder::new()
      .with_config(json!({
        "customTypes": {
          "engines": {
            "strategy": "versionsByName",
            "path": "engines"
          }
        },
        "versionGroups": [
          {
            "dependencies": ["react"],
            "dependencyTypes": ["prod", "dev", "peer"]
          }
        ]
      }))
      .with_packages(vec![json!({
        "name": "package-a",
        "version": "1.0.0",
        "dependencies": {
          "react": "18.0.0"
        },
        "engines": {
          "node": ">=16.0.0",
          "yarn": "^1.22.5"
        }
      })])
      .build_and_visit_packages();

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
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "react",
        id: "react in /dependencies of package-a",
        actual: "18.0.0",
        expected: Some("18.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "node",
        id: "node in /engines of package-a",
        actual: ">=16.0.0",
        expected: Some(">=16.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsHighestOrLowestSemver),
        dependency_name: "yarn",
        id: "yarn in /engines of package-a",
        actual: "^1.22.5",
        expected: Some("^1.22.5"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn exact_version_with_equals_prefix_should_be_fixable_when_semver_group_requires_caret() {
    // Reproduces issue #239
    // User has semverGroups with range "^" but dependency uses =9.0.0 (npm's equals prefix)
    // This should be marked as fixable (needs ^ prefix added)
    let ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "pkg-a",
        "version": "1.0.0",
        "dependencies": {
          "react": "=9.0.0"
        }
      })])
      .with_config(json!({
        "semverGroups": [{
          "range": "^"
        }]
      }))
      .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "pkg-a",
        id: "pkg-a in /version of pkg-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(SemverRangeMismatch),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "=9.0.0",
        expected: Some("^9.0.0"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn exact_version_without_prefix_should_be_fixable_when_semver_group_requires_caret() {
    // Also test plain exact versions like "9.0.0" (without =)
    // These should also be flagged when semver group requires ^
    let ctx = TestBuilder::new()
      .with_packages(vec![json!({
        "name": "pkg-a",
        "version": "1.0.0",
        "dependencies": {
          "react": "9.0.0"
        }
      })])
      .with_config(json!({
        "semverGroups": [{
          "range": "^"
        }]
      }))
      .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "pkg-a",
        id: "pkg-a in /version of pkg-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(SemverRangeMismatch),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "9.0.0",
        expected: Some("^9.0.0"),
        overridden: None,
      },
    ]);
  }
}

mod catalogs {
  use super::*;

  #[test]
  fn catalog_and_semver_mismatch() {
    let config = mock::config_from_mock(json!({}));
    let packages = mock::packages_from_mocks(vec![json!({
      "name": "package-a",
      "dependencies": {
        "mix": "1.2.3"
      },
      "devDependencies": {
        "mix": "catalog:"
      }
    })]);
    let registry_client = None;
    let catalogs = None;
    let ctx = Context::create(config, packages, registry_client, catalogs);
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
        state: InstanceState::fixable(DiffersToCatalog),
        dependency_name: "mix",
        id: "mix in /dependencies of package-a",
        actual: "1.2.3",
        expected: Some("catalog:"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalog),
        dependency_name: "mix",
        id: "mix in /devDependencies of package-a",
        actual: "catalog:",
        expected: Some("catalog:"),
        overridden: None,
      },
    ]);
  }
}
