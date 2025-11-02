use {
  super::{BasicSemver, *},
  crate::specifier::BasicSemverVariant,
  std::cmp::Ordering,
};

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
fn basic_semver_patch() {
  for prerelease in prereleases() {
    for (range, range_variant) in ranges() {
      for git_url in git_urls() {
        let value_without_protocol = format!("{range}1.2.3{prerelease}");
        let value = format!("{git_url}#{value_without_protocol}");
        match Specifier::new(&value, None) {
          Specifier::Git(actual) => {
            assert_eq!(actual.raw, value);
            assert_eq!(actual.origin, git_url);
            let semver = actual.semver.unwrap();
            assert_eq!(semver.raw, value_without_protocol);
            assert_eq!(semver.variant, BasicSemverVariant::Patch);
            assert_eq!(semver.range_variant, range_variant);
            assert_eq!(semver.node_version.major, 1);
            assert_eq!(semver.node_version.minor, 2);
            assert_eq!(semver.node_version.patch, 3);
            assert_eq!(semver.node_version.pre_release.is_empty(), prerelease.is_empty());
          }
          _ => panic!("Expected Git"),
        };
      }
      for npm_name in npm_names() {
        let value_without_protocol = format!("{range}1.2.3{prerelease}");
        let value = format!("npm:{npm_name}@{value_without_protocol}");
        match Specifier::new(&value, None) {
          Specifier::Alias(actual) => {
            assert_eq!(actual.raw, value);
            assert_eq!(actual.name, npm_name);
            let semver = actual.semver.unwrap();
            assert_eq!(semver.raw, value_without_protocol);
            assert_eq!(semver.variant, BasicSemverVariant::Patch);
            assert_eq!(semver.range_variant, range_variant);
            assert_eq!(semver.node_version.major, 1);
            assert_eq!(semver.node_version.minor, 2);
            assert_eq!(semver.node_version.patch, 3);
            assert_eq!(semver.node_version.pre_release.is_empty(), prerelease.is_empty());
          }
          _ => panic!("Expected Alias"),
        };
      }
      for protocol in protocols() {
        let without_protocol = format!("{range}1.2.3{prerelease}");
        let value = format!("{protocol}{without_protocol}");
        let local_version = BasicSemver::new("1.2.3");
        match Specifier::new(&value, local_version.as_ref()) {
          Specifier::BasicSemver(actual) => {
            assert_eq!(actual.raw, value);
            assert_eq!(actual.variant, BasicSemverVariant::Patch);
            assert_eq!(actual.range_variant, range_variant);
            assert_eq!(actual.node_version.major, 1);
            assert_eq!(actual.node_version.minor, 2);
            assert_eq!(actual.node_version.patch, 3);
            assert_eq!(actual.node_version.pre_release.is_empty(), prerelease.is_empty());
          }
          Specifier::WorkspaceProtocol(actual) => {
            assert_eq!(actual.raw, value);
            assert_eq!(actual.local_version.raw, "1.2.3");
            assert_eq!(actual.semver.raw, without_protocol);
            assert_eq!(actual.semver.variant, BasicSemverVariant::Patch);
            assert_eq!(actual.semver.range_variant, range_variant);
            assert_eq!(actual.semver.node_version.major, 1);
            assert_eq!(actual.semver.node_version.minor, 2);
            assert_eq!(actual.semver.node_version.patch, 3);
            assert_eq!(actual.semver.node_version.pre_release.is_empty(), prerelease.is_empty());
          }
          _ => panic!("Expected BasicSemver or WorkspaceProtocol"),
        };
      }
    }
  }
}

