use crate::specifier::Specifier;

#[test]
fn older_by_minor_same_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Same major, different minor
    ("1.0.0", "1.1.0"),
    ("1.0.0", "1.2.0"),
    ("1.0.0", "1.5.0"),
    ("1.4.0", "1.5.0"),
    ("1.4.0", "1.10.0"),
    // Same major, different minor and patch
    ("1.0.0", "1.1.1"),
    ("1.0.0", "1.2.5"),
    ("1.4.5", "1.5.0"),
    ("1.4.5", "1.5.10"),
    // Same major, only patch difference also counts
    ("1.0.0", "1.0.1"),
    ("1.0.0", "1.0.5"),
    ("1.4.5", "1.4.6"),
    ("1.4.5", "1.4.100"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than_by_minor(&Specifier::new(newer)),
      "Expected {older} to be older than {newer} by minor (same major)"
    );
  }
}

#[test]
fn not_older_by_minor_different_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Different major versions (even if version is older)
    ("1.0.0", "2.0.0"),
    ("1.5.0", "2.0.0"),
    ("1.9.9", "2.0.0"),
    ("1.0.0", "3.0.0"),
    ("2.4.1", "3.0.0"),
    ("5.0.0", "10.0.0"),
    // Higher major, even with lower minor/patch
    ("2.9.9", "1.0.0"),
    ("3.0.0", "2.5.10"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_minor(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by minor (different major)"
    );
  }
}

#[test]
fn not_older_by_minor_same_or_newer() {
  let cases: Vec<(&str, &str)> = vec![
    // Same version
    ("1.0.0", "1.0.0"),
    ("1.4.1", "1.4.1"),
    ("1.5.0", "1.5.0"),
    // Same major, but reversed (newer to older)
    ("1.1.0", "1.0.0"),
    ("1.5.0", "1.4.0"),
    ("1.0.1", "1.0.0"),
    ("1.5.10", "1.5.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_minor(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by minor"
    );
  }
}

#[test]
fn older_by_minor_with_ranges() {
  let cases: Vec<(&str, &str)> = vec![
    // Caret ranges, same major
    ("^1.0.0", "^1.1.0"),
    ("^1.4.0", "^1.5.0"),
    ("^1.0.0", "^1.0.1"),
    // Tilde ranges, same major
    ("~1.0.0", "~1.1.0"),
    ("~1.4.0", "~1.5.0"),
    ("~1.0.0", "~1.0.1"),
    // Mixed exact and ranges, same major
    ("1.0.0", "^1.1.0"),
    ("1.0.0", "~1.0.1"),
    ("^1.4.0", "1.5.0"),
    ("~1.4.0", "1.4.1"),
    // Comparison ranges, same major
    (">=1.0.0", ">=1.1.0"),
    (">1.0.0", ">1.0.1"),
    ("<=1.0.0", "<=1.1.0"),
    ("<1.5.0", "<1.6.0"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than_by_minor(&Specifier::new(newer)),
      "Expected {older} to be older than {newer} by minor"
    );
  }
}

#[test]
fn not_older_by_minor_with_ranges_different_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Different major
    ("^1.0.0", "^2.0.0"),
    ("~1.5.0", "~2.0.0"),
    (">=1.0.0", ">=2.0.0"),
    (">1.9.9", ">2.0.0"),
    ("1.0.0", "^2.0.0"),
    ("^1.5.0", "2.0.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_minor(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by minor (different major)"
    );
  }
}

#[test]
fn older_by_minor_with_prerelease() {
  let cases: Vec<(&str, &str)> = vec![
    // Prerelease vs release, same major
    ("1.0.0-alpha.1", "1.0.0"),
    ("1.0.0-beta.1", "1.0.0"),
    ("1.4.0-rc.1", "1.4.0"),
    // Different prereleases, same major
    ("1.0.0-alpha.1", "1.0.0-alpha.2"),
    ("1.0.0-alpha.1", "1.0.0-beta.1"),
    ("1.4.0-beta.1", "1.4.0-rc.1"),
    // Prerelease with minor difference, same major
    ("1.0.0-alpha.1", "1.1.0"),
    ("1.4.0-alpha.1", "1.5.0"),
    ("1.0.0-alpha.1", "1.0.1"),
    // Prerelease with ranges, same major
    ("^1.0.0-alpha.1", "^1.0.0"),
    ("~1.4.0-beta.1", "~1.4.0"),
    ("1.0.0-rc.1", "^1.0.0"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than_by_minor(&Specifier::new(newer)),
      "Expected {older} to be older than {newer} by minor"
    );
  }
}

#[test]
fn not_older_by_minor_with_prerelease_different_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Prerelease, different major
    ("1.0.0-alpha.1", "2.0.0"),
    ("1.9.9-rc.1", "2.0.0-alpha.1"),
    ("1.0.0-alpha.1", "2.0.0-beta.1"),
    ("^1.0.0-alpha.1", "^2.0.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_minor(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by minor (different major)"
    );
  }
}

