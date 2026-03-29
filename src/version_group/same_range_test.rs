use {
  crate::{
    instance::{FixableInstance::*, InstanceState, UnfixableInstance::*, ValidInstance::*},
    test::{
      builder::TestBuilder,
      expect::{expect, ExpectedInstance},
    },
    version_group::visit_groups,
  },
  serde_json::json,
};

#[test]
fn instance_in_a_same_range_group_satisfies_every_other_and_there_are_no_semver_groups() {
  let vg1 = json!({
    "dependencyTypes": ["local"],
    "isIgnored": true
  });
  let vg2 = json!({
    "dependencies": ["foo"],
    "policy": "sameRange"
  });
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
    .with_version_groups(vec![vg1.clone(), vg2.clone()])
    .build();
  visit_groups(&ctx, &[vg1, vg2]);
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
  let vg1 = json!({
    "dependencyTypes": ["local"],
    "isIgnored": true
  });
  let vg2 = json!({
    "dependencies": ["foo"],
    "policy": "sameRange"
  });
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
    .with_version_groups(vec![vg1.clone(), vg2.clone()])
    .build();
  visit_groups(&ctx, &[vg1, vg2]);
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
  let vg1 = json!({
    "dependencyTypes": ["local"],
    "isIgnored": true
  });
  let vg2 = json!({
    "dependencies": ["foo"],
    "policy": "sameRange"
  });
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
    .with_version_groups(vec![vg1.clone(), vg2.clone()])
    .build();
  visit_groups(&ctx, &[vg1, vg2]);
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
  let vg1 = json!({
    "dependencyTypes": ["local"],
    "isIgnored": true
  });
  let vg2 = json!({
    "dependencies": ["foo"],
    "policy": "sameRange"
  });
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
    .with_version_groups(vec![vg1.clone(), vg2.clone()])
    .build();
  visit_groups(&ctx, &[vg1, vg2]);
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

#[test]
fn caret_and_tilde_ranges_overlap_when_tilde_is_within_caret() {
  // ^1.0.0 covers 1.0.0-1.x.x, ~1.4.2 covers 1.4.2-1.4.x -- they overlap
  let vg1 = json!({
    "dependencyTypes": ["local"],
    "isIgnored": true
  });
  let vg2 = json!({
    "dependencies": ["foo"],
    "policy": "sameRange"
  });
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
    .with_version_groups(vec![vg1.clone(), vg2.clone()])
    .build();
  visit_groups(&ctx, &[vg1, vg2]);
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
      actual: "^1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "~1.4.2",
      expected: Some("~1.4.2"),
      overridden: None,
    },
  ]);
}

#[test]
fn tilde_ranges_do_not_overlap_when_minor_versions_differ() {
  // ~1.0.0 covers 1.0.x, ~1.4.2 covers 1.4.x -- no overlap
  let vg1 = json!({
    "dependencyTypes": ["local"],
    "isIgnored": true
  });
  let vg2 = json!({
    "dependencies": ["foo"],
    "policy": "sameRange"
  });
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
    .with_version_groups(vec![vg1.clone(), vg2.clone()])
    .build();
  visit_groups(&ctx, &[vg1, vg2]);
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
      actual: "~1.0.0",
      expected: Some("~1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "~1.4.2",
      expected: Some("~1.4.2"),
      overridden: None,
    },
  ]);
}

#[test]
fn exact_pinned_versions_that_differ_do_not_overlap() {
  // 1.0.0 and 1.0.1 are each a single point -- they don't intersect
  let vg1 = json!({
    "dependencyTypes": ["local"],
    "isIgnored": true
  });
  let vg2 = json!({
    "dependencies": ["foo"],
    "policy": "sameRange"
  });
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
    .with_version_groups(vec![vg1.clone(), vg2.clone()])
    .build();
  visit_groups(&ctx, &[vg1, vg2]);
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
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "1.0.1",
      expected: Some("1.0.1"),
      overridden: None,
    },
  ]);
}

#[test]
fn three_ranges_all_overlap_pairwise() {
  // >=1.0.0, ^1.2.3, <=2.0.0 -- all three pairwise intersect
  let vg1 = json!({
    "dependencyTypes": ["local"],
    "isIgnored": true
  });
  let vg2 = json!({
    "dependencies": ["foo"],
    "policy": "sameRange"
  });
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
    .with_version_groups(vec![vg1.clone(), vg2.clone()])
    .build();
  visit_groups(&ctx, &[vg1, vg2]);
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
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-c",
      id: "package-c in /version of package-c",
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
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-c",
      actual: "<=2.0.0",
      expected: Some("<=2.0.0"),
      overridden: None,
    },
  ]);
}

#[test]
fn three_ranges_where_one_does_not_overlap_with_others() {
  // ^1.0.0 and ~1.2.0 overlap, but <1.0.0 overlaps with neither
  let vg1 = json!({
    "dependencyTypes": ["local"],
    "isIgnored": true
  });
  let vg2 = json!({
    "dependencies": ["foo"],
    "policy": "sameRange"
  });
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
    .with_version_groups(vec![vg1.clone(), vg2.clone()])
    .build();
  visit_groups(&ctx, &[vg1, vg2]);
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
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-c",
      id: "package-c in /version of package-c",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: "^1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "~1.2.0",
      expected: Some("~1.2.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::unfixable(SameRangeMismatch),
      dependency_name: "foo",
      id: "foo in /dependencies of package-c",
      actual: "<1.0.0",
      expected: Some("<1.0.0"),
      overridden: None,
    },
  ]);
}
