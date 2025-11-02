use crate::{semver_range::SemverRange, specifier::Specifier};

#[test]
fn basic_behavior() {
  let cases: Vec<(&str, Option<std::rc::Rc<Specifier>>)> = vec![
    ("npm:foo", None),
    ("npm:foo@1.2.3", Some(Specifier::new("npm:foo@~1.2.3"))),
    ("npm:@foo/bar@1.2.3", Some(Specifier::new("npm:@foo/bar@~1.2.3"))),
    ("1.2.3", Some(Specifier::new("~1.2.3"))),
    ("^1.2.3", Some(Specifier::new("~1.2.3"))),
    ("~1.2.3", Some(Specifier::new("~1.2.3"))),
    ("workspace:*", Some(Specifier::new("workspace:~"))),
  ];
  for (value, expected) in cases {
    assert_eq!(Specifier::new(value).with_range(&SemverRange::Patch), expected);
  }
}

#[test]
fn on_patch_variant() {
  let starts: Vec<&str> = vec!["", "npm:foo@", "workspace:"];
  let values: Vec<&str> = vec!["^1.2.3", "~1.2.3", ">=1.2.3", ">1.2.3", "<=1.2.3", "<1.2.3", "1.2.3"];
  let changes: Vec<(SemverRange, &str)> = vec![
    (SemverRange::Minor, "^1.2.3"),
    (SemverRange::Patch, "~1.2.3"),
    (SemverRange::Gte, ">=1.2.3"),
    (SemverRange::Gt, ">1.2.3"),
    (SemverRange::Lte, "<=1.2.3"),
    (SemverRange::Lt, "<1.2.3"),
    (SemverRange::Exact, "1.2.3"),
  ];
  for value in &values {
    for start in &starts {
      for (range, expected) in &changes {
        let full_value = format!("{start}{value}");
        let full_expected = format!("{start}{expected}");
        assert_eq!(Specifier::new(&full_value).with_range(range), Some(Specifier::new(&full_expected)),);
      }
    }
  }
}

#[test]
fn on_git_patch_variant() {
  let values: Vec<&str> = vec!["^1.2.3", "~1.2.3", ">=1.2.3", ">1.2.3", "<=1.2.3", "<1.2.3"];
  let changes: Vec<(SemverRange, &str)> = vec![
    (SemverRange::Minor, "^1.2.3"),
    (SemverRange::Patch, "~1.2.3"),
    (SemverRange::Gte, ">=1.2.3"),
    (SemverRange::Gt, ">1.2.3"),
    (SemverRange::Lte, "<=1.2.3"),
    (SemverRange::Lt, "<1.2.3"),
  ];
  for value in &values {
    let full_value = format!("git@github.com:npm/cli.git#semver:{value}");
    for (range, expected) in &changes {
      let full_expected = format!("git@github.com:npm/cli.git#semver:{expected}");
      assert_eq!(Specifier::new(&full_value).with_range(range), Some(Specifier::new(&full_expected)),);
    }
  }

  // Test exact version separately (uses # not #semver:)
  let exact_value = "git@github.com:npm/cli.git#1.2.3";
  assert_eq!(
    Specifier::new(exact_value).with_range(&SemverRange::Exact),
    Some(Specifier::new("git@github.com:npm/cli.git#1.2.3")),
  );
}

#[test]
fn on_major_variant() {
  let starts: Vec<&str> = vec!["", "npm:foo@", "workspace:"];
  let values: Vec<&str> = vec!["^1", "~1", ">=1", ">1", "<=1", "<1", "1"];
  let changes: Vec<(SemverRange, &str)> = vec![
    (SemverRange::Minor, "^1"),
    (SemverRange::Patch, "~1"),
    (SemverRange::Gte, ">=1"),
    (SemverRange::Gt, ">1"),
    (SemverRange::Lte, "<=1"),
    (SemverRange::Lt, "<1"),
    (SemverRange::Exact, "1"),
  ];
  for value in values {
    for start in &starts {
      for (range, expected) in &changes {
        let full_value = format!("{start}{value}");
        let full_expected = format!("{start}{expected}");
        assert_eq!(Specifier::new(&full_value).with_range(range), Some(Specifier::new(&full_expected)),);
      }
    }
  }
}

#[test]
fn on_git_major_variant() {
  let values: Vec<&str> = vec!["^1", "~1", ">=1", ">1", "<=1", "<1"];
  let changes: Vec<(SemverRange, &str)> = vec![
    (SemverRange::Minor, "semver:^1"),
    (SemverRange::Patch, "semver:~1"),
    (SemverRange::Gte, "semver:>=1"),
    (SemverRange::Gt, "semver:>1"),
    (SemverRange::Lte, "semver:<=1"),
    (SemverRange::Lt, "semver:<1"),
  ];
  for value in values {
    for (range, expected) in &changes {
      let full_value = format!("git@github.com:npm/cli.git#semver:{value}");
      let full_expected = format!("git@github.com:npm/cli.git#{expected}");
      assert_eq!(Specifier::new(&full_value).with_range(range), Some(Specifier::new(&full_expected)),);
    }
  }

  // Test exact version separately (uses # not #semver:)
  let exact_value = "git@github.com:npm/cli.git#1";
  assert_eq!(
    Specifier::new(exact_value).with_range(&SemverRange::Exact),
    Some(Specifier::new("git@github.com:npm/cli.git#1")),
  );
}

#[test]
fn to_any() {
  let range = SemverRange::Any;

  // Define version values
  let ranges_only = vec!["^1", "~1", ">=1", ">1", "<=1", "<1"];
  let exact_versions = vec!["1", "1.2", "1.2.3"];
  let mut all_versions = ranges_only.clone();
  all_versions.extend(exact_versions.iter());

  // Add minor and patch variants of ranges
  let full_version_set: Vec<String> = ranges_only
    .iter()
    .flat_map(|r| vec![r.to_string(), format!("{}.2", r), format!("{}.2.3", r)])
    .chain(exact_versions.iter().map(|v| v.to_string()))
    .collect();

  // Helper to test a prefix pattern
  let test_pattern = |prefix: &str, expected: &str, versions: &[String]| {
    for version in versions {
      let full_value = format!("{prefix}{version}");
      let actual = Specifier::new(&full_value).with_range(&range);
      let expected = Some(Specifier::new(expected));
      assert_eq!(actual, expected, "Failed for: {full_value}");
    }
  };

  // Test basic semver (no prefix)
  test_pattern("", "*", &full_version_set);

  // Test npm aliases
  test_pattern("npm:foo@", "npm:foo", &full_version_set);

  // Test workspace protocol
  test_pattern("workspace:", "workspace:*", &full_version_set);

  // Test git URLs with semver ranges
  let git_ranges: Vec<String> = ranges_only
    .iter()
    .flat_map(|r| vec![r.to_string(), format!("{}.2", r), format!("{}.2.3", r)])
    .collect();

  for version in &git_ranges {
    let full_value = format!("git@github.com:npm/cli.git#semver:{version}");
    let actual = Specifier::new(&full_value).with_range(&range);
    let expected = Some(Specifier::new("git@github.com:npm/cli.git"));
    assert_eq!(actual, expected, "Failed for: {full_value}");
  }

  // Test git URLs with exact versions
  for version in &exact_versions {
    let full_value = format!("git@github.com:npm/cli.git#{version}");
    let actual = Specifier::new(&full_value).with_range(&range);
    let expected = Some(Specifier::new("git@github.com:npm/cli.git"));
    assert_eq!(actual, expected, "Failed for: {full_value}");
  }
}
