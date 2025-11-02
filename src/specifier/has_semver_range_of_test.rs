use crate::{semver_range::SemverRange, specifier::Specifier};

#[test]
fn exact_variants_have_exact_range() {
  let cases: Vec<&str> = vec![
    // Exact (Patch)
    "1.2.3",
    "0.0.0",
    "999.999.999",
    "1.2.3-alpha.1",
    "1.2.3-beta.2",
    // Major
    "1",
    "0",
    "999",
    // Minor
    "1.2",
    "0.0",
    "999.999",
  ];
  for value in cases {
    assert!(
      Specifier::new(value).has_semver_range_of(&SemverRange::Exact),
      "Expected {value} to have SemverRange::Exact"
    );
  }
}

#[test]
fn exact_variants_do_not_have_other_ranges() {
  let cases: Vec<&str> = vec!["1.2.3", "1", "1.2", "0.0.0", "1.2.3-alpha"];
  let ranges: Vec<SemverRange> = vec![
    SemverRange::Minor,
    SemverRange::Patch,
    SemverRange::Any,
    SemverRange::Gt,
    SemverRange::Gte,
    SemverRange::Lt,
    SemverRange::Lte,
  ];
  for value in &cases {
    for range in &ranges {
      assert!(
        !Specifier::new(value).has_semver_range_of(range),
        "Expected {value} to NOT have {range:?}"
      );
    }
  }
}

#[test]
fn caret_ranges() {
  let cases: Vec<&str> = vec![
    // Range (Patch)
    "^1.2.3",
    "^0.0.0",
    "^999.999.999",
    "^1.2.3-alpha.1",
    // RangeMinor
    "^1.2",
    "^0.0",
    // RangeMajor
    "^1",
    "^0",
    "^999",
  ];
  for value in cases {
    assert!(
      Specifier::new(value).has_semver_range_of(&SemverRange::Minor),
      "Expected {value} to have SemverRange::Minor"
    );
  }
}

#[test]
fn tilde_ranges() {
  let cases: Vec<&str> = vec![
    // Range (Patch)
    "~1.2.3",
    "~0.0.0",
    "~999.999.999",
    "~1.2.3-beta.1",
    // RangeMinor
    "~1.2",
    "~0.0",
    // RangeMajor
    "~1",
    "~0",
  ];
  for value in cases {
    assert!(
      Specifier::new(value).has_semver_range_of(&SemverRange::Patch),
      "Expected {value} to have SemverRange::Patch"
    );
  }
}

#[test]
fn gt_ranges() {
  let cases: Vec<&str> = vec![
    // Range (Patch)
    ">1.2.3",
    ">0.0.0",
    ">1.2.3-alpha",
    // RangeMinor
    ">1.2",
    // RangeMajor
    ">1",
    ">0",
  ];
  for value in cases {
    assert!(
      Specifier::new(value).has_semver_range_of(&SemverRange::Gt),
      "Expected {value} to have SemverRange::Gt"
    );
  }
}

#[test]
fn gte_ranges() {
  let cases: Vec<&str> = vec![
    // Range (Patch)
    ">=1.2.3",
    ">=0.0.0",
    ">=1.2.3-rc.1",
    // RangeMinor
    ">=1.2",
    // RangeMajor
    ">=1",
    ">=0",
  ];
  for value in cases {
    assert!(
      Specifier::new(value).has_semver_range_of(&SemverRange::Gte),
      "Expected {value} to have SemverRange::Gte"
    );
  }
}

#[test]
fn lt_ranges() {
  let cases: Vec<&str> = vec![
    // Range (Patch)
    "<1.2.3",
    "<0.0.0",
    "<1.2.3-next",
    // RangeMinor
    "<1.2",
    // RangeMajor
    "<1",
    "<999",
  ];
  for value in cases {
    assert!(
      Specifier::new(value).has_semver_range_of(&SemverRange::Lt),
      "Expected {value} to have SemverRange::Lt"
    );
  }
}

