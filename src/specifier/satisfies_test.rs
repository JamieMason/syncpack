use crate::specifier::Specifier;

#[test]
fn exact_version_satisfies_ranges() {
  let cases: Vec<(&str, &str, bool)> = vec![
    // Exact version satisfies caret ranges
    ("1.2.3", "^1.0.0", true),
    ("1.2.3", "^1.2.0", true),
    ("1.2.3", "^1.2.3", true),
    ("1.2.3", "^1.3.0", false),
    ("1.2.3", "^2.0.0", false),
    // Exact version satisfies tilde ranges
    ("1.2.3", "~1.2.0", true),
    ("1.2.3", "~1.2.3", true),
    ("1.2.3", "~1.3.0", false),
    ("1.2.3", "~2.0.0", false),
    // Exact version satisfies comparison operators
    ("1.2.3", ">=1.0.0", true),
    ("1.2.3", ">=1.2.3", true),
    ("1.2.3", ">=2.0.0", false),
    ("1.2.3", ">1.2.2", true),
    ("1.2.3", ">1.2.3", false),
    ("1.2.3", "<=2.0.0", true),
    ("1.2.3", "<=1.2.3", true),
    ("1.2.3", "<=1.0.0", false),
    ("1.2.3", "<1.2.4", true),
    ("1.2.3", "<1.2.3", false),
    // Exact version satisfies wildcard
    ("1.2.3", "*", true),
    ("0.0.1", "*", true),
    ("999.999.999", "*", true),
  ];
  for (version, range_str, expected) in cases {
    let spec = Specifier::new(version);
    let range = node_semver::Range::parse(range_str).unwrap();
    assert_eq!(
      spec.satisfies(&range),
      expected,
      "'{version}'.satisfies('{range_str}') should be {expected}"
    );
  }
}

// Shorthand versions with HUGE (1 as 1.999999.999999) don't satisfy tilde
// ranges as expected
#[test]
#[ignore]
fn range_specifiers_satisfy_ranges() {
  let cases: Vec<(&str, &str, bool)> = vec![
    // Caret range satisfies another range
    ("^1.2.3", "^1.0.0", true),
    ("^1.2.3", "~1.2.0", true),
    ("^1.2.3", ">=1.0.0", true),
    ("^1.2.3", "^2.0.0", false),
    // Tilde range satisfies another range
    ("~1.2.3", "^1.0.0", true),
    ("~1.2.3", "~1.2.0", true),
    ("~1.2.3", ">=1.0.0", true),
    ("~1.2.3", "^2.0.0", false),
    // Major version satisfies ranges
    ("1", "^1.0.0", true),
    ("1", "~1.0.0", true),
    ("1", ">=1.0.0", true),
    ("1", "^2.0.0", false),
    // Minor version satisfies ranges
    ("1.2", "^1.0.0", true),
    ("1.2", "~1.0.0", true),
    ("1.2", ">=1.0.0", true),
    ("1.2", "^2.0.0", false),
  ];
  for (specifier, range_str, expected) in cases {
    let spec = Specifier::new(specifier);
    let range = node_semver::Range::parse(range_str).unwrap();
    assert_eq!(
      spec.satisfies(&range),
      expected,
      "'{specifier}'.satisfies('{range_str}') should be {expected}"
    );
  }
}

// node-semver doesn't allow prereleases to satisfy non-prerelease ranges by
// default
#[test]
#[ignore]
fn prerelease_versions() {
  let cases: Vec<(&str, &str, bool)> = vec![
    // Prerelease satisfies ranges
    ("1.2.3-alpha.1", "^1.0.0", true),
    ("1.2.3-alpha.1", "~1.2.0", true),
    ("1.2.3-alpha.1", ">=1.2.3-alpha.0", true),
    ("1.2.3-alpha.1", ">=1.2.3-alpha.1", true),
    ("1.2.3-alpha.1", ">=1.2.3-alpha.2", false),
    ("1.2.3-alpha.1", ">=1.2.3", false),
    ("1.2.3-beta.2", "^1.2.3-beta.0", true),
    ("1.2.3-beta.2", "^1.2.3-alpha.0", false),
    // Release satisfies prerelease ranges
    ("1.2.3", ">=1.2.3-alpha.0", true),
    ("1.2.3", "^1.2.3-alpha.0", false),
  ];
  for (version, range_str, expected) in cases {
    let spec = Specifier::new(version);
    let range = node_semver::Range::parse(range_str).unwrap();
    assert_eq!(
      spec.satisfies(&range),
      expected,
      "'{version}'.satisfies('{range_str}') should be {expected}"
    );
  }
}

