use crate::specifier::Specifier;

#[test]
fn older_exact_versions() {
  let cases: Vec<(&str, &str)> = vec![
    // Patch differences
    ("1.0.0", "1.0.1"),
    ("1.0.0", "1.0.2"),
    ("1.4.0", "1.4.1"),
    // Minor differences
    ("1.0.0", "1.1.0"),
    ("1.0.0", "1.2.0"),
    ("1.4.0", "1.5.0"),
    // Major differences
    ("1.0.0", "2.0.0"),
    ("1.0.0", "3.0.0"),
    ("2.4.1", "3.0.0"),
    // Multiple component differences
    ("1.0.0", "1.1.1"),
    ("1.0.0", "2.1.0"),
    ("1.0.0", "2.1.1"),
    ("1.4.5", "2.0.0"),
    // Large version jumps
    ("1.0.0", "10.0.0"),
    ("1.0.0", "100.0.0"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than(&Specifier::new(newer)),
      "Expected {older} to be older than {newer}"
    );
  }
}

#[test]
fn not_older_exact_versions() {
  let cases: Vec<(&str, &str)> = vec![
    // Same versions
    ("1.0.0", "1.0.0"),
    ("1.4.1", "1.4.1"),
    ("2.0.0", "2.0.0"),
    // Reversed (newer to older)
    ("1.0.1", "1.0.0"),
    ("1.1.0", "1.0.0"),
    ("2.0.0", "1.0.0"),
    ("2.0.0", "1.9.9"),
    ("10.0.0", "9.0.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b}"
    );
  }
}

#[test]
fn older_with_ranges() {
  let cases: Vec<(&str, &str)> = vec![
    // Caret ranges
    ("^1.0.0", "^1.0.1"),
    ("^1.0.0", "^1.1.0"),
    ("^1.0.0", "^2.0.0"),
    // Tilde ranges
    ("~1.0.0", "~1.0.1"),
    ("~1.0.0", "~1.1.0"),
    ("~1.0.0", "~2.0.0"),
    // Mixed exact and ranges
    ("1.0.0", "^1.0.1"),
    ("1.0.0", "~1.0.1"),
    ("^1.0.0", "1.0.1"),
    ("~1.0.0", "1.0.1"),
    // Comparison ranges (>=, >, <=, <)
    (">=1.0.0", ">=1.0.1"),
    (">=1.0.0", ">=2.0.0"),
    (">1.0.0", ">1.0.1"),
    ("<=1.0.0", "<=2.0.0"),
    ("<1.0.0", "<2.0.0"),
    // Mixed comparison and exact
    (">=1.0.0", "1.0.1"),
    ("1.0.0", ">=1.0.1"),
    (">1.0.0", "2.0.0"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than(&Specifier::new(newer)),
      "Expected {older} to be older than {newer}"
    );
  }
}

#[test]
fn not_older_with_ranges() {
  let cases: Vec<(&str, &str)> = vec![
    // Same versions
    ("^1.0.0", "^1.0.0"),
    ("~1.0.0", "~1.0.0"),
    (">=1.0.0", ">=1.0.0"),
    // Reversed
    ("^1.0.1", "^1.0.0"),
    ("~2.0.0", "~1.0.0"),
    (">=2.0.0", ">=1.0.0"),
    (">2.0.0", ">1.0.0"),
    ("1.0.1", "^1.0.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b}"
    );
  }
}

#[test]
fn older_with_prerelease() {
  let cases: Vec<(&str, &str)> = vec![
    // Prerelease vs release
    ("1.0.0-alpha.1", "1.0.0"),
    ("1.0.0-beta.1", "1.0.0"),
    ("1.0.0-rc.1", "1.0.0"),
    // Different prereleases
    ("1.0.0-alpha.1", "1.0.0-alpha.2"),
    ("1.0.0-alpha.1", "1.0.0-beta.1"),
    ("1.0.0-beta.1", "1.0.0-rc.1"),
    // Prerelease with version differences
    ("1.0.0-alpha.1", "1.0.1"),
    ("1.0.0-alpha.1", "1.1.0"),
    ("1.0.0-alpha.1", "2.0.0"),
    // Prerelease with ranges
    ("^1.0.0-alpha.1", "^1.0.0"),
    ("~1.0.0-beta.1", "~1.0.0"),
    ("1.0.0-rc.1", "^1.0.0"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than(&Specifier::new(newer)),
      "Expected {older} to be older than {newer}"
    );
  }
}