#[test]
fn lte_ranges() {
  let cases: Vec<&str> = vec![
    // Range (Patch)
    "<=1.2.3",
    "<=0.0.0",
    "<=1.2.3-canary",
    // RangeMinor
    "<=1.2",
    // RangeMajor
    "<=1",
    "<=0",
  ];
  for value in cases {
    assert!(
      Specifier::new(value).has_semver_range_of(&SemverRange::Lte),
      "Expected {value} to have SemverRange::Lte"
    );
  }
}

#[test]
fn any_range() {
  let cases: Vec<&str> = vec!["*"];
  for value in cases {
    assert!(
      Specifier::new(value).has_semver_range_of(&SemverRange::Any),
      "Expected {value} to have SemverRange::Any"
    );
  }
}

#[test]
fn with_workspace_protocol() {
  let cases: Vec<(&str, SemverRange)> = vec![
    // Exact
    ("workspace:1.2.3", SemverRange::Exact),
    ("workspace:1", SemverRange::Exact),
    ("workspace:1.2", SemverRange::Exact),
    // Ranges
    ("workspace:^1.2.3", SemverRange::Minor),
    ("workspace:^1.2", SemverRange::Minor),
    ("workspace:^1", SemverRange::Minor),
    ("workspace:~1.2.3", SemverRange::Patch),
    ("workspace:~1.2", SemverRange::Patch),
    ("workspace:~1", SemverRange::Patch),
    ("workspace:>=1.2.3", SemverRange::Gte),
    ("workspace:>=1.2", SemverRange::Gte),
    ("workspace:>=1", SemverRange::Gte),
    ("workspace:>1.2.3", SemverRange::Gt),
    ("workspace:<1.2.3", SemverRange::Lt),
    ("workspace:<=1.2.3", SemverRange::Lte),
    // Range-only (no version)
    ("workspace:*", SemverRange::Any),
    ("workspace:^", SemverRange::Minor),
    ("workspace:~", SemverRange::Patch),
  ];
  for (value, expected_range) in cases {
    assert!(
      Specifier::new(value).has_semver_range_of(&expected_range),
      "Expected {value} to have {expected_range:?}"
    );
  }
}

#[test]
fn with_npm_alias() {
  let cases: Vec<(&str, SemverRange)> = vec![
    // Exact
    ("npm:foo@1.2.3", SemverRange::Exact),
    ("npm:@scope/pkg@1.2.3", SemverRange::Exact),
    ("npm:foo@1", SemverRange::Exact),
    ("npm:foo@1.2", SemverRange::Exact),
    // Ranges
    ("npm:foo@^1.2.3", SemverRange::Minor),
    ("npm:foo@^1.2", SemverRange::Minor),
    ("npm:foo@^1", SemverRange::Minor),
    ("npm:@scope/pkg@~1.2.3", SemverRange::Patch),
    ("npm:foo@~1.2", SemverRange::Patch),
    ("npm:foo@~1", SemverRange::Patch),
    ("npm:foo@>=1.2.3", SemverRange::Gte),
    ("npm:foo@>1.2.3", SemverRange::Gt),
    ("npm:foo@<1.2.3", SemverRange::Lt),
    ("npm:foo@<=1.2.3", SemverRange::Lte),
  ];
  for (value, expected_range) in cases {
    assert!(
      Specifier::new(value).has_semver_range_of(&expected_range),
      "Expected {value} to have {expected_range:?}"
    );
  }
}

