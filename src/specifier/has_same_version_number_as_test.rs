use crate::specifier::Specifier;

#[test]
fn same_version_different_ranges() {
  let cases: Vec<(&str, &str)> = vec![
    // Same exact version, different range operators
    ("1.4.1", "^1.4.1"),
    ("1.4.1", "~1.4.1"),
    ("1.4.1", ">=1.4.1"),
    ("1.4.1", ">1.4.1"),
    ("1.4.1", "<=1.4.1"),
    ("1.4.1", "<1.4.1"),
    // Different range operators, same version
    ("^1.4.1", "~1.4.1"),
    ("^1.4.1", ">=1.4.1"),
    ("~1.4.1", ">=1.4.1"),
    (">1.4.1", "<=1.4.1"),
    // With prerelease versions
    ("1.4.1-alpha.1", "^1.4.1-alpha.1"),
    ("1.4.1-beta.2", "~1.4.1-beta.2"),
    ("^1.4.1-rc.3", ">=1.4.1-rc.3"),
  ];
  for (a, b) in cases {
    assert!(
      Specifier::new(a).has_same_version_number_as(&Specifier::new(b)),
      "Expected {a} and {b} to have same version number"
    );
    // Test symmetry
    assert!(
      Specifier::new(b).has_same_version_number_as(&Specifier::new(a)),
      "Expected {b} and {a} to have same version number (symmetry check)"
    );
  }
}

#[test]
fn different_versions() {
  let cases: Vec<(&str, &str)> = vec![
    // Different patch versions
    ("1.4.1", "1.4.2"),
    ("^1.4.1", "^1.4.2"),
    ("~1.4.1", "~1.4.2"),
    // Different minor versions
    ("1.4.1", "1.5.1"),
    ("^1.4.1", "^1.5.1"),
    // Different major versions
    ("1.4.1", "2.4.1"),
    ("^1.4.1", "^2.4.1"),
    // Different prerelease versions (same base, different prerelease)
    ("1.4.1-alpha.1", "1.4.1-alpha.2"),
    ("1.4.1-alpha.1", "1.4.1-beta.1"),
    ("^1.4.1-alpha.1", "^1.4.1-alpha.2"),
    // Different base versions with prerelease
    ("1.4.1-alpha.1", "1.4.2-alpha.1"),
    ("1.4.1", "1.4.1-alpha.1"),
    // Completely different
    ("1.0.0", "2.0.0"),
    ("^1.2.3", "~4.5.6"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).has_same_version_number_as(&Specifier::new(b)),
      "Expected {a} and {b} to have different version numbers"
    );
    // Test symmetry
    assert!(
      !Specifier::new(b).has_same_version_number_as(&Specifier::new(a)),
      "Expected {b} and {a} to have different version numbers (symmetry check)"
    );
  }
}

#[test]
fn with_workspace_protocol() {
  // workspace: protocol WITH embedded versions DO have node_version and should
  // match
  let cases_same: Vec<(&str, &str)> = vec![
    // workspace vs workspace (same version)
    ("workspace:1.4.1", "workspace:1.4.1"),
    ("workspace:^1.4.1", "workspace:~1.4.1"),
    ("workspace:1.4.1", "workspace:^1.4.1"),
    // workspace vs regular (same version)
    ("workspace:1.4.1", "1.4.1"),
    ("workspace:^1.4.1", "^1.4.1"),
    ("workspace:~1.4.1", "~1.4.1"),
  ];
  for (a, b) in cases_same {
    assert!(
      Specifier::new(a).has_same_version_number_as(&Specifier::new(b)),
      "Expected {a} and {b} to have same version number"
    );
  }

  // workspace with range-only (no embedded version) have no node_version
  let cases_false: Vec<(&str, &str)> = vec![
    ("workspace:*", "workspace:^"),
    ("workspace:*", "1.4.1"),
    ("workspace:^", "^1.4.1"),
    ("workspace:~", "~1.4.1"),
  ];
  for (a, b) in cases_false {
    assert!(
      !Specifier::new(a).has_same_version_number_as(&Specifier::new(b)),
      "Expected {a} and {b} to return false (range-only workspace protocol has no node_version)"
    );
  }
}