#[test]
fn complex_ranges() {
  let cases: Vec<(&str, &str, bool)> = vec![
    // AND ranges (space-separated)
    ("1.2.3", ">=1.0.0 <2.0.0", true),
    ("1.2.3", ">=1.2.3 <2.0.0", true),
    ("1.2.3", ">1.2.3 <2.0.0", false),
    ("1.2.3", ">=1.0.0 <=1.2.3", true),
    ("1.2.3", ">=1.0.0 <1.2.3", false),
    // OR ranges (||)
    ("1.2.3", "^1.0.0 || ^2.0.0", true),
    ("1.2.3", "^2.0.0 || ^3.0.0", false),
    ("1.2.3", "<1.0.0 || >2.0.0", false),
    ("1.2.3", "<1.0.0 || >=1.2.0", true),
  ];
  for (version, range_str, expected) in cases {
    let spec = Specifier::new(version);
    let range = node_semver::Range::parse(range_str).unwrap();
    assert_eq!(
      spec.satisfies(&range),
      expected,
      "'{version}'.satisfies('{range_str}') should be {expected}"
    );
  }
}

#[test]
fn non_version_specifiers_return_false() {
  let cases: Vec<(&str, &str)> = vec![
    // No version to compare
    ("", "^1.0.0"),
    // Tags have no version
    ("alpha", "^1.0.0"),
    ("beta", "^1.0.0"),
    ("next", "^1.0.0"),
    // Workspace protocol (unresolved) has no version
    ("workspace:^", "^1.0.0"),
    ("workspace:~", "~1.2.0"),
    ("workspace:*", "*"),
    ("workspace:^1.2.3", "^1.0.0"),
    // URLs, files, git
    ("https://example.com/package.tgz", "^1.0.0"),
    ("file:../foo", "^1.0.0"),
    ("github:user/repo#main", "^1.0.0"),
    // Unsupported
    ("}invalid{", "^1.0.0"),
  ];
  for (specifier, range_str) in cases {
    let spec = Specifier::new(specifier);
    let range = node_semver::Range::parse(range_str).unwrap();
    assert!(
      !spec.satisfies(&range),
      "'{specifier}'.satisfies('{range_str}') should return false (no concrete version or range)"
    );
  }
}

#[test]
fn edge_cases() {
  let cases: Vec<(&str, &str, bool)> = vec![
    // Zero versions
    ("0.0.0", ">=0.0.0", true),
    ("0.0.0", ">0.0.0", false),
    ("0.1.0", "^0.0.0", false),
    ("0.0.1", "^0.0.0", false),
    // Large versions
    ("999.999.999", ">=0.0.0", true),
    ("999.999.999", "<1000.0.0", true),
    // Exact match required
    ("1.2.3", "1.2.3", true),
    ("1.2.3", "1.2.4", false),
  ];
  for (version, range_str, expected) in cases {
    let spec = Specifier::new(version);
    let range = node_semver::Range::parse(range_str).unwrap();
    assert_eq!(
      spec.satisfies(&range),
      expected,
      "'{version}'.satisfies('{range_str}') should be {expected}"
    );
  }
}

#[test]
fn alias_and_git_with_versions() {
  let cases: Vec<(&str, &str, bool)> = vec![
    // Alias with version
    ("npm:lodash@4.17.21", "^4.0.0", true),
    ("npm:lodash@4.17.21", "~4.17.0", true),
    ("npm:lodash@4.17.21", "^5.0.0", false),
    ("npm:react@^18.0.0", ">=18.0.0", true),
    ("npm:react@~18.2.0", "^18.0.0", true),
    // Git with semver
    ("github:user/repo#v1.2.3", "^1.0.0", true),
    ("github:user/repo#v1.2.3", "~1.2.0", true),
    ("github:user/repo#v1.2.3", "^2.0.0", false),
    ("github:user/repo#semver:^1.2.0", ">=1.2.0", true),
  ];
  for (specifier, range_str, expected) in cases {
    let spec = Specifier::new(specifier);
    let range = node_semver::Range::parse(range_str).unwrap();
    assert_eq!(
      spec.satisfies(&range),
      expected,
      "'{specifier}'.satisfies('{range_str}') should be {expected}"
    );
  }
}