#[test]
fn basic_semver_minor() {
  let cases: Vec<&str> = vec!["1.2", "1.2.x", "1.2.*"];
  for case in cases {
    for (range, range_variant) in ranges() {
      for git_url in git_urls() {
        let value_without_protocol = format!("{range}{case}");
        let value = format!("{git_url}#{value_without_protocol}");
        match Specifier::new(&value, None) {
          Specifier::Git(actual) => {
            assert_eq!(actual.raw, value);
            assert_eq!(actual.origin, git_url);
            let semver = actual.semver.unwrap();
            assert_eq!(semver.raw, format!("{range}1.2"));
            assert_eq!(semver.variant, BasicSemverVariant::Minor);
            assert_eq!(semver.range_variant, range_variant);
            assert_eq!(semver.node_version.major, 1);
            assert_eq!(semver.node_version.minor, 2);
            assert_eq!(semver.node_version.patch, get_huge());
            assert!(semver.node_version.pre_release.is_empty());
          }
          _ => panic!("Expected Git"),
        };
      }
      for npm_name in npm_names() {
        let value_without_protocol = format!("{range}{case}");
        let value = format!("npm:{npm_name}@{value_without_protocol}");
        match Specifier::new(&value, None) {
          Specifier::Alias(actual) => {
            assert_eq!(actual.raw, value);
            assert_eq!(actual.name, npm_name);
            let semver = actual.semver.unwrap();
            assert_eq!(semver.raw, format!("{range}1.2"));
            assert_eq!(semver.variant, BasicSemverVariant::Minor);
            assert_eq!(semver.range_variant, range_variant);
            assert_eq!(semver.node_version.major, 1);
            assert_eq!(semver.node_version.minor, 2);
            assert_eq!(semver.node_version.patch, get_huge());
            assert!(semver.node_version.pre_release.is_empty());
          }
          _ => panic!("Expected Alias"),
        }
      }
      for protocol in protocols() {
        let value_without_protocol = format!("{range}{case}");
        let value = format!("{protocol}{value_without_protocol}");
        let sanitised_without_protocol = format!("{range}1.2");
        let sanitised = format!("{protocol}{sanitised_without_protocol}");
        let local_version = if protocol.is_empty() { None } else { BasicSemver::new("1.2.3") };
        match Specifier::new(&value, local_version.as_ref()) {
          Specifier::BasicSemver(actual) => {
            assert_eq!(actual.raw, sanitised);
            assert_eq!(actual.variant, BasicSemverVariant::Minor);
            assert_eq!(actual.range_variant, range_variant);
            assert_eq!(actual.node_version.major, 1);
            assert_eq!(actual.node_version.minor, 2);
            assert_eq!(actual.node_version.patch, get_huge());
            assert!(actual.node_version.pre_release.is_empty());
          }
          Specifier::WorkspaceProtocol(actual) => {
            assert_eq!(actual.raw, sanitised);
            assert_eq!(actual.local_version.raw, "1.2.3");
            assert_eq!(actual.semver.raw, sanitised_without_protocol);
            assert_eq!(actual.semver.variant, BasicSemverVariant::Minor);
            assert_eq!(actual.semver.range_variant, range_variant);
            assert_eq!(actual.semver.node_version.major, 1);
            assert_eq!(actual.semver.node_version.minor, 2);
            assert_eq!(actual.semver.node_version.patch, get_huge());
            assert!(actual.semver.node_version.pre_release.is_empty());
          }
          _ => panic!("Expected BasicSemver or WorkspaceProtocol"),
        }
      }
    }
  }
}

#[test]
fn basic_semver_major() {
  let cases: Vec<&str> = vec!["1", "1.x", "1.*", "1.x.x", "1.*.*"];
  for case in cases {
    for (range, range_variant) in ranges() {
      for git_url in git_urls() {
        let value_without_protocol = format!("{range}{case}");
        let value = format!("{git_url}#{value_without_protocol}");
        match Specifier::new(&value, None) {
          Specifier::Git(actual) => {
            assert_eq!(actual.raw, value);
            assert_eq!(actual.origin, git_url);
            let semver = actual.semver.unwrap();
            assert_eq!(semver.raw, format!("{range}1"));
            assert_eq!(semver.variant, BasicSemverVariant::Major);
            assert_eq!(semver.range_variant, range_variant);
            assert_eq!(semver.node_version.major, 1);
            assert_eq!(semver.node_version.minor, get_huge());
            assert_eq!(semver.node_version.patch, get_huge());
            assert!(semver.node_version.pre_release.is_empty());
          }
          _ => panic!("Expected Git"),
        };
      }
      for npm_name in npm_names() {
        let value_without_protocol = format!("{range}{case}");
        let value = format!("npm:{npm_name}@{value_without_protocol}");
        match Specifier::new(&value, None) {
          Specifier::Alias(actual) => {
            assert_eq!(actual.raw, value);
            assert_eq!(actual.name, npm_name);
            let semver = actual.semver.unwrap();
            assert_eq!(semver.raw, format!("{range}1"));
            assert_eq!(semver.variant, BasicSemverVariant::Major);
            assert_eq!(semver.range_variant, range_variant);
            assert_eq!(semver.node_version.major, 1);
            assert_eq!(semver.node_version.minor, get_huge());
            assert_eq!(semver.node_version.patch, get_huge());
            assert!(semver.node_version.pre_release.is_empty());
          }
          _ => panic!("Expected Alias"),
        }
      }
      for protocol in protocols() {
        let value_without_protocol = format!("{range}{case}");
        let value = format!("{protocol}{value_without_protocol}");
        let sanitised_without_protocol = format!("{range}1");
        let sanitised = format!("{protocol}{sanitised_without_protocol}");
        let local_version = if protocol.is_empty() { None } else { BasicSemver::new("1.2.3") };
        match Specifier::new(&value, local_version.as_ref()) {
          Specifier::BasicSemver(actual) => {
            assert_eq!(actual.raw, sanitised);
            assert_eq!(actual.variant, BasicSemverVariant::Major);
            assert_eq!(actual.range_variant, range_variant);
            assert_eq!(actual.node_version.major, 1);
            assert_eq!(actual.node_version.minor, get_huge());
            assert_eq!(actual.node_version.patch, get_huge());
            assert!(actual.node_version.pre_release.is_empty());
          }
          Specifier::WorkspaceProtocol(actual) => {
            assert_eq!(actual.raw, sanitised);
            assert_eq!(actual.local_version.raw, "1.2.3");
            assert_eq!(actual.semver.raw, sanitised_without_protocol);
            assert_eq!(actual.semver.variant, BasicSemverVariant::Major);
            assert_eq!(actual.semver.range_variant, range_variant);
            assert_eq!(actual.semver.node_version.major, 1);
            assert_eq!(actual.semver.node_version.minor, get_huge());
            assert_eq!(actual.semver.node_version.patch, get_huge());
            assert!(actual.semver.node_version.pre_release.is_empty());
          }
          _ => panic!("Expected BasicSemver or WorkspaceProtocol"),
        }
      }
    }
  }
}

