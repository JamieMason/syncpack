use crate::specifier::Specifier;

#[test]
fn both_stable_versions() {
  let cases: Vec<(&str, &str)> = vec![
    ("1.2.3", "1.2.3"),
    ("1.2.3", "2.0.0"),
    ("1.0.0", "99.99.99"),
    ("^1.2.3", "^2.0.0"),
    ("~1.2.3", "~2.0.0"),
    (">=1.2.3", ">=2.0.0"),
    (">1.2.3", ">2.0.0"),
    ("<=1.2.3", "<=2.0.0"),
    ("<1.2.3", "<2.0.0"),
    ("1.2.3", "^2.0.0"),
    ("~1.2.3", ">=2.0.0"),
  ];
  for (a, b) in cases {
    assert!(
      Specifier::new(a).has_same_release_channel_as(&Specifier::new(b)),
      "Expected {a} and {b} to have same release channel (both stable)"
    );
  }
}

#[test]
fn both_same_prerelease_channel() {
  let cases: Vec<(&str, &str)> = vec![
    // Both alpha
    ("1.2.3-alpha.1", "1.2.3-alpha.1"),
    ("1.2.3-alpha.1", "2.0.0-alpha.5"),
    ("1.0.0-alpha", "99.0.0-alpha.999"),
    ("^1.2.3-alpha.0", "^2.0.0-alpha.1"),
    ("~1.2.3-alpha.1", "~2.0.0-alpha.2"),
    (">=1.2.3-alpha", ">=2.0.0-alpha.0"),
    // Both beta
    ("1.2.3-beta.1", "1.2.3-beta.2"),
    ("1.2.3-beta", "2.0.0-beta.5"),
    ("^1.2.3-beta.0", "^2.0.0-beta.1"),
    // Both rc
    ("1.2.3-rc.1", "1.2.3-rc.2"),
    ("1.2.3-rc", "2.0.0-rc.5"),
    ("~1.2.3-rc.0", "~2.0.0-rc.1"),
    // Both next
    ("1.2.3-next.1", "2.0.0-next.2"),
    ("^1.2.3-next", "^2.0.0-next.0"),
    // Both canary
    ("1.2.3-canary.1", "2.0.0-canary.2"),
    // Both dev
    ("1.2.3-dev.1", "2.0.0-dev.2"),
  ];
  for (a, b) in cases {
    assert!(
      Specifier::new(a).has_same_release_channel_as(&Specifier::new(b)),
      "Expected {a} and {b} to have same release channel"
    );
  }
}

#[test]
fn different_prerelease_channels() {
  let cases: Vec<(&str, &str)> = vec![
    ("1.2.3-alpha.1", "1.2.3-beta.1"),
    ("1.2.3-alpha", "1.2.3-rc"),
    ("1.2.3-beta.1", "1.2.3-rc.1"),
    ("1.2.3-next.1", "1.2.3-canary.1"),
    ("1.2.3-dev", "1.2.3-alpha"),
    ("^1.2.3-alpha.1", "^1.2.3-beta.1"),
    ("~1.2.3-rc.1", "~1.2.3-next.1"),
    (">=1.2.3-alpha", ">=1.2.3-beta"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).has_same_release_channel_as(&Specifier::new(b)),
      "Expected {a} and {b} to have different release channels"
    );
  }
}

#[test]
fn stable_vs_prerelease() {
  let cases: Vec<(&str, &str)> = vec![
    ("1.2.3", "1.2.3-alpha.1"),
    ("1.2.3", "1.2.3-beta"),
    ("2.0.0", "1.0.0-rc.1"),
    ("^1.2.3", "^1.2.3-alpha"),
    ("~1.2.3", "~2.0.0-beta.1"),
    (">=1.2.3", ">=1.2.3-next"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).has_same_release_channel_as(&Specifier::new(b)),
      "Expected {a} (stable) and {b} (prerelease) to have different release channels"
    );
  }
}

