use crate::{cli::UpdateTarget, specifier::Specifier};

#[test]
fn for_latest() {
  let cases: Vec<(&str, &str, bool)> = vec![
    // Eligible: Patch differences
    ("1.0.1", "1.0.0", true),
    ("1.0.2", "1.0.0", true),
    ("1.4.1", "1.4.0", true),
    // Eligible: Minor differences
    ("1.1.0", "1.0.0", true),
    ("1.2.0", "1.0.0", true),
    ("1.5.0", "1.4.0", true),
    // Eligible: Major differences
    ("2.0.0", "1.0.0", true),
    ("3.0.0", "1.0.0", true),
    ("3.0.0", "2.4.1", true),
    // Eligible: Multiple component differences
    ("1.1.1", "1.0.0", true),
    ("2.1.0", "1.0.0", true),
    ("2.1.1", "1.0.0", true),
    ("2.0.0", "1.4.5", true),
    // Eligible: Large version jumps
    ("10.0.0", "1.0.0", true),
    ("100.0.0", "1.0.0", true),
    // Eligible: With ranges
    ("^2.0.0", "^1.0.0", true),
    ("~2.0.0", "~1.0.0", true),
    (">=2.0.0", ">=1.0.0", true),
    // Eligible: Mixed
    ("2.0.0", "^1.0.0", true),
    ("^2.0.0", "1.0.0", true),
    // Not eligible: Same versions
    ("1.0.0", "1.0.0", false),
    ("^1.0.0", "^1.0.0", false),
    ("~1.0.0", "~1.0.0", false),
    // Not eligible: Older versions
    ("1.0.0", "1.0.1", false),
    ("1.0.0", "1.1.0", false),
    ("1.0.0", "2.0.0", false),
    ("1.9.9", "2.0.0", false),
    ("^1.0.0", "^2.0.0", false),
    // Not eligible: Without node_version
    ("latest", "1.0.0", false),
    ("1.0.0", "latest", false),
    ("https://example.com/package.tgz", "1.0.0", false),
  ];
  for (update, current, expected) in cases {
    assert_eq!(
      Specifier::new(update).is_eligible_update_for(&Specifier::new(current), &UpdateTarget::Latest),
      expected,
      "Expected {update} to {}be eligible update for {current} (target: Latest)",
      if expected { "" } else { "NOT " }
    );
  }
}

#[test]
fn for_minor() {
  let cases: Vec<(&str, &str, bool)> = vec![
    // Eligible: Same major, patch differences
    ("1.0.1", "1.0.0", true),
    ("1.0.5", "1.0.0", true),
    ("1.4.6", "1.4.5", true),
    // Eligible: Same major, minor differences
    ("1.1.0", "1.0.0", true),
    ("1.2.0", "1.0.0", true),
    ("1.5.0", "1.4.0", true),
    // Eligible: Same major, minor and patch differences
    ("1.1.1", "1.0.0", true),
    ("1.5.10", "1.4.5", true),
    // Eligible: With ranges
    ("^1.1.0", "^1.0.0", true),
    ("~1.0.1", "~1.0.0", true),
    (">=1.5.0", ">=1.4.0", true),
    // Eligible: Mixed
    ("1.1.0", "^1.0.0", true),
    ("^1.5.0", "1.4.0", true),
    // Eligible: Prerelease
    ("1.0.0", "1.0.0-alpha.1", true),
    ("1.0.1", "1.0.0-rc.1", true),
    ("1.1.0", "1.0.0-beta.1", true),
    // Not eligible: Different major
    ("2.0.0", "1.0.0", false),
    ("2.0.0", "1.9.9", false),
    ("3.0.0", "1.0.0", false),
    ("10.0.0", "5.0.0", false),
    ("^2.0.0", "^1.0.0", false),
    ("~2.0.0", "~1.5.0", false),
    ("2.0.0", "^1.0.0", false),
    ("^2.0.0", "1.0.0", false),
    // Not eligible: Same version
    ("1.0.0", "1.0.0", false),
    ("^1.4.0", "^1.4.0", false),
    // Not eligible: Older (same major)
    ("1.0.0", "1.0.1", false),
    ("1.0.0", "1.1.0", false),
    ("1.4.0", "1.5.0", false),
    ("^1.0.0", "^1.1.0", false),
    // Not eligible: Without node_version
    ("latest", "1.0.0", false),
    ("1.0.0", "latest", false),
  ];
  for (update, current, expected) in cases {
    assert_eq!(
      Specifier::new(update).is_eligible_update_for(&Specifier::new(current), &UpdateTarget::Minor),
      expected,
      "Expected {update} to {}be eligible update for {current} (target: Minor)",
      if expected { "" } else { "NOT " }
    );
  }
}