#[test]
fn basic_semver_latest() {
  let cases: Vec<&str> = vec!["*", "latest", "x"];
  for value in &cases {
    for npm_name in npm_names() {
      let value_with_protocol = format!("npm:{npm_name}@{value}");
      match Specifier::new(&value_with_protocol, None) {
        Specifier::Alias(actual) => {
          assert_eq!(actual.raw, value_with_protocol);
          assert_eq!(actual.name, npm_name);
          let semver = actual.semver.unwrap();
          assert_eq!(semver.raw, "*");
          assert_eq!(semver.variant, BasicSemverVariant::Latest);
          assert_eq!(semver.range_variant, SemverRange::Any);
          assert_eq!(semver.node_version.major, get_huge());
          assert_eq!(semver.node_version.minor, get_huge());
          assert_eq!(semver.node_version.patch, get_huge());
          assert!(semver.node_version.pre_release.is_empty());
        }
        _ => panic!("Expected Alias"),
      }
    }
    for protocol in protocols() {
      let value_with_protocol = format!("{protocol}{value}");
      let local_version = if protocol.is_empty() { None } else { BasicSemver::new("1.2.3") };
      match Specifier::new(&value_with_protocol, local_version.as_ref()) {
        Specifier::BasicSemver(actual) => {
          assert_eq!(actual.raw, "*");
          assert_eq!(actual.variant, BasicSemverVariant::Latest);
          assert_eq!(actual.range_variant, SemverRange::Any);
          assert_eq!(actual.node_version.major, get_huge());
          assert_eq!(actual.node_version.minor, get_huge());
          assert_eq!(actual.node_version.patch, get_huge());
          assert!(actual.node_version.pre_release.is_empty());
        }
        Specifier::WorkspaceProtocol(actual) => {
          assert_eq!(actual.raw, "workspace:*");
          assert_eq!(actual.local_version.raw, "1.2.3");
          assert_eq!(actual.semver.raw, "*");
          assert_eq!(actual.semver.variant, BasicSemverVariant::Latest);
          assert_eq!(actual.semver.range_variant, SemverRange::Any);
          assert_eq!(actual.semver.node_version.major, get_huge());
          assert_eq!(actual.semver.node_version.minor, get_huge());
          assert_eq!(actual.semver.node_version.patch, get_huge());
          assert!(actual.semver.node_version.pre_release.is_empty());
        }
        _ => panic!("Expected BasicSemver or WorkspaceProtocol"),
      }
    }
  }
}

#[test]
fn complex_semver() {
  let cases: Vec<&str> = vec![
    "1.x || >=2.5.0 || 5.0.0 - 7.2.3",
    "1.3.0 || <1.0.0 >2.0.0",
    "<1.0.0 >2.0.0",
    "<1.0.0 >=2.0.0",
    "<1.5.0 || >=1.6.0",
    "<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2",
    "<=1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2",
    ">1.0.0 <1.0.0",
    ">1.0.0 <=2.0.0",
    ">=2.3.4 || <=1.2.3",
  ];
  for value in cases {
    match Specifier::new(value, None) {
      Specifier::ComplexSemver(actual) => {
        assert_eq!(actual.raw, value);
      }
      _ => panic!("Expected ComplexSemver"),
    }
  }
}

