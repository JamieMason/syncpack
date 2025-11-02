use crate::specifier::Specifier;

#[test]
fn returns_none_for_non_semver() {
  let new_version = node_semver::Version::parse("2.3.4").unwrap();
  let cases: Vec<&str> = vec![
    "*",
    "latest",
    "next",
    "file:../foo",
    "git+ssh://git@github.com/npm/cli",
    "https://example.com/package.tgz",
    ">=1.2.3 <2.0.0",
    "npm:foo",
  ];
  for value in cases {
    assert_eq!(Specifier::new(value).with_node_version(&new_version), None);
  }
}

#[test]
fn on_exact() {
  let cases: Vec<(&str, &str)> = vec![("1.2.3", "2.3.4"), ("1.2.3-alpha", "2.3.4-beta.1"), ("1.2.3-alpha.0", "2.3.4")];
  for (old, new) in cases {
    let new_version = node_semver::Version::parse(new).unwrap();
    assert_eq!(Specifier::new(old).with_node_version(&new_version), Some(Specifier::new(new)));
  }
}

#[test]
fn on_major() {
  let cases: Vec<(&str, &str, &str)> = vec![
    ("1", "2.3.4", "2"),
    ("1", "2.999999.999999", "2"),
    ("^1", "2.3.4", "^2"),
    ("~1", "2.999999.999999", "~2"),
    (">=1", "2.3.4", ">=2"),
  ];
  for (old, new_version_str, expected) in cases {
    let new_version = node_semver::Version::parse(new_version_str).unwrap();
    assert_eq!(Specifier::new(old).with_node_version(&new_version), Some(Specifier::new(expected)));
  }
}

#[test]
fn on_minor() {
  let cases: Vec<(&str, &str, &str)> = vec![
    ("1.2", "2.3.4", "2.3"),
    ("1.2", "2.3.999999", "2.3"),
    ("^1.2", "2.3.4", "^2.3"),
    ("~1.2", "2.3.999999", "~2.3"),
    (">=1.2", "2.3.4", ">=2.3"),
  ];
  for (old, new_version_str, expected) in cases {
    let new_version = node_semver::Version::parse(new_version_str).unwrap();
    assert_eq!(Specifier::new(old).with_node_version(&new_version), Some(Specifier::new(expected)));
  }
}

#[test]
fn on_range() {
  let cases: Vec<(&str, &str, &str)> = vec![
    ("^1.2.3", "2.3.4", "^2.3.4"),
    ("~1.2.3", "2.3.4", "~2.3.4"),
    (">=1.2.3", "2.3.4", ">=2.3.4"),
    (">1.2.3", "2.3.4", ">2.3.4"),
    ("<=1.2.3", "2.3.4", "<=2.3.4"),
    ("<1.2.3", "2.3.4", "<2.3.4"),
    ("^1.2.3-alpha", "2.3.4-beta.1", "^2.3.4-beta.1"),
    ("~1.2.3", "2.3.4-rc.2", "~2.3.4-rc.2"),
  ];
  for (old, new_version_str, expected) in cases {
    let new_version = node_semver::Version::parse(new_version_str).unwrap();
    assert_eq!(Specifier::new(old).with_node_version(&new_version), Some(Specifier::new(expected)));
  }
}

#[test]
fn on_workspace_protocol() {
  let cases: Vec<(&str, &str, &str)> = vec![
    ("workspace:1.2.3", "2.3.4", "workspace:2.3.4"),
    ("workspace:^1.2.3", "2.3.4", "workspace:^2.3.4"),
    ("workspace:~1.2.3", "2.3.4", "workspace:~2.3.4"),
    ("workspace:>=1.2.3", "2.3.4", "workspace:>=2.3.4"),
    ("workspace:1.2.3-alpha", "2.3.4-beta", "workspace:2.3.4-beta"),
  ];
  for (old, new_version_str, expected) in cases {
    let new_version = node_semver::Version::parse(new_version_str).unwrap();
    assert_eq!(Specifier::new(old).with_node_version(&new_version), Some(Specifier::new(expected)));
  }
}

#[test]
fn on_workspace_protocol_range_only() {
  let new_version = node_semver::Version::parse("2.3.4").unwrap();
  let cases: Vec<&str> = vec!["workspace:*", "workspace:^", "workspace:~"];
  for value in cases {
    assert_eq!(Specifier::new(value).with_node_version(&new_version), None);
  }
}