#[test]
fn for_patch() {
  let cases: Vec<(&str, &str, bool)> = vec![
    // Eligible: Same major.minor, patch differences
    ("1.0.1", "1.0.0", true),
    ("1.0.2", "1.0.0", true),
    ("1.0.5", "1.0.0", true),
    ("1.4.6", "1.4.5", true),
    ("1.4.10", "1.4.5", true),
    ("2.3.1", "2.3.0", true),
    // Eligible: With ranges
    ("^1.0.1", "^1.0.0", true),
    ("~1.4.6", "~1.4.5", true),
    (">=1.0.1", ">=1.0.0", true),
    // Eligible: Mixed
    ("1.0.1", "^1.0.0", true),
    ("^1.4.6", "1.4.5", true),
    // Eligible: Prerelease
    ("1.0.0", "1.0.0-alpha.1", true),
    ("1.0.0-beta.1", "1.0.0-alpha.1", true),
    ("1.0.1", "1.0.0-rc.1", true),
    // Not eligible: Different minor (same major)
    ("1.1.0", "1.0.0", false),
    ("1.2.0", "1.0.9", false),
    ("1.5.0", "1.4.0", false),
    ("1.5.1", "1.4.5", false),
    ("2.4.0", "2.3.5", false),
    ("^1.1.0", "^1.0.0", false),
    ("~1.5.0", "~1.4.5", false),
    // Not eligible: Different major
    ("2.0.0", "1.0.0", false),
    ("2.0.0", "1.9.9", false),
    ("3.0.0", "1.5.5", false),
    ("10.0.0", "5.3.2", false),
    ("^2.0.0", "^1.0.0", false),
    ("~2.0.0", "~1.5.5", false),
    // Not eligible: Same version
    ("1.0.0", "1.0.0", false),
    ("1.4.5", "1.4.5", false),
    ("^1.0.0", "^1.0.0", false),
    // Not eligible: Older (same major.minor)
    ("1.0.0", "1.0.1", false),
    ("1.4.5", "1.4.6", false),
    ("1.0.99", "1.0.100", false),
    ("^1.0.0", "^1.0.1", false),
    // Not eligible: Without node_version
    ("latest", "1.0.0", false),
    ("1.0.0", "latest", false),
  ];
  for (update, current, expected) in cases {
    assert_eq!(
      Specifier::new(update).is_eligible_update_for(&Specifier::new(current), &UpdateTarget::Patch),
      expected,
      "Expected {update} to {}be eligible update for {current} (target: Patch)",
      if expected { "" } else { "NOT " }
    );
  }
}

#[test]
fn with_workspace_protocol() {
  let cases: Vec<(&str, &str, UpdateTarget, bool)> = vec![
    // Latest target
    ("workspace:2.0.0", "workspace:1.0.0", UpdateTarget::Latest, true),
    ("workspace:^2.0.0", "workspace:^1.0.0", UpdateTarget::Latest, true),
    ("workspace:2.0.0", "1.0.0", UpdateTarget::Latest, true),
    ("2.0.0", "workspace:1.0.0", UpdateTarget::Latest, true),
    // Minor target
    ("workspace:1.1.0", "workspace:1.0.0", UpdateTarget::Minor, true),
    ("workspace:^1.5.0", "workspace:^1.4.0", UpdateTarget::Minor, true),
    ("workspace:1.1.0", "1.0.0", UpdateTarget::Minor, true),
    ("1.1.0", "workspace:1.0.0", UpdateTarget::Minor, true),
    // Patch target
    ("workspace:1.0.1", "workspace:1.0.0", UpdateTarget::Patch, true),
    ("workspace:^1.4.6", "workspace:^1.4.5", UpdateTarget::Patch, true),
    ("workspace:1.0.1", "1.0.0", UpdateTarget::Patch, true),
    ("1.0.1", "workspace:1.0.0", UpdateTarget::Patch, true),
  ];
  for (update, current, target, expected) in cases {
    assert_eq!(
      Specifier::new(update).is_eligible_update_for(&Specifier::new(current), &target),
      expected,
      "Expected {update} to {}be eligible update for {current} (target: {:?})",
      if expected { "" } else { "NOT " },
      target
    );
  }
}