#[test]
fn older_major_variants() {
  let cases: Vec<(&str, &str)> = vec![
    // Major vs higher major
    ("1", "2"),
    ("1", "3"),
    ("5", "10"),
    // Major with ranges
    ("^1", "^2"),
    ("~1", "~2"),
    // Major vs exact (major variants have padded versions)
    // "1" -> "1.999999.999999", so it's newer than "1.0.0" but older than "2.0.0"
    ("1", "2.0.0"),
    ("1", "10.0.0"),
    ("^1", "2.0.0"),
    // Mixed
    ("1", "^2"),
    ("^1", "2"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than(&Specifier::new(newer)),
      "Expected {older} to be older than {newer}"
    );
  }
}

#[test]
fn not_older_major_variants() {
  let cases: Vec<(&str, &str)> = vec![
    // Same major
    ("1", "1"),
    ("^1", "^1"),
    ("~1", "~1"),
    // Major vs lower versions
    ("2", "1"),
    ("^2", "^1"),
    ("10", "9"),
    // Major padded version is newer than same-major exact versions
    ("1", "1.0.0"),
    ("1", "1.5.0"),
    ("1", "1.999998.999999"),
    ("^1", "1.0.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b}"
    );
  }
}

#[test]
fn older_minor_variants() {
  let cases: Vec<(&str, &str)> = vec![
    // Minor vs higher minor (same major)
    ("1.0", "1.1"),
    ("1.0", "1.5"),
    ("1.4", "1.5"),
    // Minor vs higher major
    ("1.0", "2.0"),
    ("1.5", "2.0"),
    // Minor with ranges
    ("^1.0", "^1.1"),
    ("~1.0", "~1.1"),
    ("^1.4", "^2.0"),
    // Minor vs exact (minor variants have padded patch)
    // "1.4" -> "1.4.999999", so it's newer than "1.4.0" but older than "1.5.0"
    ("1.4", "1.5.0"),
    ("1.4", "2.0.0"),
    ("^1.4", "1.5.0"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than(&Specifier::new(newer)),
      "Expected {older} to be older than {newer}"
    );
  }
}

#[test]
fn not_older_minor_variants() {
  let cases: Vec<(&str, &str)> = vec![
    // Same minor
    ("1.0", "1.0"),
    ("^1.4", "^1.4"),
    ("~1.4", "~1.4"),
    // Minor vs lower versions
    ("1.5", "1.4"),
    ("^1.5", "^1.4"),
    ("2.0", "1.5"),
    // Minor padded version is newer than same-minor exact versions
    ("1.4", "1.4.0"),
    ("1.4", "1.4.5"),
    ("1.4", "1.4.999998"),
    ("^1.4", "1.4.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b}"
    );
  }
}

#[test]
fn with_workspace_protocol() {
  let cases: Vec<(&str, &str)> = vec![
    // Workspace versions
    ("workspace:1.0.0", "workspace:1.0.1"),
    ("workspace:1.0.0", "workspace:2.0.0"),
    ("workspace:^1.0.0", "workspace:^1.0.1"),
    ("workspace:~1.0.0", "workspace:~2.0.0"),
    // Mixed workspace and regular
    ("workspace:1.0.0", "1.0.1"),
    ("1.0.0", "workspace:1.0.1"),
    ("workspace:^1.0.0", "^2.0.0"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than(&Specifier::new(newer)),
      "Expected {older} to be older than {newer}"
    );
  }
}

#[test]
fn with_npm_alias() {
  let cases: Vec<(&str, &str)> = vec![
    // Alias versions
    ("npm:foo@1.0.0", "npm:foo@1.0.1"),
    ("npm:foo@1.0.0", "npm:bar@1.0.1"),
    ("npm:foo@1.0.0", "npm:foo@2.0.0"),
    ("npm:@scope/pkg@1.0.0", "npm:@scope/pkg@1.0.1"),
    ("npm:foo@^1.0.0", "npm:foo@^1.0.1"),
    ("npm:foo@~1.0.0", "npm:foo@~2.0.0"),
    // Mixed alias and regular
    ("npm:foo@1.0.0", "1.0.1"),
    ("1.0.0", "npm:foo@1.0.1"),
    ("npm:foo@^1.0.0", "^2.0.0"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than(&Specifier::new(newer)),
      "Expected {older} to be older than {newer}"
    );
  }
}