#[test]
fn on_alias() {
  let cases: Vec<(&str, &str, &str)> = vec![
    ("npm:foo@1.2.3", "2.3.4", "npm:foo@2.3.4"),
    ("npm:@scope/pkg@1.2.3", "2.3.4", "npm:@scope/pkg@2.3.4"),
    ("npm:foo@^1.2.3", "2.3.4", "npm:foo@^2.3.4"),
    ("npm:foo@~1.2.3", "2.3.4", "npm:foo@~2.3.4"),
    ("npm:foo@>=1.2.3", "2.3.4", "npm:foo@>=2.3.4"),
    ("npm:foo@1.2.3-alpha", "2.3.4-beta", "npm:foo@2.3.4-beta"),
  ];
  for (old, new_version_str, expected) in cases {
    let new_version = node_semver::Version::parse(new_version_str).unwrap();
    assert_eq!(Specifier::new(old).with_node_version(&new_version), Some(Specifier::new(expected)));
  }
}

#[test]
fn on_alias_without_version() {
  let new_version = node_semver::Version::parse("2.3.4").unwrap();
  assert_eq!(Specifier::new("npm:foo").with_node_version(&new_version), None);
}

#[test]
fn on_git() {
  let cases: Vec<(&str, &str, &str)> = vec![
    ("git@github.com:npm/cli.git#1.2.3", "2.3.4", "git@github.com:npm/cli.git#2.3.4"),
    ("git@github.com:npm/cli.git#^1.2.3", "2.3.4", "git@github.com:npm/cli.git#^2.3.4"),
    ("git@github.com:npm/cli.git#~1.2.3", "2.3.4", "git@github.com:npm/cli.git#~2.3.4"),
    (
      "git+ssh://git@github.com/npm/cli#>=1.2.3",
      "2.3.4",
      "git+ssh://git@github.com/npm/cli#>=2.3.4",
    ),
    ("github:user/repo#1.2.3-alpha", "2.3.4-beta", "github:user/repo#2.3.4-beta"),
  ];
  for (old, new_version_str, expected) in cases {
    let new_version = node_semver::Version::parse(new_version_str).unwrap();
    assert_eq!(Specifier::new(old).with_node_version(&new_version), Some(Specifier::new(expected)));
  }
}

#[test]
fn on_git_without_semver_tag() {
  let new_version = node_semver::Version::parse("2.3.4").unwrap();
  let cases: Vec<&str> = vec![
    "git@github.com:npm/cli.git",
    "git@github.com:npm/cli.git#main",
    "git+ssh://git@github.com/npm/cli#feature-branch",
  ];
  for value in cases {
    assert_eq!(Specifier::new(value).with_node_version(&new_version), None);
  }
}

#[test]
fn preserves_prerelease() {
  let cases: Vec<(&str, &str, &str)> = vec![
    ("1.2.3", "2.3.4-alpha.1", "2.3.4-alpha.1"),
    ("^1.2.3", "2.3.4-beta", "^2.3.4-beta"),
    ("workspace:~1.2.3", "2.3.4-rc.2", "workspace:~2.3.4-rc.2"),
    ("npm:foo@1.2.3", "2.3.4-next.5", "npm:foo@2.3.4-next.5"),
  ];
  for (old, new_version_str, expected) in cases {
    let new_version = node_semver::Version::parse(new_version_str).unwrap();
    assert_eq!(Specifier::new(old).with_node_version(&new_version), Some(Specifier::new(expected)));
  }
}

#[test]
fn omits_huge_values() {
  let cases: Vec<(&str, &str, &str)> = vec![
    ("1", "2.999999.999999", "2"),
    ("1.2", "2.3.999999", "2.3"),
    ("^1", "2.999999.999999", "^2"),
    ("^1.2", "2.3.999999", "^2.3"),
    ("workspace:~1", "2.999999.999999", "workspace:~2"),
    ("npm:foo@1.2", "2.3.999999", "npm:foo@2.3"),
  ];
  for (old, new_version_str, expected) in cases {
    let new_version = node_semver::Version::parse(new_version_str).unwrap();
    assert_eq!(Specifier::new(old).with_node_version(&new_version), Some(Specifier::new(expected)));
  }
}

#[test]
fn comprehensive_range_preservation() {
  let prefixes: Vec<(&str, &str)> = vec![
    ("", ""),
    ("npm:foo@", "npm:foo@"),
    ("workspace:", "workspace:"),
    ("git@github.com:npm/cli.git#", "git@github.com:npm/cli.git#"),
  ];
  let ranges: Vec<(&str, &str)> = vec![("", ""), ("^", "^"), ("~", "~"), (">=", ">="), (">", ">"), ("<=", "<="), ("<", "<")];
  let new_version = node_semver::Version::parse("2.3.4").unwrap();

  for (old_prefix, new_prefix) in &prefixes {
    for (range, expected_range) in &ranges {
      let old_value = format!("{old_prefix}{range}1.2.3");
      let expected_value = format!("{new_prefix}{expected_range}2.3.4");
      let result = Specifier::new(&old_value).with_node_version(&new_version);
      assert_eq!(result, Some(Specifier::new(&expected_value)));
    }
  }
}
