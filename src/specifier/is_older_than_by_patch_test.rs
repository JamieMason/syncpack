use crate::specifier::Specifier;

#[test]
fn older_by_patch_same_major_and_minor() {
  let cases: Vec<(&str, &str)> = vec![
    // Same major and minor, different patch
    ("1.0.0", "1.0.1"),
    ("1.0.0", "1.0.2"),
    ("1.0.0", "1.0.5"),
    ("1.4.5", "1.4.6"),
    ("1.4.5", "1.4.10"),
    ("1.4.5", "1.4.100"),
    ("2.3.0", "2.3.1"),
    ("2.3.10", "2.3.11"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than_by_patch(&Specifier::new(newer)),
      "Expected {older} to be older than {newer} by patch (same major.minor)"
    );
  }
}

#[test]
fn not_older_by_patch_different_minor() {
  let cases: Vec<(&str, &str)> = vec![
    // Same major, different minor (even if patch is lower)
    ("1.0.0", "1.1.0"),
    ("1.0.9", "1.1.0"),
    ("1.4.0", "1.5.0"),
    ("1.4.5", "1.5.0"),
    ("1.4.10", "1.5.0"),
    ("2.3.5", "2.4.0"),
    ("2.3.99", "2.4.1"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by patch (different minor)"
    );
  }
}

#[test]
fn not_older_by_patch_different_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Different major (even if minor and patch are lower)
    ("1.0.0", "2.0.0"),
    ("1.5.0", "2.0.0"),
    ("1.5.10", "2.0.0"),
    ("1.9.9", "2.0.0"),
    ("1.0.0", "3.0.0"),
    ("2.4.1", "3.0.0"),
    ("5.3.2", "10.0.0"),
    // Higher major, even with same or lower minor/patch
    ("2.0.0", "1.0.0"),
    ("3.5.10", "2.5.10"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by patch (different major)"
    );
  }
}

#[test]
fn not_older_by_patch_same_or_newer() {
  let cases: Vec<(&str, &str)> = vec![
    // Same version
    ("1.0.0", "1.0.0"),
    ("1.4.1", "1.4.1"),
    ("1.5.10", "1.5.10"),
    ("2.3.5", "2.3.5"),
    // Same major.minor, but reversed (newer to older)
    ("1.0.1", "1.0.0"),
    ("1.4.10", "1.4.5"),
    ("1.0.100", "1.0.99"),
    ("2.3.5", "2.3.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by patch"
    );
  }
}

#[test]
fn older_by_patch_with_ranges() {
  let cases: Vec<(&str, &str)> = vec![
    // Caret ranges, same major.minor
    ("^1.0.0", "^1.0.1"),
    ("^1.4.5", "^1.4.6"),
    ("^2.3.0", "^2.3.1"),
    // Tilde ranges, same major.minor
    ("~1.0.0", "~1.0.1"),
    ("~1.4.5", "~1.4.6"),
    ("~2.3.0", "~2.3.10"),
    // Mixed exact and ranges, same major.minor
    ("1.0.0", "^1.0.1"),
    ("1.0.0", "~1.0.1"),
    ("^1.4.5", "1.4.6"),
    ("~1.4.5", "1.4.6"),
    // Comparison ranges, same major.minor
    (">=1.0.0", ">=1.0.1"),
    (">1.0.0", ">1.0.1"),
    ("<=1.0.5", "<=1.0.10"),
    ("<1.4.10", "<1.4.20"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than_by_patch(&Specifier::new(newer)),
      "Expected {older} to be older than {newer} by patch"
    );
  }
}

#[test]
fn not_older_by_patch_with_ranges_different_minor() {
  let cases: Vec<(&str, &str)> = vec![
    // Same major, different minor
    ("^1.0.0", "^1.1.0"),
    ("~1.4.5", "~1.5.0"),
    (">=1.0.0", ">=1.1.0"),
    (">1.4.9", ">1.5.0"),
    ("1.0.0", "^1.1.0"),
    ("^1.4.5", "1.5.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by patch (different minor)"
    );
  }
}

#[test]
fn not_older_by_patch_with_ranges_different_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Different major
    ("^1.0.0", "^2.0.0"),
    ("~1.5.5", "~2.0.0"),
    (">=1.0.0", ">=2.0.0"),
    (">1.9.9", ">2.0.0"),
    ("1.0.0", "^2.0.0"),
    ("^1.5.5", "2.0.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by patch (different major)"
    );
  }
}