#[test]
fn with_npm_alias() {
  let cases: Vec<(&str, &str, UpdateTarget, bool)> = vec![
    // Latest target
    ("npm:foo@2.0.0", "npm:foo@1.0.0", UpdateTarget::Latest, true),
    ("npm:foo@2.0.0", "npm:bar@1.0.0", UpdateTarget::Latest, true),
    ("npm:@scope/pkg@^2.0.0", "npm:@scope/pkg@^1.0.0", UpdateTarget::Latest, true),
    ("npm:foo@2.0.0", "1.0.0", UpdateTarget::Latest, true),
    ("2.0.0", "npm:foo@1.0.0", UpdateTarget::Latest, true),
    // Minor target
    ("npm:foo@1.1.0", "npm:foo@1.0.0", UpdateTarget::Minor, true),
    ("npm:foo@1.5.0", "npm:bar@1.4.0", UpdateTarget::Minor, true),
    ("npm:@scope/pkg@^1.5.0", "npm:@scope/pkg@^1.4.0", UpdateTarget::Minor, true),
    ("npm:foo@1.1.0", "1.0.0", UpdateTarget::Minor, true),
    ("1.1.0", "npm:foo@1.0.0", UpdateTarget::Minor, true),
    // Patch target
    ("npm:foo@1.0.1", "npm:foo@1.0.0", UpdateTarget::Patch, true),
    ("npm:foo@1.4.6", "npm:bar@1.4.5", UpdateTarget::Patch, true),
    ("npm:@scope/pkg@^1.4.6", "npm:@scope/pkg@^1.4.5", UpdateTarget::Patch, true),
    ("npm:foo@1.0.1", "1.0.0", UpdateTarget::Patch, true),
    ("1.0.1", "npm:foo@1.0.0", UpdateTarget::Patch, true),
  ];
  for (update, current, target, expected) in cases {
    assert_eq!(
      Specifier::new(update).is_eligible_update_for(&Specifier::new(current), &target),
      expected,
      "Expected {update} to {}be eligible update for {current} (target: {:?})",
      if expected { "" } else { "NOT " },
      target
    );
  }
}

#[test]
fn with_git_urls() {
  let cases: Vec<(&str, &str, UpdateTarget, bool)> = vec![
    // Latest target
    (
      "git@github.com:npm/cli.git#2.0.0",
      "git@github.com:npm/cli.git#1.0.0",
      UpdateTarget::Latest,
      true,
    ),
    (
      "git+ssh://git@github.com/npm/cli#^2.0.0",
      "git+ssh://git@github.com/npm/cli#^1.0.0",
      UpdateTarget::Latest,
      true,
    ),
    ("github:user/repo#2.0.0", "github:other/repo#1.0.0", UpdateTarget::Latest, true),
    ("git@github.com:npm/cli.git#2.0.0", "1.0.0", UpdateTarget::Latest, true),
    ("2.0.0", "git@github.com:npm/cli.git#1.0.0", UpdateTarget::Latest, true),
    // Minor target
    (
      "git@github.com:npm/cli.git#1.1.0",
      "git@github.com:npm/cli.git#1.0.0",
      UpdateTarget::Minor,
      true,
    ),
    (
      "git+ssh://git@github.com/npm/cli#^1.5.0",
      "git+ssh://git@github.com/npm/cli#^1.4.0",
      UpdateTarget::Minor,
      true,
    ),
    ("github:user/repo#1.1.0", "github:user/repo#1.0.0", UpdateTarget::Minor, true),
    ("git@github.com:npm/cli.git#1.1.0", "1.0.0", UpdateTarget::Minor, true),
    ("1.1.0", "git@github.com:npm/cli.git#1.0.0", UpdateTarget::Minor, true),
    // Patch target
    (
      "git@github.com:npm/cli.git#1.0.1",
      "git@github.com:npm/cli.git#1.0.0",
      UpdateTarget::Patch,
      true,
    ),
    (
      "git+ssh://git@github.com/npm/cli#^1.4.6",
      "git+ssh://git@github.com/npm/cli#^1.4.5",
      UpdateTarget::Patch,
      true,
    ),
    ("github:user/repo#1.0.1", "github:user/repo#1.0.0", UpdateTarget::Patch, true),
    ("git@github.com:npm/cli.git#1.0.1", "1.0.0", UpdateTarget::Patch, true),
    ("1.0.1", "git@github.com:npm/cli.git#1.0.0", UpdateTarget::Patch, true),
  ];
  for (update, current, target, expected) in cases {
    assert_eq!(
      Specifier::new(update).is_eligible_update_for(&Specifier::new(current), &target),
      expected,
      "Expected {update} to {}be eligible update for {current} (target: {:?})",
      if expected { "" } else { "NOT " },
      target
    );
  }
}