#[test]
fn file_paths() {
  let cases: Vec<&str> = vec![
    "file:../path/to/foo",
    "file:./path/to/foo",
    "file:/../path/to/foo",
    "file:/./path/to/foo",
    "file:/.path/to/foo",
    "file://.",
    "file://../path/to/foo",
    "file://./path/to/foo",
    "file:////path/to/foo",
    "file:///path/to/foo",
    "file://path/to/foo",
    "file:/path/to/foo",
    "file:/~path/to/foo",
    "file:path/to/directory",
    "file:path/to/foo",
    "file:path/to/foo.tar.gz",
    "file:path/to/foo.tgz",
  ];
  for value in cases {
    match Specifier::new(value, None) {
      Specifier::File(actual) => {
        assert_eq!(actual.raw, value);
      }
      _ => panic!("Expected File"),
    }
  }
}

#[test]
fn urls() {
  let cases: Vec<&str> = vec![
    "http://insecure.com/foo.tgz",
    "https://server.com/foo.tgz",
    "https://server.com/foo.tgz",
  ];
  for value in cases {
    match Specifier::new(value, None) {
      Specifier::Url(actual) => {
        assert_eq!(actual.raw, value);
      }
      _ => panic!("Expected Url"),
    }
  }
}

#[test]
fn tags() {
  let cases: Vec<&str> = vec!["alpha", "beta", "canary", "next", "rc"];
  for value in cases {
    match Specifier::new(value, None) {
      Specifier::Tag(actual) => {
        assert_eq!(actual.raw, value);
      }
      _ => panic!("Expected Tag"),
    }
  }
}

#[test]
fn unsupported() {
  let cases: Vec<&str> = vec![
    "$typescript",
    "/path/to/foo",
    "/path/to/foo.tar",
    "/path/to/foo.tgz",
    "1.typo.wat",
    "=v1.2.3",
    "@f fo o al/ a d s ;f",
    "@foo/bar",
    "@foo/bar@",
    "git+file://path/to/repo#1.2.3",
    "not-git@hostname.com:some/repo",
    "user/foo#1234::path:dist",
    "user/foo#notimplemented:value",
    "user/foo#path:dist",
    "user/foo#semver:^1.2.3",
  ];
  for value in cases {
    match Specifier::new(value, None) {
      Specifier::Unsupported(actual) => {
        assert_eq!(actual.raw, value);
      }
      _ => panic!("Expected Unsupported"),
    }
  }
}

#[test]
#[ignore] // does not handle "semver:" but `Specifier` is being replaced by `Specifier`
          // which does
fn with_range_on_patch_variant() {
  let starts: Vec<&str> = vec!["", "npm:foo@", "workspace:", "git@github.com:npm/cli.git#semver:"];
  let values: Vec<&str> = vec!["^1.2.3", "~1.2.3", ">=1.2.3", ">1.2.3", "<=1.2.3", "<1.2.3", "1.2.3"];
  let changes: Vec<(SemverRange, &str)> = vec![
    // (SemverRange::Any, "*"),
    (SemverRange::Minor, "^1.2.3"),
    (SemverRange::Patch, "~1.2.3"),
    (SemverRange::Gte, ">=1.2.3"),
    (SemverRange::Gt, ">1.2.3"),
    (SemverRange::Lte, "<=1.2.3"),
    (SemverRange::Lt, "<1.2.3"),
    (SemverRange::Exact, "1.2.3"),
  ];
  for value in values {
    for start in starts.clone() {
      let local_version = if start == "workspace:" { BasicSemver::new("1.2.3") } else { None };
      for (range, expected) in &changes {
        let full_value = format!("{start}{value}");
        let full_expected = format!("{start}{expected}");
        assert_eq!(
          Specifier::new(&full_value, local_version.as_ref()).with_range(range),
          Specifier::new(&full_expected, local_version.as_ref()),
        );
      }
    }
  }
}

#[test]
#[ignore] // does not handle "semver:" but `Specifier` is being replaced by `Specifier`
          // which does
fn with_range_on_major_variant() {
  let starts: Vec<&str> = vec!["", "npm:foo@", "workspace:", "git@github.com:npm/cli.git#semver:"];
  let values: Vec<&str> = vec!["^1", "~1", ">=1", ">1", "<=1", "<1", "1"];
  let changes: Vec<(SemverRange, &str)> = vec![
    // (SemverRange::Any, "*"),
    (SemverRange::Minor, "^1"),
    (SemverRange::Patch, "~1"),
    (SemverRange::Gte, ">=1"),
    (SemverRange::Gt, ">1"),
    (SemverRange::Lte, "<=1"),
    (SemverRange::Lt, "<1"),
    (SemverRange::Exact, "1"),
  ];
  for value in values {
    for start in starts.clone() {
      let local_version = if start == "workspace:" { BasicSemver::new("1.2.3") } else { None };
      for (range, expected) in &changes {
        let full_value = format!("{start}{value}");
        let full_expected = format!("{start}{expected}");
        assert_eq!(
          Specifier::new(&full_value, local_version.as_ref()).with_range(range),
          Specifier::new(&full_expected, local_version.as_ref()),
        );
      }
    }
  }
}