#[test]
fn with_git_urls() {
  let cases: Vec<(&str, SemverRange)> = vec![
    // Exact versions
    ("git@github.com:npm/cli.git#1.2.3", SemverRange::Exact),
    ("git@github.com:npm/cli.git#1", SemverRange::Exact),
    ("git@github.com:npm/cli.git#1.2", SemverRange::Exact),
    ("github:user/repo#1.2.3", SemverRange::Exact),
    ("git+ssh://git@github.com/npm/cli#1.2.3", SemverRange::Exact),
    // Ranges with semver: prefix
    ("git@github.com:npm/cli.git#semver:^1.2.3", SemverRange::Minor),
    ("git@github.com:npm/cli.git#semver:^1.2", SemverRange::Minor),
    ("git@github.com:npm/cli.git#semver:^1", SemverRange::Minor),
    ("github:user/repo#semver:~1.2.3", SemverRange::Patch),
    ("git+ssh://git@github.com/npm/cli#semver:~1.2", SemverRange::Patch),
    ("git@github.com:npm/cli.git#semver:>=1.2.3", SemverRange::Gte),
    ("github:user/repo#semver:>1.2.3", SemverRange::Gt),
    ("git@github.com:npm/cli.git#semver:<1.2.3", SemverRange::Lt),
    ("git+ssh://git@github.com/npm/cli#semver:<=1.2.3", SemverRange::Lte),
  ];
  for (value, expected_range) in cases {
    assert!(
      Specifier::new(value).has_semver_range_of(&expected_range),
      "Expected {value} to have {expected_range:?}"
    );
  }
}

#[test]
fn specifiers_without_semver_range() {
  let cases: Vec<&str> = vec![
    // Tags
    "latest",
    "next",
    "canary",
    "beta",
    // URLs
    "https://example.com/package.tgz",
    "http://example.com/package.tgz",
    // File protocols
    "file:../foo",
    "file:./local",
    // Complex semver ranges
    ">=1.2.3 <2.0.0",
    "^1.2.3 || ^2.0.0",
    "1.x",
    "1.2.x",
    // Git without semver tags
    "git@github.com:npm/cli.git",
    "git@github.com:npm/cli.git#main",
    "git@github.com:npm/cli.git#HEAD",
    "git+ssh://git@github.com/npm/cli#feature-branch",
    "github:user/repo",
    "github:user/repo#develop",
  ];
  let ranges: Vec<SemverRange> = vec![
    SemverRange::Exact,
    SemverRange::Minor,
    SemverRange::Patch,
    SemverRange::Any,
    SemverRange::Gt,
    SemverRange::Gte,
    SemverRange::Lt,
    SemverRange::Lte,
  ];
  for value in &cases {
    for range in &ranges {
      assert!(
        !Specifier::new(value).has_semver_range_of(range),
        "Expected {value} to return false for {range:?} (no semver_range)"
      );
    }
  }
}

#[test]
fn wrong_range_returns_false() {
  let cases: Vec<(&str, SemverRange)> = vec![
    // Exact but checking for range
    ("1.2.3", SemverRange::Minor),
    ("1.2.3", SemverRange::Patch),
    ("1", SemverRange::Minor),
    ("1.2", SemverRange::Patch),
    // Caret but checking for other ranges
    ("^1.2.3", SemverRange::Exact),
    ("^1.2.3", SemverRange::Patch),
    ("^1.2.3", SemverRange::Gte),
    // Tilde but checking for other ranges
    ("~1.2.3", SemverRange::Exact),
    ("~1.2.3", SemverRange::Minor),
    ("~1.2.3", SemverRange::Lte),
    // Gte but checking for gt
    (">=1.2.3", SemverRange::Gt),
    (">=1.2.3", SemverRange::Exact),
    // Gt but checking for gte
    (">1.2.3", SemverRange::Gte),
    (">1.2.3", SemverRange::Minor),
    // Lt but checking for lte
    ("<1.2.3", SemverRange::Lte),
    ("<1.2.3", SemverRange::Exact),
    // Lte but checking for lt
    ("<=1.2.3", SemverRange::Lt),
    ("<=1.2.3", SemverRange::Patch),
    // Any but checking for others
    ("*", SemverRange::Exact),
    ("*", SemverRange::Minor),
    ("*", SemverRange::Patch),
  ];
  for (value, wrong_range) in cases {
    assert!(
      !Specifier::new(value).has_semver_range_of(&wrong_range),
      "Expected {value} to return false for {wrong_range:?}"
    );
  }
}

