use crate::{specifier::semver_range::SemverRange, specifier2::Specifier2, test::faker};

#[test]
fn valid_latest() {
  for value in faker::get_latest() {
    assert_eq!(*Specifier2::new(value), Specifier2::Latest(value.to_string()));
  }
}

#[test]
fn valid_tag() {
  for value in faker::get_tag() {
    assert_eq!(*Specifier2::new(value), Specifier2::Tag(value.to_string()));
  }
}

#[test]
fn valid_major() {
  for value in faker::get_major() {
    assert_eq!(*Specifier2::new(value), Specifier2::Major(value.to_string()));
  }
}

#[test]
fn valid_minor() {
  for value in faker::get_minor() {
    assert_eq!(*Specifier2::new(value), Specifier2::Minor(value.to_string()));
  }
}

#[test]
fn valid_exact() {
  for value in faker::get_exact() {
    assert_eq!(*Specifier2::new(value), Specifier2::Exact(value.to_string()));
  }
}

#[test]
fn valid_complex_semver() {
  for value in faker::get_complex_semver() {
    assert_eq!(*Specifier2::new(value), Specifier2::ComplexSemver(value.to_string()));
  }
}

#[test]
fn valid_workspace_protocol() {
  for value in faker::get_workspace_protocol() {
    assert_eq!(*Specifier2::new(value), Specifier2::WorkspaceProtocol(value.to_string()));
  }
}

#[test]
fn valid_range() {
  for value in faker::get_range() {
    assert_eq!(*Specifier2::new(value), Specifier2::Range(value.to_string()));
  }
}

#[test]
fn valid_unsupported() {
  for value in faker::get_unsupported() {
    assert_eq!(*Specifier2::new(value), Specifier2::Unsupported(value.to_string()));
  }
}

#[test]
fn valid_range_major() {
  for value in faker::get_range_major() {
    assert_eq!(*Specifier2::new(value), Specifier2::RangeMajor(value.to_string()));
  }
}

#[test]
fn valid_range_minor() {
  for value in faker::get_range_minor() {
    assert_eq!(*Specifier2::new(value), Specifier2::RangeMinor(value.to_string()));
  }
}

#[test]
fn valid_alias() {
  for value in faker::get_alias() {
    assert_eq!(*Specifier2::new(value), Specifier2::Alias(value.to_string()));
  }
}

#[test]
fn valid_file() {
  for value in faker::get_file() {
    assert_eq!(*Specifier2::new(value), Specifier2::File(value.to_string()));
  }
}

#[test]
fn valid_git() {
  for value in faker::get_git() {
    assert_eq!(*Specifier2::new(value), Specifier2::Git(value.to_string()));
  }
}

#[test]
fn valid_url() {
  for value in faker::get_url() {
    assert_eq!(*Specifier2::new(value), Specifier2::Url(value.to_string()));
  }
}

#[test]
fn get_alias_name() {
  let cases: Vec<(&str, Option<&str>)> = vec![
    ("npm:foo", Some("foo")),
    ("npm:foo@1.2.3", Some("foo")),
    ("npm:@foo/bar@1.2.3", Some("@foo/bar")),
    ("1.2.3", None),
  ];
  for (value, expected) in cases {
    assert_eq!(Specifier2::new(value).get_alias_name(), expected);
  }
}

#[test]
fn get_semver_number() {
  let cases: Vec<(&str, Option<&str>)> = vec![
    ("npm:foo", None),
    ("npm:foo@1.2.3", Some("1.2.3")),
    ("npm:@foo/bar@1.2.3", Some("1.2.3")),
    ("1.2.3", Some("1.2.3")),
    ("^1.2.3", Some("1.2.3")),
  ];
  for (value, expected) in cases {
    assert_eq!(Specifier2::new(value).get_semver_number(), expected);
  }
}

#[test]
fn with_range() {
  let cases: Vec<(&str, Option<String>)> = vec![
    ("npm:foo", None),
    ("npm:foo@1.2.3", Some("npm:foo@~1.2.3".to_string())),
    ("npm:@foo/bar@1.2.3", Some("npm:@foo/bar@~1.2.3".to_string())),
    ("1.2.3", Some("~1.2.3".to_string())),
    ("^1.2.3", Some("~1.2.3".to_string())),
    ("~1.2.3", Some("~1.2.3".to_string())),
    ("workspace:*", Some("workspace:~".to_string())),
  ];
  for (value, expected) in cases {
    assert_eq!(Specifier2::new(value).with_range(&SemverRange::Patch), expected);
  }
}