#[test]
fn config_identifiers() {
  let cases: Vec<(&str, &str)> = vec![
    ("*", "latest"),
    ("1", "major"),
    ("1.2", "minor"),
    // exact semver versions
    ("0.0.0", "exact"),
    ("1.2.3-alpha", "exact"),
    ("1.2.3-rc.1", "exact"),
    ("1.2.3-alpha", "exact"),
    ("1.2.3-rc.0", "exact"),
    // complex semver queries
    ("1.3.0 || <1.0.0 >2.0.0", "range-complex"),
    ("<1.0.0 >2.0.0", "range-complex"),
    ("<1.0.0 >=2.0.0", "range-complex"),
    ("<1.5.0 || >=1.6.0", "range-complex"),
    ("<1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2", "range-complex"),
    ("<=1.6.16 || >=1.7.0 <1.7.11 || >=1.8.0 <1.8.2", "range-complex"),
    (">1.0.0 <1.0.0", "range-complex"),
    (">1.0.0 <=2.0.0", "range-complex"),
    (">=2.3.4 || <=1.2.3", "range-complex"),
    // workspace protocol
    ("workspace:*", "workspace-protocol"),
    ("workspace:^", "workspace-protocol"),
    ("workspace:~", "workspace-protocol"),
    // simple semver with a semver range
    ("<1.2.3-alpha", "range"),
    ("<1.2.3-rc.0", "range"),
    ("<=1.2.3-alpha", "range"),
    ("<=1.2.3-rc.0", "range"),
    (">1.2.3-alpha", "range"),
    (">1.2.3-rc.0", "range"),
    (">=1.2.3-alpha", "range"),
    (">=1.2.3-rc.0", "range"),
    ("^1.2.3", "range"),
    ("^1.2.3-alpha", "range"),
    ("^1.2.3-rc.0", "range"),
    ("~1.2.3-alpha", "range"),
    ("~1.2.3-rc.0", "range"),
    // unsupported
    ("$typescript", "unsupported"),
    ("/path/to/foo", "unsupported"),
    ("/path/to/foo.tar", "unsupported"),
    ("/path/to/foo.tgz", "unsupported"),
    ("1.typo.wat", "unsupported"),
    ("=v1.2.3", "unsupported"),
    ("@f fo o al/ a d s ;f", "unsupported"),
    ("@foo/bar", "unsupported"),
    ("@foo/bar@", "unsupported"),
    ("git+file://path/to/repo#1.2.3", "unsupported"),
    ("not-git@hostname.com:some/repo", "unsupported"),
    ("user/foo#1234::path:dist", "unsupported"),
    ("user/foo#notimplemented:value", "unsupported"),
    ("user/foo#path:dist", "unsupported"),
    ("user/foo#semver:^1.2.3", "unsupported"),
    // tags
    ("alpha", "tag"),
    ("beta", "tag"),
    ("canary", "tag"),
    // range major
    ("~1", "range-major"),
    // range minor
    ("<5.0", "range-minor"),
    ("<=5.0", "range-minor"),
    (">5.0", "range-minor"),
    (">=5.0", "range-minor"),
    ("^4.1", "range-minor"),
    ("~1.2", "range-minor"),
    ("~1.2", "range-minor"),
    // npm aliases
    ("npm:@minh.nguyen/plugin-transform-destructuring@^7.5.2", "alias"),
    ("npm:@types/selenium-webdriver@4.1.18", "alias"),
    ("npm:foo@1.2.3", "alias"),
    // file paths
    ("file:../path/to/foo", "file"),
    ("file:./path/to/foo", "file"),
    ("file:/../path/to/foo", "file"),
    ("file:/./path/to/foo", "file"),
    ("file:/.path/to/foo", "file"),
    ("file://.", "file"),
    ("file://../path/to/foo", "file"),
    ("file://./path/to/foo", "file"),
    ("file:////path/to/foo", "file"),
    ("file:///path/to/foo", "file"),
    ("file://path/to/foo", "file"),
    ("file:/path/to/foo", "file"),
    ("file:/~path/to/foo", "file"),
    ("file:path/to/directory", "file"),
    ("file:path/to/foo", "file"),
    ("file:path/to/foo.tar.gz", "file"),
    ("file:path/to/foo.tgz", "file"),
    // git urls
    ("git+https://github.com/user/foo", "git"),
    ("git+ssh://git@github.com/user/foo#1.2.3", "git"),
    ("git+ssh://git@github.com/user/foo#semver:^1.2.3", "git"),
    ("git+ssh://git@github.com:user/foo#semver:^1.2.3", "git"),
    ("git+ssh://git@notgithub.com/user/foo", "git"),
    ("git+ssh://git@notgithub.com/user/foo#1.2.3", "git"),
    ("git+ssh://git@notgithub.com/user/foo#semver:^1.2.3", "git"),
    ("git+ssh://git@notgithub.com:user/foo", "git"),
    ("git+ssh://git@notgithub.com:user/foo#1.2.3", "git"),
    ("git+ssh://git@notgithub.com:user/foo#semver:^1.2.3", "git"),
    ("git+ssh://github.com/user/foo", "git"),
    ("git+ssh://github.com/user/foo#1.2.3", "git"),
    ("git+ssh://github.com/user/foo#semver:^1.2.3", "git"),
    ("git+ssh://mydomain.com:1234#1.2.3", "git"),
    ("git+ssh://mydomain.com:1234/hey", "git"),
    ("git+ssh://mydomain.com:1234/hey#1.2.3", "git"),
    ("git+ssh://mydomain.com:foo", "git"),
    ("git+ssh://mydomain.com:foo#1.2.3", "git"),
    ("git+ssh://mydomain.com:foo/bar#1.2.3", "git"),
    ("git+ssh://notgithub.com/user/foo", "git"),
    ("git+ssh://notgithub.com/user/foo#1.2.3", "git"),
    ("git+ssh://notgithub.com/user/foo#semver:^1.2.3", "git"),
    ("git+ssh://username:password@mydomain.com:1234/hey#1.2.3", "git"),
    ("git://github.com/user/foo", "git"),
    ("git://github.com/user/foo#1.2.3", "git"),
    ("git://github.com/user/foo#semver:^1.2.3", "git"),
    ("git://notgithub.com/user/foo", "git"),
    ("git://notgithub.com/user/foo#1.2.3", "git"),
    ("git://notgithub.com/user/foo#semver:^1.2.3", "git"),
    // urls
    ("http://insecure.com/foo.tgz", "url"),
    ("https://server.com/foo.tgz", "url"),
    ("https://server.com/foo.tgz", "url"),
  ];
  for (value, expected) in cases {
    let local_version = BasicSemver::new("1.2.3");
    let actual = Specifier::new(value, local_version.as_ref());
    assert_eq!(
      actual.get_config_identifier(),
      expected,
      "{value} should have a config identifier of {expected} {actual:#?}"
    );
  }
}

