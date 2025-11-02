use {crate::specifier::Specifier, std::rc::Rc};

#[test]
fn exact_version_satisfies_multiple_ranges() {
  let cases: Vec<(&str, Vec<&str>, bool)> = vec![
    // Single range
    ("1.2.3", vec!["^1.0.0"], true),
    ("1.2.3", vec!["^2.0.0"], false),
    // Multiple compatible ranges
    ("1.2.3", vec!["^1.0.0", "~1.2.0"], true),
    ("1.2.3", vec![">=1.0.0", "<2.0.0"], true),
    ("1.2.3", vec!["^1.0.0", ">=1.2.0", "<=1.3.0"], true),
    // One incompatible range fails all
    ("1.2.3", vec!["^1.0.0", "^2.0.0"], false),
    ("1.2.3", vec![">=1.0.0", "<1.2.0"], false),
    ("1.2.3", vec!["^1.0.0", "~1.3.0"], false),
    // Empty ranges (should be true - vacuous truth)
    ("1.2.3", vec![], true),
    // Many ranges all compatible
    ("1.2.3", vec![">=1.0.0", ">=1.2.0", ">=1.2.3", "<2.0.0", "<=1.2.3"], true),
  ];
  for (version, ranges_str, expected) in cases {
    let spec = Specifier::new(version);
    let others: Vec<Rc<Specifier>> = ranges_str.iter().map(|r| Specifier::new(r)).collect();
    assert_eq!(
      spec.satisfies_all(&others),
      expected,
      "'{version}'.satisfies_all({ranges_str:?}) should be {expected}"
    );
  }
}

#[test]
fn range_specifiers_satisfy_multiple_ranges() {
  let cases: Vec<(&str, Vec<&str>, bool)> = vec![
    // Caret satisfies multiple
    ("^1.2.3", vec!["^1.0.0"], true),
    ("^1.2.3", vec!["^1.0.0", ">=1.2.0"], true),
    ("^1.2.3", vec!["^1.0.0", "^2.0.0"], false),
    // Tilde satisfies multiple
    ("~1.2.3", vec!["~1.2.0"], true),
    ("~1.2.3", vec!["^1.0.0", "~1.2.0"], true),
    ("~1.2.3", vec!["~1.2.0", "~1.3.0"], false),
    // Major version
    ("1", vec!["^1.0.0", ">=1.0.0"], true),
    ("1", vec!["^1.0.0", "^2.0.0"], false),
    // Minor version
    ("1.2", vec!["^1.0.0", "~1.2.0"], true),
    ("1.2", vec!["^1.0.0", "^2.0.0"], false),
  ];
  for (specifier, ranges_str, expected) in cases {
    let spec = Specifier::new(specifier);
    let others: Vec<Rc<Specifier>> = ranges_str.iter().map(|r| Specifier::new(r)).collect();
    assert_eq!(
      spec.satisfies_all(&others),
      expected,
      "'{specifier}'.satisfies_all({ranges_str:?}) should be {expected}"
    );
  }
}

#[test]
#[ignore] // node-semver doesn't allow prereleases to satisfy non-prerelease ranges by
          // default
fn prerelease_versions_with_multiple_ranges() {
  let cases: Vec<(&str, Vec<&str>, bool)> = vec![
    // Prerelease satisfies multiple compatible ranges
    ("1.2.3-alpha.1", vec!["^1.0.0", ">=1.2.0"], true),
    ("1.2.3-alpha.1", vec![">=1.2.3-alpha.0", "<1.2.3-beta.0"], true),
    ("1.2.3-alpha.1", vec!["^1.2.3-alpha.0", ">=1.2.3-alpha.1"], true),
    // Prerelease fails one range
    ("1.2.3-alpha.1", vec!["^1.0.0", ">=1.2.3"], false),
    ("1.2.3-alpha.1", vec![">=1.2.3-alpha.0", ">=1.2.3-beta.0"], false),
    // Release with prerelease ranges
    ("1.2.3", vec![">=1.2.3-alpha.0", "<=2.0.0"], true),
    ("1.2.3", vec!["^1.2.3-alpha.0", ">=1.2.3"], false),
  ];
  for (version, ranges_str, expected) in cases {
    let spec = Specifier::new(version);
    let others: Vec<Rc<Specifier>> = ranges_str.iter().map(|r| Specifier::new(r)).collect();
    assert_eq!(
      spec.satisfies_all(&others),
      expected,
      "'{version}'.satisfies_all({ranges_str:?}) should be {expected}"
    );
  }
}

