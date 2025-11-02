use crate::specifier::Specifier;

#[test]
fn complex_semver_returns_node_range() {
  let cases = vec![
    ">=1.0.0 <2.0.0",
    "~1.4.2 || ^1.4.2",
    "<1.5.0 || >=1.6.0",
    ">1.0.0 <=2.0.0",
    "<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2",
    "~1.0.0 || ^1.0.0",
    ">=2.3.4 || <=1.2.3",
  ];

  for value in cases {
    let spec = Specifier::new(value);
    let range = spec.get_node_range();
    assert!(
      range.is_some(),
      "ComplexSemver '{value}' should return Some(range) from get_node_range()"
    );
  }
}

#[test]
fn range_variants_return_node_range() {
  let cases = vec![
    ("^1.2.3", "Range"),
    ("~1.2.3", "Range"),
    (">=1.2.3", "Range"),
    (">1.2.3", "Range"),
    ("<=1.2.3", "Range"),
    ("<1.2.3", "Range"),
  ];

  for (value, variant) in cases {
    let spec = Specifier::new(value);
    let range = spec.get_node_range();
    assert!(
      range.is_some(),
      "{variant} '{value}' should return Some(range) from get_node_range()"
    );
  }
}

#[test]
fn range_major_and_minor_return_node_range() {
  let cases = vec![
    ("^1", "RangeMajor"),
    ("~1", "RangeMajor"),
    ("^1.2", "RangeMinor"),
    ("~1.2", "RangeMinor"),
  ];

  for (value, variant) in cases {
    let spec = Specifier::new(value);
    let range = spec.get_node_range();
    assert!(
      range.is_some(),
      "{variant} '{value}' should return Some(range) from get_node_range()"
    );
  }
}

#[test]
fn alias_with_version_returns_node_range() {
  let cases = vec!["npm:lodash@4.17.21", "npm:react@^18.0.0", "npm:foo@~1.2.3"];

  for value in cases {
    let spec = Specifier::new(value);
    let range = spec.get_node_range();
    assert!(
      range.is_some(),
      "Alias '{value}' with version should return Some(range) from get_node_range()"
    );
  }
}

#[test]
fn git_with_semver_returns_node_range() {
  let cases = vec!["github:user/repo#v1.2.3", "github:user/repo#semver:^1.2.0"];

  for value in cases {
    let spec = Specifier::new(value);
    let range = spec.get_node_range();
    assert!(
      range.is_some(),
      "Git '{value}' with semver should return Some(range) from get_node_range()"
    );
  }
}

#[test]
fn latest_returns_node_range() {
  let cases = vec![
    ("*", "Latest", ">=0.0.0 <=999999.999999.999999"),
    ("latest", "Latest", ">=0.0.0 <=999999.999999.999999"),
    ("x", "Latest", ">=0.0.0 <=999999.999999.999999"),
    ("*", "Latest", ">=0.0.0 <=999999.999999.999999"),
    ("workspace:*", "Latest", ">=0.0.0 <=999999.999999.999999"),
  ];

  for (value, variant, expected_range) in cases {
    let spec = Specifier::new(value);
    let range = spec.get_node_range();
    assert!(
      range.is_some(),
      "{variant} '{value}' should return Some(range) from get_node_range()"
    );
    assert_eq!(
      range.unwrap().to_string(),
      expected_range,
      "{variant} '{value}' should produce range '{expected_range}'"
    );
  }
}

#[test]
fn workspace_protocol_resolved_returns_node_range() {
  // Resolved WorkspaceProtocol (with embedded version) should return range
  let cases = vec!["workspace:^1.2.3", "workspace:~1.2.3", "workspace:1.2.3", "workspace:*"];

  for value in cases {
    let spec = Specifier::new(value);
    let range = spec.get_node_range();
    assert!(
      range.is_some(),
      "WorkspaceProtocol '{value}' with embedded version should return Some(range)"
    );
  }
}

#[test]
fn non_version_specifiers_return_none() {
  let cases = vec![
    ("", "None"),
    ("alpha", "Tag"),
    ("beta", "Tag"),
    ("next", "Tag"),
    ("workspace:^", "WorkspaceProtocol (unresolved)"),
    ("workspace:~", "WorkspaceProtocol (unresolved)"),
    ("https://example.com/package.tgz", "Url"),
    ("file:../foo", "File"),
    ("github:user/repo#main", "Git (no semver)"),
    ("}invalid{", "Unsupported"),
  ];

  for (value, variant) in cases {
    let spec = Specifier::new(value);
    let range = spec.get_node_range();
    assert!(range.is_none(), "{variant} '{value}' should return None from get_node_range()");
  }
}

#[test]
fn major_and_minor_return_computed_ranges() {
  let cases = vec![
    ("1", "Major", ">=1.0.0 <2.0.0"),
    ("2", "Major", ">=2.0.0 <3.0.0"),
    ("1.2", "Minor", ">=1.2.0 <1.3.0"),
    ("1.5", "Minor", ">=1.5.0 <1.6.0"),
  ];

  for (value, variant, expected_range) in cases {
    let spec = Specifier::new(value);
    let range = spec.get_node_range();
    assert!(
      range.is_some(),
      "{variant} '{value}' should return Some(range) from get_node_range()"
    );
    assert_eq!(
      range.unwrap().to_string(),
      expected_range,
      "{variant} '{value}' should produce range '{expected_range}'"
    );
  }
}

#[test]
fn exact_returns_node_range() {
  let cases = vec![("1.2.3", "Exact")];

  for (value, variant) in cases {
    let spec = Specifier::new(value);
    let range = spec.get_node_range();
    assert!(
      range.is_some(),
      "{variant} '{value}' should return Some(range) from get_node_range()"
    );
  }
}