#[test]
fn comparison() {
  let cases: Vec<(&str, &str, Ordering)> = vec![
    /* normal versions */
    ("0.0.0", "0.0.1", Ordering::Less),
    ("0.0.0", "0.1.0", Ordering::Less),
    ("0.0.0", "1.0.0", Ordering::Less),
    ("0.0.0", "0.0.0", Ordering::Equal),
    ("0.0.1", "0.0.0", Ordering::Greater),
    ("0.1.0", "0.0.0", Ordering::Greater),
    ("1.0.0", "0.0.0", Ordering::Greater),
    /* range versions where versions differ */
    ("0.0.0", "~0.0.1", Ordering::Less),
    ("0.0.0", "~0.1.0", Ordering::Less),
    ("0.0.0", "~1.0.0", Ordering::Less),
    ("0.0.1", "~0.0.0", Ordering::Greater),
    ("0.1.0", "~0.0.0", Ordering::Greater),
    ("1.0.0", "~0.0.0", Ordering::Greater),
    ("0.0.0", "^0.0", Ordering::Less),
    ("0", "~0.0", Ordering::Greater),
    ("0", "^0.0", Ordering::Greater),
    /* range greediness applies only when versions are equal */
    ("0.0.0", "~0.0.0", Ordering::Less),
    ("0.0.0", "~0.0", Ordering::Less),
    ("0.0.0", "^0.0.0", Ordering::Less),
    ("0.0", "~0.0", Ordering::Less),
    ("0.0", "^0.0", Ordering::Less),
    ("~0", "^0", Ordering::Less),
    ("0", "~0", Ordering::Less),
    ("0", "^0", Ordering::Less),
    ("0.0.0", ">0.0.0", Ordering::Less),
    ("0.0.0", ">=0.0.0", Ordering::Less),
    ("0.0.0", "<=0.0.0", Ordering::Greater),
    ("0.0.0", "<0.0.0", Ordering::Greater),
    ("0.0.0", "*", Ordering::Less),
    ("^0.0.0", "*", Ordering::Less),
    ("~0.0.0", "*", Ordering::Less),
    (">0.0.0", "*", Ordering::Less),
    (">=0.0.0", "*", Ordering::Less),
    ("<=0.0.0", "*", Ordering::Less),
    ("<0.0.0", "*", Ordering::Less),
    /* an empty or missing specifier is always bottom rank below anything valid */
    ("", "0.0.0", Ordering::Less),
    ("", "<0.0.0", Ordering::Less),
    /* stable should be favoured over tagged */
    ("0.0.0", "0.0.0-alpha", Ordering::Greater),
    /* equal tags should not affect comparison */
    ("0.0.0-alpha", "0.0.0-alpha", Ordering::Equal),
    ("0.0.0-alpha", "0.1.0-alpha", Ordering::Less),
    ("0.0.0-alpha", "1.0.0-alpha", Ordering::Less),
    ("0.0.0-alpha", "0.0.0-alpha", Ordering::Equal),
    ("0.0.1-alpha", "0.0.0-alpha", Ordering::Greater),
    ("0.1.0-alpha", "0.0.0-alpha", Ordering::Greater),
    ("1.0.0-alpha", "0.0.0-alpha", Ordering::Greater),
    /* preleases should matter when version is equal */
    ("0.0.0-rc.0.0.0", "0.0.0-rc.0.0.0", Ordering::Equal),
    ("0.0.0-rc.0.0.0", "0.0.0-rc.0.1.0", Ordering::Less),
    ("0.0.0-rc.0.0.0", "0.0.0-rc.1.0.0", Ordering::Less),
    ("0.0.0-rc.0.0.0", "0.0.0-rc.0.0.0", Ordering::Equal),
    ("0.0.0-rc.0.0.1", "0.0.0-rc.0.0.0", Ordering::Greater),
    ("0.0.0-rc.0.1.0", "0.0.0-rc.0.0.0", Ordering::Greater),
    ("0.0.0-rc.1.0.0", "0.0.0-rc.0.0.0", Ordering::Greater),
    /* preleases should not matter when version is greater */
    ("0.1.0-rc.0.0.0", "0.0.0-rc.0.1.0", Ordering::Greater),
    /* compares tags a-z */
    ("0.0.0-alpha", "0.0.0-alpha", Ordering::Equal),
    ("0.0.0-alpha", "0.0.0-beta", Ordering::Less),
    ("0.0.0-beta", "0.0.0-alpha", Ordering::Greater),
    /* range greediness is the same on prereleases */
    ("0.0.0-rc.0", "~0.0.1-rc.0", Ordering::Less),
    ("0.0.0-rc.0", "~0.1.0-rc.0", Ordering::Less),
    ("0.0.0-rc.0", "~1.0.0-rc.0", Ordering::Less),
    ("0.0.0-rc.0", "~0.0.0-rc.0", Ordering::Less),
    ("0.0.1-rc.0", "~0.0.0-rc.0", Ordering::Greater),
    ("0.1.0-rc.0", "~0.0.0-rc.0", Ordering::Greater),
    ("1.0.0-rc.0", "~0.0.0-rc.0", Ordering::Greater),
    ("0.0.0-rc.0", "~0.0.0-rc.0", Ordering::Less),
    ("0.0.0-rc.0", "^0.0.0-rc.0", Ordering::Less),
    ("0.0.0-rc.0", ">0.0.0-rc.0", Ordering::Less),
    ("0.0.0-rc.0", ">=0.0.0-rc.0", Ordering::Less),
    ("0.0.0-rc.0", "<=0.0.0-rc.0", Ordering::Greater),
    ("0.0.0-rc.0", "<0.0.0-rc.0", Ordering::Greater),
    /* workspace: protocol is ignored and the semver is used */
    ("workspace:*", "1.2.3", Ordering::Greater),
    ("workspace:0.0.0", "<0.0.0", Ordering::Greater),
    ("workspace:>0.0.0", "<0.0.0", Ordering::Greater),
    ("workspace:<=0.0.0", "<0.0.0", Ordering::Greater),
    ("workspace:0.0.0", "0.0.0", Ordering::Equal),
    ("workspace:~", "~1.2.3", Ordering::Equal),
    /* git with a semver tag is ignored and the semver is used */
    ("github:org/repo#v0.0.0", "github:org/repo#v0.0.0", Ordering::Equal),
    ("github:org/repo#v0.0.0", "github:org/repo#0.0.0", Ordering::Equal),
    ("github:org/repo#v0.0.0", "0.0.0", Ordering::Equal),
    ("github:org/repo#v0.0.1", "0.0.0", Ordering::Greater),
    ("github:org/repo#0.0.1", "0.0.0", Ordering::Greater),
  ];
  for (str_a, str_b, expected) in cases {
    let local_version = BasicSemver::new("1.2.3");
    let a = Specifier::new(str_a, local_version.as_ref());
    let b = Specifier::new(str_b, local_version.as_ref());
    let ordering = a.cmp(&b);
    assert_eq!(ordering, expected, "{str_a} should be {expected:?} than {str_b} {a:#?} {b:#?}");
  }
}