#[test]
fn with_prerelease_versions() {
  let cases: Vec<(&str, &str, UpdateTarget, bool)> = vec![
    // Eligible: Latest target
    ("1.0.0", "1.0.0-alpha.1", UpdateTarget::Latest, true),
    ("1.0.0", "1.0.0-beta.1", UpdateTarget::Latest, true),
    ("1.0.0-beta.1", "1.0.0-alpha.1", UpdateTarget::Latest, true),
    ("1.0.0-rc.1", "1.0.0-beta.1", UpdateTarget::Latest, true),
    ("2.0.0-alpha.1", "1.0.0", UpdateTarget::Latest, true),
    ("2.0.0", "1.0.0-rc.1", UpdateTarget::Latest, true),
    // Eligible: Minor target
    ("1.0.0", "1.0.0-alpha.1", UpdateTarget::Minor, true),
    ("1.0.0-beta.1", "1.0.0-alpha.1", UpdateTarget::Minor, true),
    ("1.1.0-alpha.1", "1.0.0", UpdateTarget::Minor, true),
    ("1.1.0", "1.0.0-rc.1", UpdateTarget::Minor, true),
    // Eligible: Patch target
    ("1.0.0", "1.0.0-alpha.1", UpdateTarget::Patch, true),
    ("1.0.0-beta.1", "1.0.0-alpha.1", UpdateTarget::Patch, true),
    ("1.0.0-rc.1", "1.0.0-beta.1", UpdateTarget::Patch, true),
    ("1.0.1-alpha.1", "1.0.0", UpdateTarget::Patch, true),
    ("1.0.1", "1.0.0-rc.1", UpdateTarget::Patch, true),
    // Not eligible: Minor target (different major)
    ("2.0.0-alpha.1", "1.0.0", UpdateTarget::Minor, false),
    ("2.0.0", "1.9.9-rc.1", UpdateTarget::Minor, false),
    // Not eligible: Patch target (different major or minor)
    ("1.1.0-alpha.1", "1.0.0", UpdateTarget::Patch, false),
    ("1.1.0", "1.0.9-rc.1", UpdateTarget::Patch, false),
    ("2.0.0-alpha.1", "1.0.0", UpdateTarget::Patch, false),
  ];
  for (update, current, target, expected) in cases {
    assert_eq!(
      Specifier::new(update).is_eligible_update_for(&Specifier::new(current), &target),
      expected,
      "Expected {update} to {}be eligible update for {current} (target: {:?})",
      if expected { "" } else { "NOT " },
      target
    );
  }
}