#[test]
fn older_by_patch_with_prerelease() {
  let cases: Vec<(&str, &str)> = vec![
    // Prerelease vs release, same major.minor
    ("1.0.0-alpha.1", "1.0.0"),
    ("1.0.0-beta.1", "1.0.0"),
    ("1.4.5-rc.1", "1.4.5"),
    ("2.3.0-alpha.1", "2.3.0"),
    // Different prereleases, same major.minor
    ("1.0.0-alpha.1", "1.0.0-alpha.2"),
    ("1.0.0-alpha.1", "1.0.0-beta.1"),
    ("1.4.5-beta.1", "1.4.5-rc.1"),
    // Prerelease with patch difference, same major.minor
    ("1.0.0-alpha.1", "1.0.1"),
    ("1.4.5-alpha.1", "1.4.6"),
    ("2.3.0-rc.1", "2.3.1"),
    // Prerelease with ranges, same major.minor
    ("^1.0.0-alpha.1", "^1.0.0"),
    ("~1.4.5-beta.1", "~1.4.5"),
    ("1.0.0-rc.1", "^1.0.0"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than_by_patch(&Specifier::new(newer)),
      "Expected {older} to be older than {newer} by patch"
    );
  }
}

#[test]
fn not_older_by_patch_with_prerelease_different_minor() {
  let cases: Vec<(&str, &str)> = vec![
    // Prerelease, different minor
    ("1.0.0-alpha.1", "1.1.0"),
    ("1.4.5-rc.1", "1.5.0"),
    ("2.3.0-beta.1", "2.4.0"),
    ("^1.0.0-alpha.1", "^1.1.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by patch (different minor)"
    );
  }
}

#[test]
fn not_older_by_patch_with_prerelease_different_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Prerelease, different major
    ("1.0.0-alpha.1", "2.0.0"),
    ("1.9.9-rc.1", "2.0.0-alpha.1"),
    ("1.5.5-alpha.1", "2.0.0-beta.1"),
    ("^1.0.0-alpha.1", "^2.0.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by patch (different major)"
    );
  }
}

#[test]
fn major_variants_cannot_be_older_by_patch() {
  let cases: Vec<(&str, &str)> = vec![
    // Major variants have padded versions (e.g., "1" -> "1.999999.999999")
    // Different majors fail the major check
    ("1", "2"),
    ("^1", "^2"),
    ("~1", "~2"),
    // Same major, but padded minor differs from exact versions
    ("1", "1.0.0"),
    ("1", "1.0.1"),
    ("^1", "1.0.0"),
    ("~1", "1.0.1"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by patch (major variants)"
    );
  }
}

#[test]
fn minor_variants_cannot_be_older_by_patch() {
  let cases: Vec<(&str, &str)> = vec![
    // Minor variants have padded patch (e.g., "1.4" -> "1.4.999999")
    // Different minor fails the minor check
    ("1.0", "1.1"),
    ("1.4", "1.5"),
    ("^1.0", "^1.1"),
    ("~1.4", "~1.5"),
    // Different major fails the major check
    ("1.0", "2.0"),
    ("1.5", "2.0"),
    // Same major.minor, but minor variant is newer than exact versions
    ("1.4", "1.4.0"),
    ("1.4", "1.4.5"),
    ("1.4", "1.4.999998"),
    ("^1.4", "1.4.0"),
    ("~1.0", "1.0.5"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by patch (minor variants)"
    );
  }
}

#[test]
fn with_workspace_protocol_same_major_minor() {
  let cases: Vec<(&str, &str)> = vec![
    // Workspace versions, same major.minor
    ("workspace:1.0.0", "workspace:1.0.1"),
    ("workspace:1.0.0", "workspace:1.0.5"),
    ("workspace:1.4.5", "workspace:1.4.6"),
    ("workspace:^1.0.0", "workspace:^1.0.1"),
    ("workspace:~1.4.5", "workspace:~1.4.6"),
    // Mixed workspace and regular, same major.minor
    ("workspace:1.0.0", "1.0.1"),
    ("1.0.0", "workspace:1.0.1"),
    ("workspace:^1.4.5", "^1.4.6"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than_by_patch(&Specifier::new(newer)),
      "Expected {older} to be older than {newer} by patch"
    );
  }
}

#[test]
fn with_workspace_protocol_different_minor() {
  let cases: Vec<(&str, &str)> = vec![
    // Different minor
    ("workspace:1.0.0", "workspace:1.1.0"),
    ("workspace:1.4.5", "workspace:1.5.0"),
    ("workspace:^1.0.0", "workspace:^1.1.0"),
    ("workspace:1.0.0", "1.1.0"),
    ("1.0.0", "workspace:1.1.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by patch (different minor)"
    );
  }
}