#[test]
fn sorting() {
  fn to_specifiers(specifiers: Vec<&str>) -> Vec<Specifier> {
    specifiers.iter().map(|r| Specifier::new(r, None)).collect()
  }
  let mut specifiers = to_specifiers(vec!["0.0.0", "<0.0.0", "*", ">0.0.0", ">=0.0.0", "<=0.0.0", "^0.0.0", "~0.0.0"]);
  let expected = to_specifiers(vec!["<0.0.0", "<=0.0.0", "0.0.0", "~0.0.0", "^0.0.0", ">=0.0.0", ">0.0.0", "*"]);
  specifiers.sort();
  assert_eq!(specifiers, expected, "{specifiers:?}, {expected:?}");
}

#[test]
fn sorting_aliases() {
  fn to_specifiers(specifiers: Vec<&str>) -> Vec<Specifier> {
    specifiers
      .iter()
      .map(|r| Specifier::new(&format!("npm:@jsr/std__fmt@{r}"), None))
      .collect()
  }
  let mut specifiers = to_specifiers(vec!["0.0.0", "<0.0.0", "*", ">0.0.0", ">=0.0.0", "<=0.0.0", "^0.0.0", "~0.0.0"]);
  let expected = to_specifiers(vec!["<0.0.0", "<=0.0.0", "0.0.0", "~0.0.0", "^0.0.0", ">=0.0.0", ">0.0.0", "*"]);
  specifiers.sort();
  assert_eq!(specifiers, expected, "{specifiers:?}, {expected:?}");
}