#[test]
fn major_variants_cannot_be_older_by_minor() {
  let cases: Vec<(&str, &str)> = vec![
    // Major variants have padded versions (e.g., "1" -> "1.999999.999999")
    // So different majors fail the major check
    ("1", "2"),
    ("^1", "^2"),
    ("~1", "~2"),
    ("1", "^2"),
    // Major vs exact/minor - different effective versions
    // "1" (1.999999.999999) vs "1.0.0" - they're in same major, but major variant is newer
    // So this tests the "other > self" part failing
    ("1", "1.0.0"),
    ("^1", "1.0.0"),
    ("1", "^1.0.0"),
    // But within same major with proper ordering should work
    // Actually, testing all combinations for major variants
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_minor(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by minor (major variants)"
    );
  }
}

#[test]
fn minor_variants_same_major() {
  // Minor variants have padded patch (e.g., "1.4" -> "1.4.999999")
  // These should be older by minor (same major, proper ordering)
  let older_cases: Vec<(&str, &str)> = vec![("1.0", "1.1"), ("1.4", "1.5"), ("^1.0", "^1.1"), ("1.4", "1.5.0")];

  for (older, newer) in older_cases {
    assert!(
      Specifier::new(older).is_older_than_by_minor(&Specifier::new(newer)),
      "Expected {older} to be older than {newer} by minor"
    );
  }

  // These should NOT be older by minor (minor variant is newer than same-minor
  // exact)
  let not_older_cases: Vec<(&str, &str)> = vec![("1.4", "1.4.0"), ("1.4", "1.4.5"), ("^1.4", "1.4.0"), ("~1.4", "1.4.999998")];

  for (a, b) in not_older_cases {
    assert!(
      !Specifier::new(a).is_older_than_by_minor(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by minor (padded patch is newer)"
    );
  }
}

#[test]
fn minor_variants_different_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Different major
    ("1.0", "2.0"),
    ("1.5", "2.0"),
    ("^1.4", "^2.0"),
    ("~1.9", "~2.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_minor(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by minor (different major)"
    );
  }
}

#[test]
fn with_workspace_protocol_same_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Workspace versions, same major
    ("workspace:1.0.0", "workspace:1.1.0"),
    ("workspace:1.0.0", "workspace:1.0.1"),
    ("workspace:1.4.0", "workspace:1.5.0"),
    ("workspace:^1.0.0", "workspace:^1.1.0"),
    ("workspace:~1.0.0", "workspace:~1.0.1"),
    // Mixed workspace and regular, same major
    ("workspace:1.0.0", "1.1.0"),
    ("1.0.0", "workspace:1.1.0"),
    ("workspace:^1.4.0", "^1.5.0"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than_by_minor(&Specifier::new(newer)),
      "Expected {older} to be older than {newer} by minor"
    );
  }
}

#[test]
fn with_workspace_protocol_different_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Different major
    ("workspace:1.0.0", "workspace:2.0.0"),
    ("workspace:1.9.9", "workspace:2.0.0"),
    ("workspace:^1.5.0", "workspace:^2.0.0"),
    ("workspace:1.0.0", "2.0.0"),
    ("1.0.0", "workspace:2.0.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_minor(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by minor (different major)"
    );
  }
}

#[test]
fn with_npm_alias_same_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Alias versions, same major
    ("npm:foo@1.0.0", "npm:foo@1.1.0"),
    ("npm:foo@1.0.0", "npm:bar@1.1.0"),
    ("npm:foo@1.0.0", "npm:foo@1.0.1"),
    ("npm:@scope/pkg@1.4.0", "npm:@scope/pkg@1.5.0"),
    ("npm:foo@^1.0.0", "npm:foo@^1.1.0"),
    ("npm:foo@~1.4.0", "npm:foo@~1.4.1"),
    // Mixed alias and regular, same major
    ("npm:foo@1.0.0", "1.1.0"),
    ("1.0.0", "npm:foo@1.1.0"),
    ("npm:foo@^1.4.0", "^1.5.0"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than_by_minor(&Specifier::new(newer)),
      "Expected {older} to be older than {newer} by minor"
    );
  }
}

#[test]
fn with_npm_alias_different_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Different major
    ("npm:foo@1.0.0", "npm:foo@2.0.0"),
    ("npm:foo@1.9.9", "npm:bar@2.0.0"),
    ("npm:@scope/pkg@1.5.0", "npm:@scope/pkg@2.0.0"),
    ("npm:foo@^1.0.0", "npm:foo@^2.0.0"),
    ("npm:foo@1.0.0", "2.0.0"),
    ("1.0.0", "npm:foo@2.0.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_minor(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by minor (different major)"
    );
  }
}