#[test]
fn with_workspace_protocol_different_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Different major
    ("workspace:1.0.0", "workspace:2.0.0"),
    ("workspace:1.9.9", "workspace:2.0.0"),
    ("workspace:^1.5.5", "workspace:^2.0.0"),
    ("workspace:1.0.0", "2.0.0"),
    ("1.0.0", "workspace:2.0.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by patch (different major)"
    );
  }
}

#[test]
fn with_npm_alias_same_major_minor() {
  let cases: Vec<(&str, &str)> = vec![
    // Alias versions, same major.minor
    ("npm:foo@1.0.0", "npm:foo@1.0.1"),
    ("npm:foo@1.0.0", "npm:bar@1.0.1"),
    ("npm:foo@1.4.5", "npm:foo@1.4.6"),
    ("npm:@scope/pkg@1.0.0", "npm:@scope/pkg@1.0.1"),
    ("npm:foo@^1.0.0", "npm:foo@^1.0.1"),
    ("npm:foo@~1.4.5", "npm:foo@~1.4.6"),
    // Mixed alias and regular, same major.minor
    ("npm:foo@1.0.0", "1.0.1"),
    ("1.0.0", "npm:foo@1.0.1"),
    ("npm:foo@^1.4.5", "^1.4.6"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than_by_patch(&Specifier::new(newer)),
      "Expected {older} to be older than {newer} by patch"
    );
  }
}

#[test]
fn with_npm_alias_different_minor() {
  let cases: Vec<(&str, &str)> = vec![
    // Different minor
    ("npm:foo@1.0.0", "npm:foo@1.1.0"),
    ("npm:foo@1.4.5", "npm:bar@1.5.0"),
    ("npm:@scope/pkg@1.0.0", "npm:@scope/pkg@1.1.0"),
    ("npm:foo@^1.0.0", "npm:foo@^1.1.0"),
    ("npm:foo@1.0.0", "1.1.0"),
    ("1.0.0", "npm:foo@1.1.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by patch (different minor)"
    );
  }
}

#[test]
fn with_npm_alias_different_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Different major
    ("npm:foo@1.0.0", "npm:foo@2.0.0"),
    ("npm:foo@1.9.9", "npm:bar@2.0.0"),
    ("npm:@scope/pkg@1.5.5", "npm:@scope/pkg@2.0.0"),
    ("npm:foo@^1.0.0", "npm:foo@^2.0.0"),
    ("npm:foo@1.0.0", "2.0.0"),
    ("1.0.0", "npm:foo@2.0.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by patch (different major)"
    );
  }
}

#[test]
fn with_git_urls_same_major_minor() {
  let cases: Vec<(&str, &str)> = vec![
    // Git with semver tags, same major.minor
    ("git@github.com:npm/cli.git#1.0.0", "git@github.com:npm/cli.git#1.0.1"),
    ("git@github.com:npm/cli.git#1.4.5", "git@github.com:npm/cli.git#1.4.6"),
    ("git@github.com:npm/cli.git#^1.0.0", "git@github.com:npm/cli.git#^1.0.1"),
    ("github:user/repo#1.0.0", "github:user/repo#1.0.1"),
    ("git+ssh://git@github.com/npm/cli#~1.4.5", "git+ssh://git@github.com/npm/cli#~1.4.6"),
    // Git vs regular, same major.minor
    ("git@github.com:npm/cli.git#1.0.0", "1.0.1"),
    ("1.0.0", "git@github.com:npm/cli.git#1.0.1"),
    ("github:user/repo#^1.4.5", "^1.4.6"),
  ];
  for (older, newer) in cases {
    assert!(
      Specifier::new(older).is_older_than_by_patch(&Specifier::new(newer)),
      "Expected {older} to be older than {newer} by patch"
    );
  }
}

#[test]
fn with_git_urls_different_minor() {
  let cases: Vec<(&str, &str)> = vec![
    // Different minor
    ("git@github.com:npm/cli.git#1.0.0", "git@github.com:npm/cli.git#1.1.0"),
    ("git@github.com:npm/cli.git#^1.4.5", "git@github.com:npm/cli.git#^1.5.0"),
    ("github:user/repo#1.0.0", "github:user/repo#1.1.0"),
    ("git@github.com:npm/cli.git#1.0.0", "1.1.0"),
    ("1.0.0", "git@github.com:npm/cli.git#1.1.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by patch (different minor)"
    );
  }
}