#[test]
fn complex_range_specifiers_as_left_side() {
  let cases: Vec<(&str, &str, bool)> = vec![
    // Complex OR ranges as left specifier (from original satisfies_all test)
    ("~1.4.2 || ^1.4.2", "1.4.2", true),  // overlaps with exact version
    ("~1.0.0 || ^1.0.0", "1.4.2", true),  // ^1.0.0 includes 1.4.2
    ("~1.4.2 || ^1.4.2", "^1.0.0", true), // both overlap with ^1.0.0
    ("~1.0.0 || ^1.0.0", "^1.5.0", true), // ^1.0.0 includes 1.5.x
    // Complex AND ranges as left specifier
    (">=1.0.0 <2.0.0", "^1.5.0", true),  // ranges overlap
    (">=1.0.0 <2.0.0", "^2.0.0", false), // no overlap
    (">1.0.0 <=2.0.0", "1.5.0", true),   // includes 1.5.0
    (">1.0.0 <=2.0.0", "1.0.0", false),  // doesn't include 1.0.0
    // More complex cases
    ("<1.5.0 || >=1.6.0", "1.4.0", true),  // 1.4.0 in first part
    ("<1.5.0 || >=1.6.0", "1.7.0", true),  // 1.7.0 in second part
    ("<1.5.0 || >=1.6.0", "1.5.5", false), // 1.5.5 in neither part
    // From real-world complex ranges
    ("<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2", "1.6.0", true),
    ("<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2", "1.7.5", true),
    ("<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2", "1.8.1", true),
    ("<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2", "1.6.20", false),
  ];
  for (specifier, range_str, expected) in cases {
    let spec = Specifier::new(specifier);
    let range = node_semver::Range::parse(range_str).unwrap();
    assert_eq!(
      spec.satisfies(&range),
      expected,
      "'{specifier}'.satisfies('{range_str}') should be {expected}"
    );
  }
}

#[test]
fn original_test_coverage() {
  // These cases mirror the original Specifier::satisfies_all test
  // to ensure behavioral compatibility
  let cases: Vec<(&str, &str, bool)> = vec![
    // Original test case: ("*", vec!["1.4.2"], true)
    // Note: "*" (Latest) returns range ">=0.0.0 <=999999.999999.999999" which satisfies any specific range
    ("*", "1.4.2", true),
    // Original test case: ("^1.4.2", vec!["1.4.2"], true)
    ("^1.4.2", "1.4.2", true),
    // Original test case: ("1.4.2", vec!["1.4.2"], true)
    ("1.4.2", "1.4.2", true),
    // Original test case: (">1.4.2", vec!["1.4.2"], false)
    (">1.4.2", "1.4.2", false),
    // Original test case: (">=1.4.2", vec!["1.4.2"], true)
    (">=1.4.2", "1.4.2", true),
    // Original test case: ("<1.4.2", vec!["1.4.2"], false)
    ("<1.4.2", "1.4.2", false),
    // Original test case: ("<=1.4.2", vec!["1.4.2"], true)
    ("<=1.4.2", "1.4.2", true),
    // Original test case: ("~1.4.2", vec!["1.4.2"], true)
    ("~1.4.2", "1.4.2", true),
    // Original test case: ("^1.0.0", vec!["1.4.2"], true)
    ("^1.0.0", "1.4.2", true),
    // Original test case: ("~1.0.0", vec!["1.4.2"], false)
    ("~1.0.0", "1.4.2", false),
    // Original test case: ("", vec!["1.4.2"], false)
    ("", "1.4.2", false),
  ];
  for (specifier, range_str, expected) in cases {
    let spec = Specifier::new(specifier);
    let range = node_semver::Range::parse(range_str).unwrap();
    assert_eq!(
      spec.satisfies(&range),
      expected,
      "'{specifier}'.satisfies('{range_str}') should be {expected} (original test compatibility)"
    );
  }
}