#[test]
fn comprehensive_range_check() {
  let test_data: Vec<(&str, SemverRange)> = vec![
    ("1.2.3", SemverRange::Exact),
    ("^1.2.3", SemverRange::Minor),
    ("~1.2.3", SemverRange::Patch),
    (">=1.2.3", SemverRange::Gte),
    (">1.2.3", SemverRange::Gt),
    ("<=1.2.3", SemverRange::Lte),
    ("<1.2.3", SemverRange::Lt),
    ("*", SemverRange::Any),
  ];

  let prefixes: Vec<&str> = vec!["", "npm:foo@", "workspace:"];

  for (base_value, expected_range) in &test_data {
    for prefix in &prefixes {
      let full_value = format!("{prefix}{base_value}");
      assert!(
        Specifier::new(&full_value).has_semver_range_of(expected_range),
        "Expected {full_value} to have {expected_range:?}"
      );

      // Test that it doesn't match other ranges
      for (_, other_range) in &test_data {
        if other_range != expected_range {
          assert!(
            !Specifier::new(&full_value).has_semver_range_of(other_range),
            "Expected {full_value} to NOT have {other_range:?}"
          );
        }
      }
    }
  }
}

#[test]
fn prerelease_versions_preserve_range() {
  let cases: Vec<(&str, SemverRange)> = vec![
    ("1.2.3-alpha.1", SemverRange::Exact),
    ("^1.2.3-beta.2", SemverRange::Minor),
    ("~1.2.3-rc.3", SemverRange::Patch),
    (">=1.2.3-next.4", SemverRange::Gte),
    (">1.2.3-canary.5", SemverRange::Gt),
    ("<=1.2.3-dev.6", SemverRange::Lte),
    ("<1.2.3-alpha", SemverRange::Lt),
  ];
  for (value, expected_range) in cases {
    assert!(
      Specifier::new(value).has_semver_range_of(&expected_range),
      "Expected {value} to have {expected_range:?}"
    );
  }
}

#[test]
fn edge_cases() {
  // Zero versions
  assert!(Specifier::new("0.0.0").has_semver_range_of(&SemverRange::Exact));
  assert!(Specifier::new("^0.0.0").has_semver_range_of(&SemverRange::Minor));
  assert!(Specifier::new("~0.0.0").has_semver_range_of(&SemverRange::Patch));

  // Large version numbers
  assert!(Specifier::new("999.999.999").has_semver_range_of(&SemverRange::Exact));
  assert!(Specifier::new("^999.999.999").has_semver_range_of(&SemverRange::Minor));

  // Single digit major
  assert!(Specifier::new("0").has_semver_range_of(&SemverRange::Exact));
  assert!(Specifier::new("^0").has_semver_range_of(&SemverRange::Minor));
  assert!(Specifier::new("~0").has_semver_range_of(&SemverRange::Patch));

  // Two component version
  assert!(Specifier::new("0.0").has_semver_range_of(&SemverRange::Exact));
  assert!(Specifier::new("^0.0").has_semver_range_of(&SemverRange::Minor));
  assert!(Specifier::new("~0.0").has_semver_range_of(&SemverRange::Patch));
}

#[test]
fn all_range_types_tested() {
  // Ensure we test all SemverRange variants
  let ranges_with_examples: Vec<(SemverRange, &str)> = vec![
    (SemverRange::Any, "*"),
    (SemverRange::Minor, "^1.2.3"),
    (SemverRange::Exact, "1.2.3"),
    (SemverRange::Gt, ">1.2.3"),
    (SemverRange::Gte, ">=1.2.3"),
    (SemverRange::Lt, "<1.2.3"),
    (SemverRange::Lte, "<=1.2.3"),
    (SemverRange::Patch, "~1.2.3"),
  ];

  for (range, example) in ranges_with_examples {
    assert!(
      Specifier::new(example).has_semver_range_of(&range),
      "Expected {example} to have {range:?}"
    );
  }
}