#[test]
fn with_npm_alias() {
  let cases_same: Vec<(&str, &str)> = vec![
    // Same version, different aliases
    ("npm:foo@1.4.1", "npm:bar@1.4.1"),
    ("npm:foo@^1.4.1", "npm:bar@~1.4.1"),
    ("npm:@scope/pkg@1.4.1", "npm:@other/pkg@1.4.1"),
    // Alias vs regular
    ("npm:foo@1.4.1", "1.4.1"),
    ("npm:foo@^1.4.1", "^1.4.1"),
    ("npm:@scope/pkg@~1.4.1", "~1.4.1"),
    // With prerelease
    ("npm:foo@1.4.1-alpha.1", "npm:bar@1.4.1-alpha.1"),
    ("npm:foo@^1.4.1-beta", "^1.4.1-beta"),
  ];
  for (a, b) in cases_same {
    assert!(
      Specifier::new(a).has_same_version_number_as(&Specifier::new(b)),
      "Expected {a} and {b} to have same version number"
    );
  }

  let cases_different: Vec<(&str, &str)> = vec![
    // Different versions
    ("npm:foo@1.4.1", "npm:foo@1.4.2"),
    ("npm:foo@^1.4.1", "npm:foo@^1.5.1"),
    ("npm:@scope/pkg@1.4.1", "npm:@scope/pkg@2.4.1"),
    // Alias without version (no node_version)
    ("npm:foo", "npm:bar"),
    ("npm:foo", "1.4.1"),
    ("npm:foo@1.4.1", "npm:bar"),
  ];
  for (a, b) in cases_different {
    assert!(
      !Specifier::new(a).has_same_version_number_as(&Specifier::new(b)),
      "Expected {a} and {b} to have different version numbers or no node_version"
    );
  }
}

#[test]
fn with_git_urls() {
  let cases_same: Vec<(&str, &str)> = vec![
    // Same version, same or different repos
    ("git@github.com:npm/cli.git#1.4.1", "git@github.com:npm/cli.git#1.4.1"),
    ("git@github.com:npm/cli.git#^1.4.1", "git@github.com:npm/cli.git#~1.4.1"),
    ("git@github.com:npm/cli.git#1.4.1", "git@github.com:other/pkg.git#1.4.1"),
    (
      "git+ssh://git@github.com/npm/cli#^1.4.1",
      "git+ssh://git@github.com/npm/cli#>=1.4.1",
    ),
    ("github:user/repo#1.4.1", "github:other/repo#1.4.1"),
    // Git vs regular
    ("git@github.com:npm/cli.git#1.4.1", "1.4.1"),
    ("git@github.com:npm/cli.git#^1.4.1", "^1.4.1"),
    ("github:user/repo#~1.4.1", "~1.4.1"),
    // With prerelease
    (
      "git@github.com:npm/cli.git#1.4.1-alpha.1",
      "git@github.com:npm/cli.git#1.4.1-alpha.1",
    ),
    ("git@github.com:npm/cli.git#^1.4.1-beta", "^1.4.1-beta"),
  ];
  for (a, b) in cases_same {
    assert!(
      Specifier::new(a).has_same_version_number_as(&Specifier::new(b)),
      "Expected {a} and {b} to have same version number"
    );
  }

  let cases_different: Vec<(&str, &str)> = vec![
    // Different versions
    ("git@github.com:npm/cli.git#1.4.1", "git@github.com:npm/cli.git#1.4.2"),
    ("git@github.com:npm/cli.git#^1.4.1", "git@github.com:npm/cli.git#^1.5.1"),
    ("github:user/repo#1.4.1", "github:user/repo#2.4.1"),
    // Git without semver tag (no node_version)
    ("git@github.com:npm/cli.git", "git@github.com:npm/cli.git#main"),
    ("git@github.com:npm/cli.git#main", "git@github.com:npm/cli.git#develop"),
    ("git@github.com:npm/cli.git", "1.4.1"),
    ("git@github.com:npm/cli.git#main", "^1.4.1"),
  ];
  for (a, b) in cases_different {
    assert!(
      !Specifier::new(a).has_same_version_number_as(&Specifier::new(b)),
      "Expected {a} and {b} to have different version numbers or no node_version"
    );
  }
}