#[test]
fn major_and_minor_variants() {
  let cases: Vec<(&str, &str, UpdateTarget, bool)> = vec![
    // Latest target
    // Major variants have padded versions (e.g., "1" -> "1.999999.999999")
    ("2", "1.0.0", UpdateTarget::Latest, true),
    ("^2", "^1.0.0", UpdateTarget::Latest, true),
    ("1", "1.0.0", UpdateTarget::Latest, true),
    ("1.5", "1.4.0", UpdateTarget::Latest, true),
    ("1.4", "1.4.0", UpdateTarget::Latest, true),
    // Minor target
    // Minor variants have padded patch (e.g., "1.4" -> "1.4.999999")
    ("2", "1.0.0", UpdateTarget::Minor, false),
    ("^2", "^1.0.0", UpdateTarget::Minor, false),
    ("1", "1.0.0", UpdateTarget::Minor, true),
    ("1.5", "1.4.0", UpdateTarget::Minor, true),
    ("1.4", "1.4.0", UpdateTarget::Minor, true),
    // Patch target - "1.4" (1.4.HUGE) matches anything in 1.4.* range
    ("1", "1.0.0", UpdateTarget::Patch, false),
    ("1.5", "1.4.0", UpdateTarget::Patch, false),
    ("1.4", "1.4.0", UpdateTarget::Patch, true),
  ];
  for (update, current, target, expected) in cases {
    assert_eq!(
      Specifier::new(update).is_eligible_update_for(&Specifier::new(current), &target),
      expected,
      "Expected {update} to {}be eligible update for {current} (target: {:?})",
      if expected { "" } else { "NOT " },
      target
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

  let targets: Vec<UpdateTarget> = vec![UpdateTarget::Latest, UpdateTarget::Minor, UpdateTarget::Patch];

  // Without version as update
  for without in &without_version {
    for with in &with_version {
      for target in &targets {
        assert!(
          !Specifier::new(without).is_eligible_update_for(&Specifier::new(with), target),
          "Expected {without} to NOT be eligible update for {with} (target: {target:?}, no node_version)"
        );
      }
    }
  }

  // Without version as current
  for with in &with_version {
    for without in &without_version {
      for target in &targets {
        assert!(
          !Specifier::new(with).is_eligible_update_for(&Specifier::new(without), target),
          "Expected {with} to NOT be eligible update for {without} (target: {target:?}, no node_version)"
        );
      }
    }
  }

  // Without version as both
  for a in &without_version {
    for b in &without_version {
      for target in &targets {
        assert!(
          !Specifier::new(a).is_eligible_update_for(&Specifier::new(b), target),
          "Expected {a} to NOT be eligible update for {b} (target: {target:?}, no node_version)"
        );
      }
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
    "1.0.0",
    "1.0.1",
    "1.1.0",
    "1.4.0",
    "1.4.1",
    "2.0.0",
    "10.0.0",
  ];

  // For Latest: each version should be eligible update for all earlier versions
  for i in 0..versions.len() {
    for j in (i + 1)..versions.len() {
      let current = versions[i];
      let update = versions[j];
      assert!(
        Specifier::new(update).is_eligible_update_for(&Specifier::new(current), &UpdateTarget::Latest),
        "Expected {update} to be eligible update for {current} (target: Latest)"
      );
      assert!(
        !Specifier::new(current).is_eligible_update_for(&Specifier::new(update), &UpdateTarget::Latest),
        "Expected {current} to NOT be eligible update for {update} (target: Latest)"
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

  let targets: Vec<UpdateTarget> = vec![UpdateTarget::Latest, UpdateTarget::Minor, UpdateTarget::Patch];

  for value in cases {
    let spec = Specifier::new(value);
    for target in &targets {
      assert!(
        !spec.is_eligible_update_for(&spec, target),
        "Expected {value} to NOT be eligible update for itself (target: {target:?})"
      );
    }
  }
}

#[test]
fn edge_cases() {
  let cases: Vec<(&str, &str, UpdateTarget, bool)> = vec![
    // Zero versions
    ("0.0.1", "0.0.0", UpdateTarget::Latest, true),
    ("0.0.1", "0.0.0", UpdateTarget::Minor, true),
    ("0.0.1", "0.0.0", UpdateTarget::Patch, true),
    ("0.1.0", "0.0.0", UpdateTarget::Latest, true),
    ("0.1.0", "0.0.0", UpdateTarget::Minor, true),
    ("0.1.0", "0.0.0", UpdateTarget::Patch, false),
    ("1.0.0", "0.9.9", UpdateTarget::Latest, true),
    ("1.0.0", "0.9.9", UpdateTarget::Minor, false),
    ("1.0.0", "0.9.9", UpdateTarget::Patch, false),
    // Large version numbers
    ("999.999.999", "1.0.0", UpdateTarget::Latest, true),
    ("1.999.999", "1.0.0", UpdateTarget::Minor, true),
    ("1.0.999", "1.0.0", UpdateTarget::Patch, true),
    // Complex prerelease identifiers
    ("1.0.0-alpha.1.2.4", "1.0.0-alpha.1.2.3", UpdateTarget::Latest, true),
    ("1.0.0-alpha.1.2.4", "1.0.0-alpha.1.2.3", UpdateTarget::Minor, true),
    ("1.0.0-alpha.1.2.4", "1.0.0-alpha.1.2.3", UpdateTarget::Patch, true),
  ];
  for (update, current, target, expected) in cases {
    assert_eq!(
      Specifier::new(update).is_eligible_update_for(&Specifier::new(current), &target),
      expected,
      "Expected {update} to {}be eligible update for {current} (target: {:?})",
      if expected { "" } else { "NOT " },
      target
    );
  }
}