#[test]
fn with_git_urls_same_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Git with semver tags, same major
    ("git@github.com:npm/cli.git#1.0.0", "git@github.com:npm/cli.git#1.1.0"),
    ("git@github.com:npm/cli.git#1.0.0", "git@github.com:npm/cli.git#1.0.1"),
    ("git@github.com:npm/cli.git#^1.4.0", "git@github.com:npm/cli.git#^1.5.0"),
    ("github:user/repo#1.0.0", "github:user/repo#1.1.0"),
    ("git+ssh://git@github.com/npm/cli#~1.0.0", "git+ssh://git@github.com/npm/cli#~1.0.1"),
    // Git vs regular, same major
    ("git@github.com:npm/cli.git#1.0.0", "1.1.0"),
    ("1.0.0", "git@github.com:npm/cli.git#1.1.0"),
    ("github:user/repo#^1.4.0", "^1.5.0"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than_by_minor(&Specifier::new(newer)),
      "Expected {older} to be older than {newer} by minor"
    );
  }
}

#[test]
fn with_git_urls_different_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Different major
    ("git@github.com:npm/cli.git#1.0.0", "git@github.com:npm/cli.git#2.0.0"),
    ("git@github.com:npm/cli.git#^1.5.0", "git@github.com:npm/cli.git#^2.0.0"),
    ("github:user/repo#1.9.9", "github:user/repo#2.0.0"),
    ("git@github.com:npm/cli.git#1.0.0", "2.0.0"),
    ("1.0.0", "git@github.com:npm/cli.git#2.0.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_minor(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by minor (different major)"
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

  let with_version_same_major: Vec<&str> = vec!["1.0.0", "^1.1.0", "~1.5.0"];

  // Without version vs without version
  for a in &without_version {
    for b in &without_version {
      assert!(
        !Specifier::new(a).is_older_than_by_minor(&Specifier::new(b)),
        "Expected {a} to NOT be older than {b} by minor (no node_version)"
      );
    }
  }

  // Without version vs with version (both directions)
  for without in &without_version {
    for with in &with_version_same_major {
      assert!(
        !Specifier::new(without).is_older_than_by_minor(&Specifier::new(with)),
        "Expected {without} to NOT be older than {with} by minor (no node_version)"
      );
      assert!(
        !Specifier::new(with).is_older_than_by_minor(&Specifier::new(without)),
        "Expected {with} to NOT be older than {without} by minor (no node_version)"
      );
    }
  }
}

#[test]
fn comprehensive_same_major_progression() {
  let versions: Vec<&str> = vec![
    "1.0.0-alpha.1",
    "1.0.0-beta.1",
    "1.0.0",
    "1.0.1",
    "1.1.0",
    "1.4.0",
    "1.4.1",
    "1.5.0",
    "1.10.0",
  ];

  // Each version should be older than all versions that come after it (same
  // major)
  for i in 0..versions.len() {
    for j in (i + 1)..versions.len() {
      let older = versions[i];
      let newer = versions[j];
      assert!(
        Specifier::new(older).is_older_than_by_minor(&Specifier::new(newer)),
        "Expected {older} to be older than {newer} by minor"
      );
      assert!(
        !Specifier::new(newer).is_older_than_by_minor(&Specifier::new(older)),
        "Expected {newer} to NOT be older than {older} by minor"
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
    assert!(
      !spec.is_older_than_by_minor(&spec),
      "Expected {value} to NOT be older than itself by minor"
    );
  }
}

#[test]
fn edge_cases() {
  // Zero versions, same major
  assert!(Specifier::new("0.0.0").is_older_than_by_minor(&Specifier::new("0.0.1")));
  assert!(Specifier::new("0.0.0").is_older_than_by_minor(&Specifier::new("0.1.0")));
  assert!(!Specifier::new("0.0.1").is_older_than_by_minor(&Specifier::new("0.0.0")));

  // Zero to one (different major)
  assert!(!Specifier::new("0.9.9").is_older_than_by_minor(&Specifier::new("1.0.0")));

  // Large version numbers, same major
  assert!(Specifier::new("1.0.0").is_older_than_by_minor(&Specifier::new("1.999.999")));
  assert!(Specifier::new("1.999.998").is_older_than_by_minor(&Specifier::new("1.999.999")));

  // Large version numbers, different major
  assert!(!Specifier::new("1.999.999").is_older_than_by_minor(&Specifier::new("2.0.0")));

  // Complex prerelease identifiers, same major
  assert!(Specifier::new("1.0.0-alpha.1.2.3").is_older_than_by_minor(&Specifier::new("1.0.0-alpha.1.2.4")));
  assert!(Specifier::new("1.0.0-alpha.1").is_older_than_by_minor(&Specifier::new("1.0.0")));
  assert!(Specifier::new("1.0.0-alpha.1").is_older_than_by_minor(&Specifier::new("1.1.0")));
}