#[test]
fn major_and_minor_variants() {
  // Major and Minor variants have padded node_versions (e.g., "1" ->
  // "1.999999.999999") So they should NOT match non-padded exact versions
  let cases_different: Vec<(&str, &str)> = vec![
    // Major vs exact (different versions after padding)
    ("1", "1.0.0"),
    ("1", "1.4.1"),
    // Major vs range (different versions)
    ("1", "^1.0.0"),
    ("^1", "^1.0.0"),
    // Minor vs exact (different versions after padding)
    ("1.4", "1.4.0"),
    ("1.4", "1.4.1"),
    // Minor vs range (different versions)
    ("1.4", "^1.4.0"),
    ("^1.4", "^1.4.0"),
    // Major vs minor (different versions)
    ("1", "1.4"),
    ("^1", "^1.4"),
  ];
  for (a, b) in cases_different {
    assert!(
      !Specifier::new(a).has_same_version_number_as(&Specifier::new(b)),
      "Expected {a} and {b} to have different version numbers (padded variants)"
    );
  }

  let cases_same: Vec<(&str, &str)> = vec![
    // Same major variants (both pad to same value)
    ("1", "^1"),
    ("1", "~1"),
    ("^1", "~1"),
    // Same minor variants (both pad to same value)
    ("1.4", "^1.4"),
    ("1.4", "~1.4"),
    ("^1.4", "~1.4"),
    // Major variant matches explicit padded version
    ("1", "1.999999.999999"),
    ("^1", "^1.999999.999999"),
    // Minor variant matches explicit padded version
    ("1.4", "1.4.999999"),
    ("^1.4", "^1.4.999999"),
  ];
  for (a, b) in cases_same {
    assert!(
      Specifier::new(a).has_same_version_number_as(&Specifier::new(b)),
      "Expected {a} and {b} to have same version number (same padded values)"
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
    ("latest", "1.4.1"),
    // URLs
    ("https://example.com/package.tgz", "https://other.com/pkg.tgz"),
    ("http://example.com/package.tgz", "1.4.1"),
    // File protocols
    ("file:../foo", "file:../bar"),
    ("file:./local", "1.4.1"),
    // Complex semver ranges
    (">=1.2.3 <2.0.0", ">=2.0.0 <3.0.0"),
    ("^1.2.3 || ^2.0.0", "^3.0.0 || ^4.0.0"),
    (">=1.2.3 <2.0.0", "1.4.1"),
    // Workspace protocol range-only
    ("workspace:*", "workspace:^"),
    ("workspace:~", "workspace:*"),
    // Npm alias without version
    ("npm:foo", "npm:bar"),
    // Git without semver tags
    ("git@github.com:npm/cli.git", "git@github.com:npm/cli.git#main"),
    (
      "git+ssh://git@github.com/npm/cli#feature",
      "git+ssh://git@github.com/npm/cli#develop",
    ),
    // Mixed - one with version, one without
    ("1.4.1", "latest"),
    ("^1.4.1", "https://example.com/package.tgz"),
    ("~1.4.1", "file:../foo"),
    ("npm:foo@1.4.1", "npm:bar"),
    ("git@github.com:npm/cli.git#1.4.1", "git@github.com:npm/cli.git"),
  ];
  for (a, b) in cases {
    assert!(
      !Specifier::new(a).has_same_version_number_as(&Specifier::new(b)),
      "Expected {a} and {b} to return false (at least one has no node_version)"
    );
  }
}

#[test]
fn self_comparison() {
  let cases: Vec<&str> = vec![
    "1.4.1",
    "^1.4.1",
    "~1.4.1",
    ">=1.4.1",
    "1.4.1-alpha.1",
    "^1.4.1-beta.2",
    "1",
    "1.4",
    "^1",
    "~1.4",
    "npm:foo@1.4.1",
    "npm:@scope/pkg@^1.4.1",
    "git@github.com:npm/cli.git#1.4.1",
    "github:user/repo#^1.4.1",
  ];
  for value in cases {
    let spec = Specifier::new(value);
    assert!(
      spec.has_same_version_number_as(&spec),
      "Expected {value} to have same version number as itself"
    );
  }
}

#[test]
fn comprehensive_exact_version_matches() {
  let version = "1.4.1";
  let variants: Vec<&str> = vec![
    "1.4.1",
    "^1.4.1",
    "~1.4.1",
    ">=1.4.1",
    ">1.4.1",
    "<=1.4.1",
    "<1.4.1",
    "npm:foo@1.4.1",
    "npm:@scope/pkg@1.4.1",
    "npm:foo@^1.4.1",
    "npm:foo@~1.4.1",
    "git@github.com:npm/cli.git#1.4.1",
    "git@github.com:npm/cli.git#^1.4.1",
    "git+ssh://git@github.com/npm/cli#~1.4.1",
    "github:user/repo#1.4.1",
  ];

  // Every variant should match every other variant
  for i in 0..variants.len() {
    for j in 0..variants.len() {
      let a = variants[i];
      let b = variants[j];
      assert!(
        Specifier::new(a).has_same_version_number_as(&Specifier::new(b)),
        "Expected {a} and {b} to have same version number ({version})"
      );
    }
  }
}

#[test]
fn comprehensive_prerelease_version_matches() {
  let version = "1.4.1-alpha.1";
  let variants: Vec<&str> = vec![
    "1.4.1-alpha.1",
    "^1.4.1-alpha.1",
    "~1.4.1-alpha.1",
    ">=1.4.1-alpha.1",
    "npm:foo@1.4.1-alpha.1",
    "npm:foo@^1.4.1-alpha.1",
    "git@github.com:npm/cli.git#1.4.1-alpha.1",
    "github:user/repo#~1.4.1-alpha.1",
  ];

  // Every variant should match every other variant
  for i in 0..variants.len() {
    for j in 0..variants.len() {
      let a = variants[i];
      let b = variants[j];
      assert!(
        Specifier::new(a).has_same_version_number_as(&Specifier::new(b)),
        "Expected {a} and {b} to have same version number ({version})"
      );
    }
  }
}

#[test]
fn edge_cases() {
  // Same version, all zeroes
  assert!(Specifier::new("0.0.0").has_same_version_number_as(&Specifier::new("^0.0.0")));
  assert!(Specifier::new("0.0.0").has_same_version_number_as(&Specifier::new("~0.0.0")));

  // Different when one component differs
  assert!(!Specifier::new("0.0.0").has_same_version_number_as(&Specifier::new("0.0.1")));
  assert!(!Specifier::new("0.0.0").has_same_version_number_as(&Specifier::new("0.1.0")));
  assert!(!Specifier::new("0.0.0").has_same_version_number_as(&Specifier::new("1.0.0")));

  // Large version numbers
  assert!(Specifier::new("999.999.999").has_same_version_number_as(&Specifier::new("^999.999.999")));

  // Complex prerelease identifiers
  assert!(Specifier::new("1.0.0-alpha.1.2.3").has_same_version_number_as(&Specifier::new("^1.0.0-alpha.1.2.3")));
  assert!(!Specifier::new("1.0.0-alpha.1.2.3").has_same_version_number_as(&Specifier::new("1.0.0-alpha.1.2.4")));
}

#[test]
fn none_variant() {
  // None variant (from "*" or empty) should not have node_version
  let cases: Vec<&str> = vec!["*", "latest", "1.4.1", "^1.4.1"];
  let none_spec = Specifier::new("*");

  for value in cases {
    assert!(
      !none_spec.has_same_version_number_as(&Specifier::new(value)),
      "Expected None variant (*) and {value} to return false"
    );
  }
}