#[test]
fn with_git_urls() {
  let cases: Vec<(&str, &str)> = vec![
    // Git with semver tags
    ("git@github.com:npm/cli.git#1.0.0", "git@github.com:npm/cli.git#1.0.1"),
    ("git@github.com:npm/cli.git#1.0.0", "git@github.com:npm/cli.git#2.0.0"),
    ("git@github.com:npm/cli.git#^1.0.0", "git@github.com:npm/cli.git#^1.0.1"),
    ("github:user/repo#1.0.0", "github:user/repo#1.0.1"),
    ("git+ssh://git@github.com/npm/cli#1.0.0", "git+ssh://git@github.com/npm/cli#2.0.0"),
    // Git vs regular
    ("git@github.com:npm/cli.git#1.0.0", "1.0.1"),
    ("1.0.0", "git@github.com:npm/cli.git#1.0.1"),
    ("github:user/repo#^1.0.0", "^2.0.0"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than(&Specifier::new(newer)),
      "Expected {older} to be older than {newer}"
    );
  }
}

#[test]
fn specifiers_without_node_version_always_false() {
  let without_version: Vec<&str> = vec![
    "latest",
    "next",
    "canary",
    "https://example.com/package.tgz",
    "file:../foo",
    ">=1.2.3 <2.0.0",
    "^1.2.3 || ^2.0.0",
    "workspace:*",
    "workspace:^",
    "npm:foo",
    "git@github.com:npm/cli.git",
    "git@github.com:npm/cli.git#main",
  ];

  let with_version: Vec<&str> = vec!["1.0.0", "^1.0.0", "~2.0.0"];

  // Without version vs without version
  for a in &without_version {
    for b in &without_version {
      assert!(
        !Specifier::new(a).is_older_than(&Specifier::new(b)),
        "Expected {a} to NOT be older than {b} (no node_version)"
      );
    }
  }

  // Without version vs with version (both directions)
  for without in &without_version {
    for with in &with_version {
      assert!(
        !Specifier::new(without).is_older_than(&Specifier::new(with)),
        "Expected {without} to NOT be older than {with} (no node_version)"
      );
      assert!(
        !Specifier::new(with).is_older_than(&Specifier::new(without)),
        "Expected {with} to NOT be older than {without} (no node_version)"
      );
    }
  }
}

#[test]
fn comprehensive_version_progression() {
  let versions: Vec<&str> = vec![
    "0.0.0",
    "0.0.1",
    "0.1.0",
    "1.0.0-alpha.1",
    "1.0.0-beta.1",
    "1.0.0-rc.1",
    "1.0.0",
    "1.0.1",
    "1.1.0",
    "1.4.0",
    "1.4.1",
    "2.0.0",
    "10.0.0",
  ];

  // Each version should be older than all versions that come after it
  for i in 0..versions.len() {
    for j in (i + 1)..versions.len() {
      let older = versions[i];
      let newer = versions[j];
      assert!(
        Specifier::new(older).is_older_than(&Specifier::new(newer)),
        "Expected {older} to be older than {newer}"
      );
      assert!(
        !Specifier::new(newer).is_older_than(&Specifier::new(older)),
        "Expected {newer} to NOT be older than {older}"
      );
    }
  }
}

#[test]
fn self_comparison_always_false() {
  let cases: Vec<&str> = vec![
    "1.0.0",
    "^1.0.0",
    "~1.0.0",
    ">=1.0.0",
    "1.0.0-alpha.1",
    "1",
    "1.4",
    "npm:foo@1.0.0",
    "workspace:1.0.0",
    "git@github.com:npm/cli.git#1.0.0",
    "latest",
    "https://example.com/package.tgz",
  ];
  for value in cases {
    let spec = Specifier::new(value);
    assert!(!spec.is_older_than(&spec), "Expected {value} to NOT be older than itself");
  }
}

#[test]
fn edge_cases() {
  // Zero versions
  assert!(Specifier::new("0.0.0").is_older_than(&Specifier::new("0.0.1")));
  assert!(Specifier::new("0.0.0").is_older_than(&Specifier::new("1.0.0")));
  assert!(!Specifier::new("0.0.1").is_older_than(&Specifier::new("0.0.0")));

  // Large version numbers
  assert!(Specifier::new("999.999.998").is_older_than(&Specifier::new("999.999.999")));
  assert!(Specifier::new("1.0.0").is_older_than(&Specifier::new("999.999.999")));

  // Complex prerelease identifiers
  assert!(Specifier::new("1.0.0-alpha.1.2.3").is_older_than(&Specifier::new("1.0.0-alpha.1.2.4")));
  assert!(Specifier::new("1.0.0-alpha.1").is_older_than(&Specifier::new("1.0.0")));
}
