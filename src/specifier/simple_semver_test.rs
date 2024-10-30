use {
  super::*,
  node_semver::{Identifier, Version},
};

#[test]
fn returns_err_when_specifier_is_not_simple_semver() {
  assert_eq!(
    SimpleSemver::new("<2 || >3"),
    Err("'<2 || >3' was expected to be a simple semver specifier but was not recognised".to_string())
  );
}

#[test]
fn returns_struct_for_comparison_and_sorting() {
  let cases: Vec<(&str, Orderable)> = vec![
    (
      "0.0.0",
      Orderable {
        range: SemverRange::Exact,
        version: Version {
          major: 0,
          minor: 0,
          patch: 0,
          build: vec![],
          pre_release: vec![],
        },
      },
    ),
    (
      "1.2.3-alpha",
      Orderable {
        range: SemverRange::Exact,
        version: Version {
          major: 1,
          minor: 2,
          patch: 3,
          build: vec![],
          pre_release: vec![Identifier::AlphaNumeric("alpha".to_string())],
        },
      },
    ),
    (
      "1.2.3-rc.18",
      Orderable {
        range: SemverRange::Exact,
        version: Version {
          major: 1,
          minor: 2,
          patch: 3,
          build: vec![],
          pre_release: vec![Identifier::AlphaNumeric("rc".to_string()), Identifier::Numeric(18)],
        },
      },
    ),
  ];
  for (str, expected) in cases {
    let raw = str.to_string();
    let semver = SimpleSemver::new(&raw).unwrap();
    let orderable = semver.get_orderable();
    assert_eq!(orderable.range, expected.range, "range");
    assert_eq!(orderable.version.major, expected.version.major, "version.major");
    assert_eq!(orderable.version.minor, expected.version.minor, "version.minor");
    assert_eq!(orderable.version.patch, expected.version.patch, "version.patch");
    assert_eq!(orderable.version.build, expected.version.build, "version.build");
    assert_eq!(orderable.version.pre_release, expected.version.pre_release, "version.pre_release");
  }
}

#[test]
fn replaces_the_semver_range_of_a_specifier() {
  let cases: Vec<(&str, SemverRange, SimpleSemver)> = vec![
    // from exact
    ("0.0.0", SemverRange::Any, SimpleSemver::Latest("*".to_string())),
    ("0.0.0", SemverRange::Minor, SimpleSemver::Range("^0.0.0".to_string())),
    ("0.0.0", SemverRange::Exact, SimpleSemver::Exact("0.0.0".to_string())),
    ("0.0.0", SemverRange::Gt, SimpleSemver::Range(">0.0.0".to_string())),
    ("0.0.0", SemverRange::Gte, SimpleSemver::Range(">=0.0.0".to_string())),
    ("0.0.0", SemverRange::Lt, SimpleSemver::Range("<0.0.0".to_string())),
    ("0.0.0", SemverRange::Lte, SimpleSemver::Range("<=0.0.0".to_string())),
    ("0.0.0", SemverRange::Patch, SimpleSemver::Range("~0.0.0".to_string())),
    // from another range
    ("~0.0.0", SemverRange::Any, SimpleSemver::Latest("*".to_string())),
    ("~0.0.0", SemverRange::Minor, SimpleSemver::Range("^0.0.0".to_string())),
    ("~0.0.0", SemverRange::Exact, SimpleSemver::Exact("0.0.0".to_string())),
    ("~0.0.0", SemverRange::Gt, SimpleSemver::Range(">0.0.0".to_string())),
    ("~0.0.0", SemverRange::Gte, SimpleSemver::Range(">=0.0.0".to_string())),
    ("~0.0.0", SemverRange::Lt, SimpleSemver::Range("<0.0.0".to_string())),
    ("~0.0.0", SemverRange::Lte, SimpleSemver::Range("<=0.0.0".to_string())),
    ("~0.0.0", SemverRange::Patch, SimpleSemver::Range("~0.0.0".to_string())),
    // from major
    ("0", SemverRange::Any, SimpleSemver::Latest("*".to_string())),
    ("0", SemverRange::Minor, SimpleSemver::Major("0".to_string())),
    ("0", SemverRange::Exact, SimpleSemver::Major("0".to_string())),
    ("0", SemverRange::Gt, SimpleSemver::Major("0".to_string())),
    ("0", SemverRange::Gte, SimpleSemver::Major("0".to_string())),
    ("0", SemverRange::Lt, SimpleSemver::Major("0".to_string())),
    ("0", SemverRange::Lte, SimpleSemver::Major("0".to_string())),
    ("0", SemverRange::Patch, SimpleSemver::Major("0".to_string())),
    // from minor
    ("0.0", SemverRange::Any, SimpleSemver::Latest("*".to_string())),
    ("0.0", SemverRange::Minor, SimpleSemver::RangeMinor("^0.0".to_string())),
    ("0.0", SemverRange::Exact, SimpleSemver::Minor("0.0".to_string())),
    ("0.0", SemverRange::Gt, SimpleSemver::RangeMinor(">0.0".to_string())),
    ("0.0", SemverRange::Gte, SimpleSemver::RangeMinor(">=0.0".to_string())),
    ("0.0", SemverRange::Lt, SimpleSemver::RangeMinor("<0.0".to_string())),
    ("0.0", SemverRange::Lte, SimpleSemver::RangeMinor("<=0.0".to_string())),
    ("0.0", SemverRange::Patch, SimpleSemver::RangeMinor("~0.0".to_string())),
    // from another range minor
    ("^0.0", SemverRange::Any, SimpleSemver::Latest("*".to_string())),
    ("^0.0", SemverRange::Minor, SimpleSemver::RangeMinor("^0.0".to_string())),
    ("^0.0", SemverRange::Exact, SimpleSemver::Minor("0.0".to_string())),
    ("^0.0", SemverRange::Gt, SimpleSemver::RangeMinor(">0.0".to_string())),
    ("^0.0", SemverRange::Gte, SimpleSemver::RangeMinor(">=0.0".to_string())),
    ("^0.0", SemverRange::Lt, SimpleSemver::RangeMinor("<0.0".to_string())),
    ("^0.0", SemverRange::Lte, SimpleSemver::RangeMinor("<=0.0".to_string())),
    ("^0.0", SemverRange::Patch, SimpleSemver::RangeMinor("~0.0".to_string())),
  ];
  for (before, range, expected) in cases {
    let semver = SimpleSemver::new(before).unwrap();
    let after = semver.with_range(&range);
    assert_eq!(after, expected);
  }
}

#[test]
fn cannot_replace_the_semver_range_of_latest_since_the_version_is_not_known() {
  let before = SimpleSemver::new("*").unwrap();
  let after = before.with_range(&SemverRange::Exact);
  assert_eq!(after, SimpleSemver::Latest("*".to_string()));
}
