use std::rc::Rc;

use crate::{specifier::semver_range::SemverRange, specifier2::Specifier2, test::faker};

fn ranges() -> Vec<(&'static str, SemverRange)> {
  vec![
    // ("*", SemverRange::Any),
    ("", SemverRange::Exact),
    ("^", SemverRange::Minor),
    ("~", SemverRange::Patch),
    (">=", SemverRange::Gte),
    (">", SemverRange::Gt),
    ("<=", SemverRange::Lte),
    ("<", SemverRange::Lt),
  ]
}

fn prereleases() -> Vec<&'static str> {
  vec!["", "-alpha", "-alpha.0"]
}

fn protocols() -> Vec<&'static str> {
  vec!["", "workspace:"]
}

fn npm_names() -> Vec<&'static str> {
  vec!["@jsr/std__fs", "@minh.nguyen/plugin-transform-destructuring", "foo"]
}

fn git_urls() -> Vec<&'static str> {
  vec![
    "git+ssh://git@github.com/npm/cli",
    "git@github.com:npm/cli.git",
    "github:uNetworking/uWebSockets.js",
  ]
}

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
  let cases: Vec<(&str, Option<Rc<Specifier2>>)> = vec![
    ("npm:foo", None),
    ("npm:foo@1.2.3", Some(Specifier2::new("npm:foo@~1.2.3"))),
    ("npm:@foo/bar@1.2.3", Some(Specifier2::new("npm:@foo/bar@~1.2.3"))),
    ("1.2.3", Some(Specifier2::new("~1.2.3"))),
    ("^1.2.3", Some(Specifier2::new("~1.2.3"))),
    ("~1.2.3", Some(Specifier2::new("~1.2.3"))),
    ("workspace:*", Some(Specifier2::new("workspace:~"))),
  ];
  for (value, expected) in cases {
    assert_eq!(Specifier2::new(value).with_range(&SemverRange::Patch), expected);
  }
}

#[test]
fn basic_semver_patch() {
  for prerelease in prereleases() {
    for (range, range_variant) in ranges() {
      for git_url in git_urls() {
        let value_without_protocol = format!("{range}1.2.3{prerelease}");
        let value = format!("{git_url}#{value_without_protocol}");
        // match Specifier2::new(&value, None) {
        //   Specifier2::Git(actual) => {
        //     assert_eq!(actual.raw, value);
        //     assert_eq!(actual.origin, git_url);
        //     let semver = actual.semver.unwrap();
        //     assert_eq!(semver.raw, value_without_protocol);
        //     assert_eq!(semver.variant, BasicSemverVariant::Patch);
        //     assert_eq!(semver.range_variant, range_variant);
        //     assert_eq!(semver.node_version.major, 1);
        //     assert_eq!(semver.node_version.minor, 2);
        //     assert_eq!(semver.node_version.patch, 3);
        //     assert_eq!(semver.node_version.pre_release.is_empty(), prerelease.is_empty());
        //   }
        //   _ => panic!("Expected Git"),
        // };
      }
      for npm_name in npm_names() {
        let value_without_protocol = format!("{range}1.2.3{prerelease}");
        let value = format!("npm:{npm_name}@{value_without_protocol}");
        // match Specifier2::new(&value) {
        //   Specifier2::Alias(actual) => {
        //     assert_eq!(actual.raw, value);
        //     assert_eq!(actual.name, npm_name);
        //     let semver = actual.semver.unwrap();
        //     assert_eq!(semver.raw, value_without_protocol);
        //     assert_eq!(semver.variant, BasicSemverVariant::Patch);
        //     assert_eq!(semver.range_variant, range_variant);
        //     assert_eq!(semver.node_version.major, 1);
        //     assert_eq!(semver.node_version.minor, 2);
        //     assert_eq!(semver.node_version.patch, 3);
        //     assert_eq!(semver.node_version.pre_release.is_empty(), prerelease.is_empty());
        //   }
        //   _ => panic!("Expected Alias"),
        // };
      }
      for protocol in protocols() {
        let without_protocol = format!("{range}1.2.3{prerelease}");
        let semver_number = format!("1.2.3{prerelease}");
        let value = format!("{protocol}{without_protocol}");
        // let local_version = BasicSemver::new("1.2.3");
        let specifier = Specifier2::new(&value);
        match &*specifier {
          Specifier2::Exact(actual) | Specifier2::Range(actual) | Specifier2::WorkspaceProtocol(actual) => {
            assert_eq!(*actual, value);
            assert_eq!(specifier.get_semver_number(), Some(semver_number.as_str()), "{value} -> {actual}");
            // assert_eq!(actual.variant, BasicSemverVariant::Patch);
            // assert_eq!(actual.range_variant, range_variant);
            // assert_eq!(actual.node_version.major, 1);
            // assert_eq!(actual.node_version.minor, 2);
            // assert_eq!(actual.node_version.patch, 3);
            // assert_eq!(actual.node_version.pre_release.is_empty(), prerelease.is_empty());
          }
          // Specifier2::WorkspaceProtocol(actual) => {
          //   assert_eq!(actual.raw, value);
          //   assert_eq!(actual.local_version.raw, "1.2.3");
          //   assert_eq!(actual.semver.raw, without_protocol);
          //   assert_eq!(actual.semver.variant, BasicSemverVariant::Patch);
          //   assert_eq!(actual.semver.range_variant, range_variant);
          //   assert_eq!(actual.semver.node_version.major, 1);
          //   assert_eq!(actual.semver.node_version.minor, 2);
          //   assert_eq!(actual.semver.node_version.patch, 3);
          //   assert_eq!(actual.semver.node_version.pre_release.is_empty(), prerelease.is_empty());
          // }
          _ => panic!("Expected BasicSemver or WorkspaceProtocol"),
        };
      }
    }
  }
}