#[test]
fn with_workspace_protocol() {
  let cases_same: Vec<(&str, &str)> = vec![
    // Both stable
    ("workspace:1.2.3", "workspace:2.0.0"),
    ("workspace:^1.2.3", "workspace:~2.0.0"),
    ("workspace:1.2.3", "1.2.3"),
    ("workspace:^1.2.3", "^2.0.0"),
    // Both same prerelease
    ("workspace:1.2.3-alpha.1", "workspace:2.0.0-alpha.2"),
    ("workspace:^1.2.3-beta", "workspace:~2.0.0-beta.1"),
    ("workspace:1.2.3-alpha", "1.2.3-alpha.5"),
  ];
  for (a, b) in cases_same {
    assert!(
      Specifier::new(a).has_same_release_channel_as(&Specifier::new(b)),
      "Expected {a} and {b} to have same release channel"
    );
  }

  let cases_different: Vec<(&str, &str)> = vec![
    ("workspace:1.2.3", "workspace:1.2.3-alpha"),
    ("workspace:1.2.3-alpha", "workspace:1.2.3-beta"),
    ("workspace:^1.2.3-rc", "workspace:^1.2.3-next"),
    ("workspace:1.2.3-alpha", "1.2.3-beta"),
  ];
  for (a, b) in cases_different {
    assert!(
      !Specifier::new(a).has_same_release_channel_as(&Specifier::new(b)),
      "Expected {a} and {b} to have different release channels"
    );
  }
}

#[test]
fn with_npm_alias() {
  let cases_same: Vec<(&str, &str)> = vec![
    // Both stable
    ("npm:foo@1.2.3", "npm:bar@2.0.0"),
    ("npm:foo@^1.2.3", "npm:foo@~2.0.0"),
    ("npm:@scope/pkg@1.2.3", "npm:@other/pkg@2.0.0"),
    ("npm:foo@1.2.3", "1.2.3"),
    ("npm:foo@^1.2.3", "^2.0.0"),
    // Both same prerelease
    ("npm:foo@1.2.3-alpha.1", "npm:foo@2.0.0-alpha.2"),
    ("npm:foo@^1.2.3-beta", "npm:bar@~2.0.0-beta.1"),
    ("npm:@scope/pkg@1.2.3-rc", "npm:@other/pkg@1.2.3-rc.5"),
    ("npm:foo@1.2.3-alpha", "1.2.3-alpha.5"),
  ];
  for (a, b) in cases_same {
    assert!(
      Specifier::new(a).has_same_release_channel_as(&Specifier::new(b)),
      "Expected {a} and {b} to have same release channel"
    );
  }

  let cases_different: Vec<(&str, &str)> = vec![
    ("npm:foo@1.2.3", "npm:foo@1.2.3-alpha"),
    ("npm:foo@1.2.3-alpha", "npm:foo@1.2.3-beta"),
    ("npm:@scope/pkg@^1.2.3-rc", "npm:@scope/pkg@^1.2.3-next"),
    ("npm:foo@1.2.3-alpha", "1.2.3-beta"),
  ];
  for (a, b) in cases_different {
    assert!(
      !Specifier::new(a).has_same_release_channel_as(&Specifier::new(b)),
      "Expected {a} and {b} to have different release channels"
    );
  }
}

#[test]
fn with_git_urls() {
  let cases_same: Vec<(&str, &str)> = vec![
    // Both stable
    ("git@github.com:npm/cli.git#1.2.3", "git@github.com:npm/cli.git#2.0.0"),
    ("git+ssh://git@github.com/npm/cli#^1.2.3", "git+ssh://git@github.com/npm/cli#~2.0.0"),
    ("github:user/repo#1.2.3", "github:other/repo#2.0.0"),
    ("git@github.com:npm/cli.git#1.2.3", "1.2.3"),
    // Both same prerelease
    (
      "git@github.com:npm/cli.git#1.2.3-alpha.1",
      "git@github.com:npm/cli.git#2.0.0-alpha.2",
    ),
    (
      "git+ssh://git@github.com/npm/cli#^1.2.3-beta",
      "git+ssh://git@github.com/npm/cli#~2.0.0-beta.1",
    ),
    ("github:user/repo#1.2.3-rc", "github:user/repo#1.2.3-rc.5"),
    ("git@github.com:npm/cli.git#1.2.3-alpha", "1.2.3-alpha.5"),
  ];
  for (a, b) in cases_same {
    assert!(
      Specifier::new(a).has_same_release_channel_as(&Specifier::new(b)),
      "Expected {a} and {b} to have same release channel"
    );
  }

  let cases_different: Vec<(&str, &str)> = vec![
    ("git@github.com:npm/cli.git#1.2.3", "git@github.com:npm/cli.git#1.2.3-alpha"),
    ("git@github.com:npm/cli.git#1.2.3-alpha", "git@github.com:npm/cli.git#1.2.3-beta"),
    ("github:user/repo#^1.2.3-rc", "github:user/repo#^1.2.3-next"),
    ("git+ssh://git@github.com/npm/cli#1.2.3-alpha", "1.2.3-beta"),
  ];
  for (a, b) in cases_different {
    assert!(
      !Specifier::new(a).has_same_release_channel_as(&Specifier::new(b)),
      "Expected {a} and {b} to have different release channels"
    );
  }
}