#[test]
fn satisfies_all() {
  let cases: Vec<(&str, Vec<&str>, bool)> = vec![
    ("*", vec!["1.4.2"], true),
    ("^1.4.2", vec!["1.4.2"], true),
    ("1.4.2", vec!["1.4.2"], true),
    (">1.4.2", vec!["1.4.2"], false),
    (">=1.4.2", vec!["1.4.2"], true),
    ("<1.4.2", vec!["1.4.2"], false),
    ("<=1.4.2", vec!["1.4.2"], true),
    ("~1.4.2", vec!["1.4.2"], true),
    ("^1.0.0", vec!["1.4.2"], true),
    ("~1.0.0", vec!["1.4.2"], false),
    ("", vec!["1.4.2"], false),
    ("~1.4.2 || ^1.4.2", vec!["1.4.2"], true),
    ("~1.0.0 || ^1.0.0", vec!["1.4.2"], true),
  ];
  for (value, others, expected) in cases {
    let spec = Specifier::new(value, None);
    let other_specs: Vec<Specifier> = others.iter().map(|r| Specifier::new(r, None)).collect();
    let refs_to_other_specs: Vec<&Specifier> = other_specs.iter().collect();
    assert_eq!(
      spec.satisfies_all(refs_to_other_specs),
      expected,
      "'{value}'.satisfies_all({others:?}) should be {expected}"
    );
  }
}

#[test]
fn issue_213_git_tags_starting_with_v() {
  let value = "github:uNetworking/uWebSockets.js#v20.43.0";
  match Specifier::new(value, None) {
    Specifier::Git(actual) => {
      assert_eq!(actual.raw, value);
      assert_eq!(actual.origin, "github:uNetworking/uWebSockets.js");
      let semver = actual.semver.as_ref().unwrap();
      assert_eq!(semver.raw, "20.43.0");
      assert_eq!(semver.variant, BasicSemverVariant::Patch);
      assert_eq!(semver.range_variant, SemverRange::Exact);
      assert_eq!(semver.node_version.major, 20);
      assert_eq!(semver.node_version.minor, 43);
      assert_eq!(semver.node_version.patch, 0);
      assert!(semver.node_version.pre_release.is_empty());

      let next_semver = BasicSemver::new("1.2.3").unwrap();
      let edited = actual.with_semver(&next_semver);
      assert_eq!(edited.raw, "github:uNetworking/uWebSockets.js#v1.2.3");
      assert_eq!(edited.origin, "github:uNetworking/uWebSockets.js");
      let semver = edited.semver.unwrap();
      assert_eq!(semver.raw, "1.2.3");
      assert_eq!(semver.variant, BasicSemverVariant::Patch);
      assert_eq!(semver.range_variant, SemverRange::Exact);
      assert_eq!(semver.node_version.major, 1);
      assert_eq!(semver.node_version.minor, 2);
      assert_eq!(semver.node_version.patch, 3);
      assert!(semver.node_version.pre_release.is_empty());
    }
    _ => panic!("Expected Git"),
  };
}