#[test]
fn non_version_specifiers_return_false() {
  let specifiers = vec![
    "",
    "alpha",
    "beta",
    "workspace:^",
    "https://example.com/package.tgz",
    "file:../foo",
    "github:user/repo#main",
    "}invalid{",
  ];
  let ranges_str = vec!["^1.0.0", ">=1.2.0"];
  let others: Vec<Rc<Specifier>> = ranges_str.iter().map(|r| Specifier::new(r)).collect();

  for specifier in specifiers {
    let spec = Specifier::new(specifier);
    assert!(
      !spec.satisfies_all(&others),
      "'{specifier}'.satisfies_all({ranges_str:?}) should return false (no concrete version or range)"
    );
  }
}

#[test]
fn edge_cases() {
  let cases: Vec<(&str, Vec<&str>, bool)> = vec![
    // Zero versions
    ("0.0.0", vec![">=0.0.0", "<=1.0.0"], true),
    ("0.0.0", vec![">0.0.0", "<1.0.0"], false),
    // Large versions
    ("999.999.999", vec![">=0.0.0", "<1000.0.0"], true),
    // Narrow range intersection
    ("1.2.3", vec![">=1.2.3", "<=1.2.3"], true),
    ("1.2.3", vec![">1.2.3", "<1.2.3"], false),
    // Many ranges
    ("5.0.0", vec![">=4.0.0", ">=5.0.0", "<=6.0.0", "<=5.0.0", "^5.0.0", "~5.0.0"], true),
    ("5.1.0", vec![">=4.0.0", ">=5.0.0", "<=6.0.0", "<=5.0.0", "^5.0.0", "~5.0.0"], false),
  ];
  for (version, ranges_str, expected) in cases {
    let spec = Specifier::new(version);
    let others: Vec<Rc<Specifier>> = ranges_str.iter().map(|r| Specifier::new(r)).collect();
    assert_eq!(
      spec.satisfies_all(&others),
      expected,
      "'{version}'.satisfies_all({ranges_str:?}) should be {expected}"
    );
  }
}

#[test]
fn alias_and_git_with_multiple_ranges() {
  let cases: Vec<(&str, Vec<&str>, bool)> = vec![
    // Alias with multiple ranges
    ("npm:lodash@4.17.21", vec!["^4.0.0", ">=4.17.0"], true),
    ("npm:lodash@4.17.21", vec!["^4.0.0", "^5.0.0"], false),
    ("npm:react@^18.2.0", vec![">=18.0.0", "<19.0.0"], true),
    // Git with multiple ranges
    ("github:user/repo#v1.2.3", vec!["^1.0.0", "~1.2.0"], true),
    ("github:user/repo#v1.2.3", vec!["^1.0.0", "^2.0.0"], false),
    ("github:user/repo#semver:^1.2.0", vec![">=1.2.0", "<2.0.0"], true),
  ];
  for (specifier, ranges_str, expected) in cases {
    let spec = Specifier::new(specifier);
    let others: Vec<Rc<Specifier>> = ranges_str.iter().map(|r| Specifier::new(r)).collect();
    assert_eq!(
      spec.satisfies_all(&others),
      expected,
      "'{specifier}'.satisfies_all({ranges_str:?}) should be {expected}"
    );
  }
}

#[test]
fn empty_ranges_array() {
  let cases = vec!["1.2.3", "^1.2.3", "~1.2.3", "npm:pkg@1.0.0", "github:user/repo#v1.0.0"];

  for specifier in cases {
    let spec = Specifier::new(specifier);
    let others: Vec<Rc<Specifier>> = vec![];
    assert!(
      spec.satisfies_all(&others),
      "'{specifier}'.satisfies_all([]) should be true (vacuous truth)"
    );
  }
}