#[test]
fn specifiers_without_node_version() {
  let cases: Vec<(&str, &str)> = vec![
    // Tags
    ("latest", "next"),
    ("latest", "canary"),
    ("next", "beta"),
    // URLs
    ("https://example.com/package.tgz", "https://other.com/pkg.tgz"),
    ("http://example.com/package.tgz", "http://other.com/pkg.tgz"),
    // File protocols
    ("file:../foo", "file:../bar"),
    ("file:./local", "file:./other"),
    // Complex semver ranges
    (">=1.2.3 <2.0.0", ">=2.0.0 <3.0.0"),
    ("^1.2.3 || ^2.0.0", "^3.0.0 || ^4.0.0"),
    // Git without semver tags
    ("git@github.com:npm/cli.git", "git@github.com:npm/cli.git#main"),
    (
      "git+ssh://git@github.com/npm/cli#feature",
      "git+ssh://git@github.com/npm/cli#develop",
    ),
    // Npm alias without version
    ("npm:foo", "npm:bar"),
    // Workspace protocol without version
    ("workspace:*", "workspace:^"),
    ("workspace:~", "workspace:*"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).has_same_release_channel_as(&Specifier::new(b)),
      "Expected {a} and {b} to return false (no node_version available)"
    );
  }
}

#[test]
fn one_with_version_one_without() {
  let cases: Vec<(&str, &str)> = vec![
    ("1.2.3", "latest"),
    ("1.2.3-alpha", "next"),
    ("^1.2.3", "https://example.com/package.tgz"),
    ("~1.2.3-beta", "file:../foo"),
    (">=1.2.3", ">=1.2.3 <2.0.0"),
    ("workspace:1.2.3", "workspace:*"),
    ("npm:foo@1.2.3", "npm:foo"),
    ("git@github.com:npm/cli.git#1.2.3", "git@github.com:npm/cli.git"),
    ("1.2.3", "git@github.com:npm/cli.git#main"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).has_same_release_channel_as(&Specifier::new(b)),
      "Expected {a} (with version) and {b} (without version) to return false"
    );
    // Test in reverse order too
    assert!(
      !Specifier::new(b).has_same_release_channel_as(&Specifier::new(a)),
      "Expected {b} (without version) and {a} (with version) to return false"
    );
  }
}

#[test]
fn multi_segment_prerelease_identifiers() {
  let cases_same: Vec<(&str, &str)> = vec![
    // Only first segment matters
    ("1.2.3-alpha.1", "1.2.3-alpha.2"),
    ("1.2.3-alpha.1.2", "1.2.3-alpha.5.9"),
    ("1.2.3-beta.x.y.z", "1.2.3-beta.a.b.c"),
    ("1.2.3-rc.1.2.3", "2.0.0-rc.9.8.7"),
  ];
  for (a, b) in cases_same {
    assert!(
      Specifier::new(a).has_same_release_channel_as(&Specifier::new(b)),
      "Expected {a} and {b} to have same release channel (first prerelease segment matches)"
    );
  }

  let cases_different: Vec<(&str, &str)> = vec![("1.2.3-alpha.1.2", "1.2.3-beta.1.2"), ("1.2.3-rc.x.y", "1.2.3-next.x.y")];
  for (a, b) in cases_different {
    assert!(
      !Specifier::new(a).has_same_release_channel_as(&Specifier::new(b)),
      "Expected {a} and {b} to have different release channels (first prerelease segment differs)"
    );
  }
}

#[test]
fn major_and_minor_variants() {
  let cases_same: Vec<(&str, &str)> = vec![
    // Both stable
    ("1", "2"),
    ("1.2", "2.3"),
    ("^1", "~2"),
    ("^1.2", "~2.3"),
    ("1", "1.2.3"),
    ("1.2", "1.2.3"),
  ];
  for (a, b) in cases_same {
    assert!(
      Specifier::new(a).has_same_release_channel_as(&Specifier::new(b)),
      "Expected {a} and {b} to have same release channel (both stable)"
    );
  }
}

#[test]
fn self_comparison() {
  let cases: Vec<&str> = vec![
    "1.2.3",
    "^1.2.3",
    "1.2.3-alpha.1",
    "~1.2.3-beta",
    "workspace:1.2.3",
    "npm:foo@1.2.3",
    "git@github.com:npm/cli.git#1.2.3",
  ];
  for value in cases {
    let spec = Specifier::new(value);
    assert!(
      spec.has_same_release_channel_as(&spec),
      "Expected {value} to have same release channel as itself"
    );
  }
}