#[test]
fn with_git_urls_different_major() {
  let cases: Vec<(&str, &str)> = vec![
    // Different major
    ("git@github.com:npm/cli.git#1.0.0", "git@github.com:npm/cli.git#2.0.0"),
    ("git@github.com:npm/cli.git#^1.5.5", "git@github.com:npm/cli.git#^2.0.0"),
    ("github:user/repo#1.9.9", "github:user/repo#2.0.0"),
    ("git@github.com:npm/cli.git#1.0.0", "2.0.0"),
    ("1.0.0", "git@github.com:npm/cli.git#2.0.0"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
      "Expected {a} to NOT be older than {b} by patch (different major)"
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

  let with_version_same_major_minor: Vec<&str> = vec!["1.0.0", "^1.0.1", "~1.0.5"];

  // Without version vs without version
  for a in &without_version {
    for b in &without_version {
      assert!(
        !Specifier::new(a).is_older_than_by_patch(&Specifier::new(b)),
        "Expected {a} to NOT be older than {b} by patch (no node_version)"
      );
    }
  }

  // Without version vs with version (both directions)
  for without in &without_version {
    for with in &with_version_same_major_minor {
      assert!(
        !Specifier::new(without).is_older_than_by_patch(&Specifier::new(with)),
        "Expected {without} to NOT be older than {with} by patch (no node_version)"
      );
      assert!(
        !Specifier::new(with).is_older_than_by_patch(&Specifier::new(without)),
        "Expected {with} to NOT be older than {without} by patch (no node_version)"
      );
    }
  }
}

#[test]
fn comprehensive_same_major_minor_progression() {
  let versions: Vec<&str> = vec![
    "1.0.0-alpha.1",
    "1.0.0-beta.1",
    "1.0.0",
    "1.0.1",
    "1.0.2",
    "1.0.5",
    "1.0.10",
    "1.0.100",
  ];

  // Each version should be older than all versions that come after it (same
  // major.minor)
  for i in 0..versions.len() {
    for j in (i + 1)..versions.len() {
      let older = versions[i];
      let newer = versions[j];
      assert!(
        Specifier::new(older).is_older_than_by_patch(&Specifier::new(newer)),
        "Expected {older} to be older than {newer} by patch"
      );
      assert!(
        !Specifier::new(newer).is_older_than_by_patch(&Specifier::new(older)),
        "Expected {newer} to NOT be older than {older} by patch"
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
      !spec.is_older_than_by_patch(&spec),
      "Expected {value} to NOT be older than itself by patch"
    );
  }
}

#[test]
fn edge_cases() {
  // Zero versions, same major.minor
  assert!(Specifier::new("0.0.0").is_older_than_by_patch(&Specifier::new("0.0.1")));
  assert!(!Specifier::new("0.0.1").is_older_than_by_patch(&Specifier::new("0.0.0")));

  // Zero to different minor (same major)
  assert!(!Specifier::new("0.0.9").is_older_than_by_patch(&Specifier::new("0.1.0")));

  // Zero to one (different major)
  assert!(!Specifier::new("0.0.9").is_older_than_by_patch(&Specifier::new("1.0.0")));

  // Large version numbers, same major.minor
  assert!(Specifier::new("1.0.0").is_older_than_by_patch(&Specifier::new("1.0.999")));
  assert!(Specifier::new("1.0.998").is_older_than_by_patch(&Specifier::new("1.0.999")));

  // Large version numbers, different minor
  assert!(!Specifier::new("1.0.999").is_older_than_by_patch(&Specifier::new("1.1.0")));

  // Large version numbers, different major
  assert!(!Specifier::new("1.0.999").is_older_than_by_patch(&Specifier::new("2.0.0")));

  // Complex prerelease identifiers, same major.minor
  assert!(Specifier::new("1.0.0-alpha.1.2.3").is_older_than_by_patch(&Specifier::new("1.0.0-alpha.1.2.4")));
  assert!(Specifier::new("1.0.0-alpha.1").is_older_than_by_patch(&Specifier::new("1.0.0")));
  assert!(Specifier::new("1.0.0-alpha.1").is_older_than_by_patch(&Specifier::new("1.0.1")));

  // Complex prerelease identifiers, different minor
  assert!(!Specifier::new("1.0.0-alpha.1").is_older_than_by_patch(&Specifier::new("1.1.0")));
}