#[test]
fn complex_range_specifiers_as_left_side() {
  let cases: Vec<(&str, Vec<&str>, bool)> = vec![
    // Complex OR ranges as left specifier
    ("~1.4.2 || ^1.4.2", vec!["1.4.2"], true),
    ("~1.0.0 || ^1.0.0", vec!["1.4.2"], true),
    ("~1.4.2 || ^1.4.2", vec!["^1.0.0", ">=1.4.0"], true),
    ("~1.0.0 || ^1.0.0", vec!["^1.5.0"], true),
    // Complex AND ranges as left specifier
    (">=1.0.0 <2.0.0", vec!["^1.5.0"], true),
    (">=1.0.0 <2.0.0", vec!["^2.0.0"], false),
    (">1.0.0 <=2.0.0", vec!["1.5.0"], true),
    (">1.0.0 <=2.0.0", vec!["1.0.0"], false),
    // Multiple ranges to satisfy
    (">=1.0.0 <2.0.0", vec![">=1.5.0", "<=1.8.0"], true),
    (">=1.0.0 <2.0.0", vec![">=1.5.0", ">=2.0.0"], false),
    // More complex cases
    ("<1.5.0 || >=1.6.0", vec!["1.4.0"], true),
    ("<1.5.0 || >=1.6.0", vec!["1.7.0"], true),
    ("<1.5.0 || >=1.6.0", vec!["1.5.5"], false),
    ("<1.5.0 || >=1.6.0", vec![">=1.3.0", "<=2.0.0"], true),
    // Real-world complex ranges
    ("<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2", vec!["1.6.0"], true),
    ("<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2", vec!["1.7.5"], true),
    ("<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2", vec!["1.8.1"], true),
    ("<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2", vec!["1.6.20"], false),
    ("<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2", vec![">=1.0.0", "<2.0.0"], true),
  ];
  for (specifier, ranges_str, expected) in cases {
    let spec = Specifier::new(specifier);
    let others: Vec<Rc<Specifier>> = ranges_str.iter().map(|r| Specifier::new(r)).collect();
    assert_eq!(
      spec.satisfies_all(&others),
      expected,
      "'{specifier}'.satisfies_all({ranges_str:?}) should be {expected}"
    );
  }
}

#[test]
fn original_test_coverage() {
  // These cases mirror the original Specifier::satisfies_all test
  // to ensure behavioral compatibility
  let cases: Vec<(&str, Vec<&str>, bool)> = vec![
    // Original test case: ("^1.4.2", vec!["1.4.2"], true)
    ("^1.4.2", vec!["1.4.2"], true),
    // Original test case: ("1.4.2", vec!["1.4.2"], true)
    ("1.4.2", vec!["1.4.2"], true),
    // Original test case: (">1.4.2", vec!["1.4.2"], false)
    (">1.4.2", vec!["1.4.2"], false),
    // Original test case: (">=1.4.2", vec!["1.4.2"], true)
    (">=1.4.2", vec!["1.4.2"], true),
    // Original test case: ("<1.4.2", vec!["1.4.2"], false)
    ("<1.4.2", vec!["1.4.2"], false),
    // Original test case: ("<=1.4.2", vec!["1.4.2"], true)
    ("<=1.4.2", vec!["1.4.2"], true),
    // Original test case: ("~1.4.2", vec!["1.4.2"], true)
    ("~1.4.2", vec!["1.4.2"], true),
    // Original test case: ("^1.0.0", vec!["1.4.2"], true)
    ("^1.0.0", vec!["1.4.2"], true),
    // Original test case: ("~1.0.0", vec!["1.4.2"], false)
    ("~1.0.0", vec!["1.4.2"], false),
    // Original test case: ("*", vec!["1.4.2"], true)
    // Note: "*" (Latest) returns range ">=0.0.0 <=999999.999999.999999" which satisfies any specific range
    ("*", vec!["1.4.2"], true),
    // Original test case: ("~1.4.2 || ^1.4.2", vec!["1.4.2"], true)
    ("~1.4.2 || ^1.4.2", vec!["1.4.2"], true),
    // Original test case: ("~1.0.0 || ^1.0.0", vec!["1.4.2"], true)
    ("~1.0.0 || ^1.0.0", vec!["1.4.2"], true),
  ];
  for (specifier, ranges_str, expected) in cases {
    let spec = Specifier::new(specifier);
    let others: Vec<Rc<Specifier>> = ranges_str.iter().map(|r| Specifier::new(r)).collect();
    assert_eq!(
      spec.satisfies_all(&others),
      expected,
      "'{specifier}'.satisfies_all({ranges_str:?}) should be {expected} (original test compatibility)"
    );
  }
}
