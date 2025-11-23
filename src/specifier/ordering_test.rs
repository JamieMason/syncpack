use {crate::specifier::Specifier, std::cmp::Ordering};

/// Tests for Ord, PartialOrd, and Eq trait implementations on Specifier
///
/// The ordering behavior matches the old Specifier:
/// 1. Primary sort: By version number (using node_semver::Version::cmp)
/// 2. Secondary sort: By range greediness when versions are equal
/// 3. None handling: Specifiers without versions sort before those with versions
///
/// Range greediness ranking (from SemverRange):
/// Lt(0) < Lte(1) < Exact(2) < Patch(3) < Minor(4) < Gte(5) < Gt(6) < Any(7)

// =============================================================================
// Basic Ordering Tests
// =============================================================================

#[test]
fn equal_specifiers_are_equal() {
  let a = Specifier::new("1.2.3");
  let b = Specifier::new("1.2.3");
  assert_eq!(a.cmp(&b), Ordering::Equal);
  assert_eq!(a, b);
}

#[test]
fn different_versions_compare_by_version() {
  let older = Specifier::new("1.0.0");
  let newer = Specifier::new("2.0.0");
  assert_eq!(older.cmp(&newer), Ordering::Less);
  assert_eq!(newer.cmp(&older), Ordering::Greater);
}

#[test]
fn semver_ordering_respects_numeric_comparison() {
  let cases = vec![
    ("0.0.1", "0.0.2"),
    ("0.1.0", "0.2.0"),
    ("1.0.0", "2.0.0"),
    ("1.0.0", "1.0.1"),
    ("1.0.0", "1.1.0"),
    ("1.2.3", "1.2.4"),
    ("1.2.3", "1.3.0"),
    ("1.2.3", "2.0.0"),
  ];

  for (older, newer) in cases {
    let a = Specifier::new(older);
    let b = Specifier::new(newer);
    assert_eq!(a.cmp(&b), Ordering::Less, "Expected {older} < {newer}");
    assert_eq!(b.cmp(&a), Ordering::Greater, "Expected {newer} > {older}");
  }
}

// =============================================================================
// Range Greediness Tests (Secondary Sort)
// =============================================================================

#[test]
fn same_version_sorts_by_range_greediness() {
  // Expected order: Lt(0) < Lte(1) < Exact(2) < Patch(3) < Minor(4) < Gte(5) < Gt(6) < Any(7)
  let specs = [
    Specifier::new("<1.0.0"),  // Lt - rank 0
    Specifier::new("<=1.0.0"), // Lte - rank 1
    Specifier::new("1.0.0"),   // Exact - rank 2
    Specifier::new("~1.0.0"),  // Patch - rank 3
    Specifier::new("^1.0.0"),  // Minor - rank 4
    Specifier::new(">=1.0.0"), // Gte - rank 5
    Specifier::new(">1.0.0"),  // Gt - rank 6
    Specifier::new("*"),       // Any - rank 7
  ];

  // Verify each is less than the next
  for i in 0..specs.len() - 1 {
    assert_eq!(
      specs[i].cmp(&specs[i + 1]),
      Ordering::Less,
      "Expected {:?} < {:?}",
      specs[i],
      specs[i + 1]
    );
  }
}

#[test]
fn sorting_same_version_different_ranges() {
  let mut specifiers = vec![
    Specifier::new("0.0.0"),
    Specifier::new("<0.0.0"),
    Specifier::new("*"),
    Specifier::new(">0.0.0"),
    Specifier::new(">=0.0.0"),
    Specifier::new("<=0.0.0"),
    Specifier::new("^0.0.0"),
    Specifier::new("~0.0.0"),
  ];

  let expected = vec![
    Specifier::new("<0.0.0"),
    Specifier::new("<=0.0.0"),
    Specifier::new("0.0.0"),
    Specifier::new("~0.0.0"),
    Specifier::new("^0.0.0"),
    Specifier::new(">=0.0.0"),
    Specifier::new(">0.0.0"),
    Specifier::new("*"),
  ];

  specifiers.sort();
  assert_eq!(specifiers, expected);
}

#[test]
fn range_greediness_tiebreaker_for_equal_versions() {
  // All have version 2.5.0, but different ranges
  let exact = Specifier::new("2.5.0");
  let patch = Specifier::new("~2.5.0");
  let minor = Specifier::new("^2.5.0");

  assert_eq!(exact.cmp(&patch), Ordering::Less);
  assert_eq!(patch.cmp(&minor), Ordering::Less);
  assert_eq!(exact.cmp(&minor), Ordering::Less);
}

// =============================================================================
// Sorting Multiple Versions and Ranges
// =============================================================================

#[test]
fn sorting_mixed_versions_and_ranges() {
  let mut specifiers = [
    Specifier::new("2.0.0"),
    Specifier::new("1.0.0"),
    Specifier::new("^1.5.0"),
    Specifier::new("~1.0.0"),
    Specifier::new("^1.0.0"),
    Specifier::new("1.5.0"),
  ];

  specifiers.sort();

  // Expected order:
  // 1.0.0 (exact, version 1.0.0)
  // ~1.0.0 (patch, version 1.0.0)
  // ^1.0.0 (minor, version 1.0.0)
  // 1.5.0 (exact, version 1.5.0)
  // ^1.5.0 (minor, version 1.5.0)
  // 2.0.0 (exact, version 2.0.0)
  assert_eq!(specifiers[0], Specifier::new("1.0.0"));
  assert_eq!(specifiers[1], Specifier::new("~1.0.0"));
  assert_eq!(specifiers[2], Specifier::new("^1.0.0"));
  assert_eq!(specifiers[3], Specifier::new("1.5.0"));
  assert_eq!(specifiers[4], Specifier::new("^1.5.0"));
  assert_eq!(specifiers[5], Specifier::new("2.0.0"));
}

#[test]
fn sorting_preserves_version_then_range_order() {
  let mut specifiers = [
    Specifier::new("3.0.0"),
    Specifier::new("^1.0.0"),
    Specifier::new("~2.0.0"),
    Specifier::new("1.0.0"),
    Specifier::new("2.0.0"),
    Specifier::new("^2.0.0"),
  ];

  specifiers.sort();

  assert_eq!(specifiers[0], Specifier::new("1.0.0"));
  assert_eq!(specifiers[1], Specifier::new("^1.0.0"));
  assert_eq!(specifiers[2], Specifier::new("2.0.0"));
  assert_eq!(specifiers[3], Specifier::new("~2.0.0"));
  assert_eq!(specifiers[4], Specifier::new("^2.0.0"));
  assert_eq!(specifiers[5], Specifier::new("3.0.0"));
}

// =============================================================================
// Non-Version Specifiers (sort before versioned)
// =============================================================================

#[test]
fn non_version_specifiers_sort_before_versioned() {
  let versioned = Specifier::new("1.0.0");
  let file = Specifier::new("file:../path");
  let url = Specifier::new("https://example.com/package.tgz");
  let tag = Specifier::new("alpha");
  let unsupported = Specifier::new("}invalid{");

  assert_eq!(file.cmp(&versioned), Ordering::Less);
  assert_eq!(url.cmp(&versioned), Ordering::Less);
  assert_eq!(tag.cmp(&versioned), Ordering::Less);
  assert_eq!(unsupported.cmp(&versioned), Ordering::Less);
}

#[test]
fn non_version_specifiers_are_equal_to_each_other() {
  let file = Specifier::new("file:../path");
  let url = Specifier::new("https://example.com/package.tgz");
  let tag = Specifier::new("alpha");

  // Non-version specifiers compare equal (no version to compare)
  assert_eq!(file.cmp(&url), Ordering::Equal);
  assert_eq!(file.cmp(&tag), Ordering::Equal);
  assert_eq!(url.cmp(&tag), Ordering::Equal);
}

#[test]
fn sorting_non_version_specifiers() {
  let mut specifiers = [
    Specifier::new("1.0.0"),
    Specifier::new("file:../path"),
    Specifier::new("alpha"),
    Specifier::new("workspace:^"),
    Specifier::new("2.0.0"),
  ];

  specifiers.sort();

  // Non-version specifiers should come first (order among them preserved by stable sort)
  // Then versioned specifiers in order
  assert_eq!(specifiers[0], Specifier::new("file:../path"));
  assert_eq!(specifiers[1], Specifier::new("alpha"));
  assert_eq!(specifiers[2], Specifier::new("workspace:^"));
  assert_eq!(specifiers[3], Specifier::new("1.0.0"));
  assert_eq!(specifiers[4], Specifier::new("2.0.0"));
}

#[test]
fn none_variant_sorts_before_versioned() {
  let none = Specifier::new("");
  let versioned = Specifier::new("1.0.0");

  assert_eq!(none.cmp(&versioned), Ordering::Less);
  assert_eq!(versioned.cmp(&none), Ordering::Greater);
}

// =============================================================================
// Workspace Protocol Tests
// =============================================================================

#[test]
fn unresolved_workspace_protocol_sorts_before_versioned() {
  let workspace = Specifier::new("workspace:^");
  let versioned = Specifier::new("1.0.0");

  assert_eq!(workspace.cmp(&versioned), Ordering::Less);
  assert_eq!(versioned.cmp(&workspace), Ordering::Greater);
}

#[test]
fn workspace_protocol_with_embedded_version_sorts_normally() {
  let workspace = Specifier::new("workspace:^1.5.0");
  let exact = Specifier::new("1.5.0");
  let minor = Specifier::new("^1.5.0");

  // workspace:^1.5.0 should behave like ^1.5.0
  // Both should be equal (same version, same range)
  assert_eq!(workspace.cmp(&minor), Ordering::Equal);

  // workspace:^1.5.0 > 1.5.0 (minor > exact)
  assert_eq!(workspace.cmp(&exact), Ordering::Greater);
}

// =============================================================================
// Shorthand Versions (Major, Minor) with HUGE padding
// =============================================================================

#[test]
fn major_version_shorthand_sorts_correctly() {
  let major = Specifier::new("1"); // Becomes 1.999999.999999
  let patch = Specifier::new("1.0.0");
  let high_patch = Specifier::new("1.999.999");

  // "1" (1.999999.999999) should be greater than any real 1.x.x version
  assert_eq!(major.cmp(&patch), Ordering::Greater);
  assert_eq!(major.cmp(&high_patch), Ordering::Greater);
}

#[test]
fn minor_version_shorthand_sorts_correctly() {
  let minor = Specifier::new("1.4"); // Becomes 1.4.999999
  let exact = Specifier::new("1.4.0");
  let high_patch = Specifier::new("1.4.999");

  // "1.4" (1.4.999999) should be greater than any real 1.4.x version
  assert_eq!(minor.cmp(&exact), Ordering::Greater);
  assert_eq!(minor.cmp(&high_patch), Ordering::Greater);
}

#[test]
fn major_shorthand_with_range() {
  let range_major = Specifier::new("^1"); // ^1.999999.999999
  let range_full = Specifier::new("^1.0.0");

  // Should compare as different versions
  assert_eq!(range_major.cmp(&range_full), Ordering::Greater);
}

#[test]
fn minor_shorthand_with_range() {
  let range_minor = Specifier::new("~1.2"); // ~1.2.999999
  let range_full = Specifier::new("~1.2.0");

  // Should compare as different versions
  assert_eq!(range_minor.cmp(&range_full), Ordering::Greater);
}

// =============================================================================
// Alias (npm:) Specifiers
// =============================================================================

#[test]
fn aliases_sort_by_version_and_range() {
  let alias1 = Specifier::new("npm:@jsr/std__fmt@1.0.0");
  let alias2 = Specifier::new("npm:@jsr/std__fmt@2.0.0");
  let alias3 = Specifier::new("npm:@jsr/std__fmt@^1.0.0");

  assert_eq!(alias1.cmp(&alias2), Ordering::Less);
  assert_eq!(alias1.cmp(&alias3), Ordering::Less); // exact < minor
  assert_eq!(alias3.cmp(&alias2), Ordering::Less); // 1.x.x < 2.x.x
}

#[test]
fn aliases_without_version_sort_after_those_with_version() {
  let alias_no_version = Specifier::new("npm:@jsr/std__fmt");
  let alias_with_version = Specifier::new("npm:@jsr/std__fmt@1.0.0");

  // Aliases without explicit version default to "*" which has HUGE version
  // So they sort AFTER versioned aliases
  assert_eq!(alias_no_version.cmp(&alias_with_version), Ordering::Greater);
}

#[test]
fn sorting_aliases_with_same_versions_different_ranges() {
  let mut specifiers = vec![
    Specifier::new("npm:@jsr/std__fmt@0.0.0"),
    Specifier::new("npm:@jsr/std__fmt@<0.0.0"),
    Specifier::new("npm:@jsr/std__fmt@*"),
    Specifier::new("npm:@jsr/std__fmt@>0.0.0"),
    Specifier::new("npm:@jsr/std__fmt@>=0.0.0"),
    Specifier::new("npm:@jsr/std__fmt@<=0.0.0"),
    Specifier::new("npm:@jsr/std__fmt@^0.0.0"),
    Specifier::new("npm:@jsr/std__fmt@~0.0.0"),
  ];

  let expected = vec![
    Specifier::new("npm:@jsr/std__fmt@<0.0.0"),
    Specifier::new("npm:@jsr/std__fmt@<=0.0.0"),
    Specifier::new("npm:@jsr/std__fmt@0.0.0"),
    Specifier::new("npm:@jsr/std__fmt@~0.0.0"),
    Specifier::new("npm:@jsr/std__fmt@^0.0.0"),
    Specifier::new("npm:@jsr/std__fmt@>=0.0.0"),
    Specifier::new("npm:@jsr/std__fmt@>0.0.0"),
    Specifier::new("npm:@jsr/std__fmt@*"),
  ];

  specifiers.sort();
  assert_eq!(specifiers, expected);
}

// =============================================================================
// Git Specifiers
// =============================================================================

#[test]
fn git_specifiers_with_semver_sort_normally() {
  let git1 = Specifier::new("github:user/repo#1.0.0");
  let git2 = Specifier::new("github:user/repo#2.0.0");
  let git3 = Specifier::new("github:user/repo#semver:^1.0.0");

  assert_eq!(git1.cmp(&git2), Ordering::Less);
  assert_eq!(git1.cmp(&git3), Ordering::Less); // exact < minor
}

#[test]
fn git_specifiers_without_semver_sort_before_those_with() {
  let git_no_semver = Specifier::new("github:user/repo");
  let git_with_semver = Specifier::new("github:user/repo#1.0.0");

  assert_eq!(git_no_semver.cmp(&git_with_semver), Ordering::Less);
}

// =============================================================================
// Prerelease Versions
// =============================================================================

#[test]
fn prerelease_versions_sort_correctly() {
  let stable = Specifier::new("1.0.0");
  let alpha = Specifier::new("1.0.0-alpha");
  let beta = Specifier::new("1.0.0-beta");
  let rc = Specifier::new("1.0.0-rc.1");

  // Prereleases come before stable
  assert_eq!(alpha.cmp(&stable), Ordering::Less);
  assert_eq!(beta.cmp(&stable), Ordering::Less);
  assert_eq!(rc.cmp(&stable), Ordering::Less);

  // Alphabetical ordering among prereleases
  assert_eq!(alpha.cmp(&beta), Ordering::Less);
  assert_eq!(alpha.cmp(&rc), Ordering::Less);
  assert_eq!(beta.cmp(&rc), Ordering::Less);
}

#[test]
fn multi_segment_prerelease_ordering() {
  let cases = vec![
    ("1.0.0-alpha.1", "1.0.0-alpha.1.1"),
    ("1.0.0-alpha.1.1", "1.0.0-alpha.1.2"),
    ("1.0.0-alpha.1.2.3", "1.0.0-alpha.1.2.3.4"),
  ];

  for (older, newer) in cases {
    let a = Specifier::new(older);
    let b = Specifier::new(newer);
    assert_eq!(a.cmp(&b), Ordering::Less, "Expected {older} < {newer}");
  }
}

#[test]
fn prerelease_with_ranges() {
  let exact = Specifier::new("1.0.0-alpha");
  let patch = Specifier::new("~1.0.0-alpha");
  let minor = Specifier::new("^1.0.0-alpha");

  // Same version, different ranges
  assert_eq!(exact.cmp(&patch), Ordering::Less);
  assert_eq!(patch.cmp(&minor), Ordering::Less);
}

// =============================================================================
// Complex Semver (multiple range expressions)
// =============================================================================

#[test]
fn complex_semver_without_simple_version_sorts_before_versioned() {
  let complex = Specifier::new(">=1.0.0 <2.0.0");
  let simple = Specifier::new("1.5.0");

  // Complex semver has no single node_version, so sorts before
  assert_eq!(complex.cmp(&simple), Ordering::Less);
}

// =============================================================================
// Latest/Any Specifier
// =============================================================================

#[test]
fn latest_with_version_sorts_by_version() {
  // Latest("*") has HUGE version (999999.999999.999999), should sort after versioned
  let latest = Specifier::new("*");
  let versioned = Specifier::new("1.0.0");

  assert_eq!(latest.cmp(&versioned), Ordering::Greater);
}

// =============================================================================
// PartialOrd Trait Tests
// =============================================================================

#[test]
fn partial_ord_delegates_to_ord() {
  let a = Specifier::new("1.0.0");
  let b = Specifier::new("2.0.0");

  assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
  assert_eq!(b.partial_cmp(&a), Some(Ordering::Greater));
  assert_eq!(a.partial_cmp(&a), Some(Ordering::Equal));
}

#[test]
fn partial_ord_always_returns_some() {
  let specs = vec![
    Specifier::new("1.0.0"),
    Specifier::new("file:../path"),
    Specifier::new("alpha"),
    Specifier::new("workspace:^"),
  ];

  for spec1 in &specs {
    for spec2 in &specs {
      assert!(spec1.partial_cmp(spec2).is_some());
    }
  }
}

// =============================================================================
// Eq Trait Tests
// =============================================================================

#[test]
fn eq_trait_matches_partial_eq() {
  let a = Specifier::new("1.2.3");
  let b = Specifier::new("1.2.3");
  let c = Specifier::new("1.2.4");

  // Eq is a marker trait, but we can verify it's consistent
  assert_eq!(a, b);
  assert_ne!(a, c);
}

#[test]
fn eq_reflexive() {
  let a = Specifier::new("1.2.3");
  assert_eq!(a, a);
}

#[test]
fn eq_symmetric() {
  let a = Specifier::new("1.2.3");
  let b = Specifier::new("1.2.3");

  assert_eq!(a, b);
  assert_eq!(b, a);
}

#[test]
fn eq_transitive() {
  let a = Specifier::new("1.2.3");
  let b = Specifier::new("1.2.3");
  let c = Specifier::new("1.2.3");

  assert_eq!(a, b);
  assert_eq!(b, c);
  assert_eq!(a, c);
}

// =============================================================================
// Comprehensive Sorting Test (Real-World Scenario)
// =============================================================================

#[test]
fn comprehensive_sorting() {
  let mut actual = vec![
    // Versioned specifiers with various ranges
    Specifier::new("1.0.0-beta.1"),
    Specifier::new("^1.0.0"),
    Specifier::new(">2.0.0"),
    Specifier::new("1.5.0"),
    Specifier::new("1.0.0"),
    Specifier::new("<=1.0.0"),
    Specifier::new("1.0.0-alpha"),
    Specifier::new("<1.0.0"),
    Specifier::new(">=1.5.0"),
    Specifier::new("~1.0.0"),
    Specifier::new("2.0.0"),
    // Shorthand versions
    Specifier::new("~1.2"), // Range minor shorthand
    Specifier::new("1.2"),  // Minor shorthand
    Specifier::new("1"),    // Major shorthand
    Specifier::new("^1"),   // Range major shorthand
    // Aliases
    Specifier::new("npm:typescript@*"),
    Specifier::new("npm:react@1.0.0"),
    Specifier::new("npm:lodash@^4.17.0"),
    Specifier::new("npm:jest"), // Alias without version
    Specifier::new("npm:vue@~2.6.0"),
    // Git specifiers
    Specifier::new("github:user/repo#1.0.0"),
    Specifier::new("github:user/repo#semver:^1.5.0"),
    Specifier::new("github:user/repo"), // Git without version
    // Non-version specifiers
    Specifier::new("file:../local"),
    Specifier::new("https://example.com/package.tgz"),
    Specifier::new("alpha"),
    Specifier::new("beta"),
    Specifier::new("workspace:^"),
    Specifier::new("workspace:*"),
    Specifier::new(""), // None variant
    // Latest
    Specifier::new("*"),
    Specifier::new("latest"),
    // Complex semver (no simple version)
    Specifier::new(">=1.0.0 <2.0.0"),
    // Unsupported
    Specifier::new("}invalid{"),
  ];

  actual.sort();

  // Expected order after sorting:
  // 1. Non-version specifiers (file, url, tag, workspace without version, git without version, alias without version, none, complex,
  //    unsupported) - all compare as Equal
  // 2. Versioned specifiers sorted by: a) Version number (ascending) b) Range greediness (for same version): Lt < Lte < Exact < Patch <
  //    Minor < Gte < Gt < Any
  // 3. Latest (*) sorts last due to HUGE version (999999.999999.999999)
  let expected = vec![
    // Non-version specifiers come first
    Specifier::new("github:user/repo"),                // Git without version
    Specifier::new("file:../local"),                   // File
    Specifier::new("https://example.com/package.tgz"), // Url
    Specifier::new("alpha"),                           // Tag
    Specifier::new("beta"),                            // Tag
    Specifier::new("workspace:^"),                     // Workspace without version
    Specifier::new("workspace:*"),                     // Workspace without version
    Specifier::new(""),                                // None
    Specifier::new(">=1.0.0 <2.0.0"),                  // Complex semver
    Specifier::new("}invalid{"),                       // Unsupported
    // Versioned specifiers sorted by version then range greediness
    Specifier::new("1.0.0-alpha"),                    // Prerelease < stable
    Specifier::new("1.0.0-beta.1"),                   // Prerelease < stable
    Specifier::new("<1.0.0"),                         // Lt (rank 0)
    Specifier::new("<=1.0.0"),                        // Lte (rank 1)
    Specifier::new("1.0.0"),                          // Exact (rank 2)
    Specifier::new("npm:react@1.0.0"),                // Exact (rank 2)
    Specifier::new("github:user/repo#1.0.0"),         // Exact (rank 2)
    Specifier::new("~1.0.0"),                         // Patch (rank 3)
    Specifier::new("^1.0.0"),                         // Minor (rank 4)
    Specifier::new("1.2"),                            // Minor shorthand (1.2.999999)
    Specifier::new("~1.2"),                           // Range minor shorthand (~1.2.999999)
    Specifier::new("1.5.0"),                          // Exact
    Specifier::new("github:user/repo#semver:^1.5.0"), // Minor
    Specifier::new(">=1.5.0"),                        // Gte (rank 5)
    Specifier::new("1"),                              // Major shorthand (1.999999.999999)
    Specifier::new("^1"),                             // Range major shorthand (^1.999999.999999)
    Specifier::new("2.0.0"),                          // Exact
    Specifier::new(">2.0.0"),                         // Gt (rank 6)
    Specifier::new("npm:vue@~2.6.0"),                 // Version 2.6.0
    Specifier::new("npm:lodash@^4.17.0"),             // Version 4.17.0
    // Latest with HUGE version (999999.999999.999999)
    Specifier::new("npm:jest"),         // Alias without version (defaults to *)
    Specifier::new("npm:typescript@*"), // Alias with explicit *
    Specifier::new("*"),                // Latest
    Specifier::new("latest"),           // Latest
  ];

  assert_eq!(actual.len(), expected.len(), "Actual and expected have different lengths");

  // Check all items except the last 4 HUGE version items (which can be in any order since they're equal)
  for i in 0..(expected.len() - 4) {
    assert_eq!(
      &actual[i], &expected[i],
      "Mismatch at index {}: expected {:?}, got {:?}",
      i, expected[i], actual[i]
    );
  }

  // The last 4 items should all be HUGE version items, but order doesn't matter
  // They are: npm:jest, npm:typescript@*, *, latest
  let last_four_actual: Vec<String> = actual.iter().skip(expected.len() - 4).map(|s| s.get_raw().to_string()).collect();
  let last_four_expected: Vec<String> = expected.iter().skip(expected.len() - 4).map(|s| s.get_raw().to_string()).collect();

  // Sort both to compare sets rather than order
  let mut last_four_actual_sorted = last_four_actual.clone();
  let mut last_four_expected_sorted = last_four_expected.clone();
  last_four_actual_sorted.sort();
  last_four_expected_sorted.sort();

  assert_eq!(
    last_four_actual_sorted, last_four_expected_sorted,
    "Last 4 items should be the same set (in any order)"
  );
}

// =============================================================================
// Edge Cases
// =============================================================================

#[test]
fn comparing_specifier_with_itself() {
  let spec = Specifier::new("1.2.3");
  assert_eq!(spec.cmp(&spec), Ordering::Equal);
}

#[test]
fn stable_sort_preserves_equal_elements_order() {
  let mut specifiers = vec![
    Specifier::new("file:path1"),
    Specifier::new("file:path2"),
    Specifier::new("alpha"),
    Specifier::new("beta"),
  ];

  let original_order = specifiers.clone();
  specifiers.sort();

  // All non-version specifiers compare equal, so stable sort preserves order
  assert_eq!(specifiers, original_order);
}

#[test]
fn unsupported_specifiers_sort_consistently() {
  let unsupported1 = Specifier::new("}invalid{");
  let unsupported2 = Specifier::new("@@@");
  let versioned = Specifier::new("1.0.0");

  // Both unsupported sort before versioned
  assert_eq!(unsupported1.cmp(&versioned), Ordering::Less);
  assert_eq!(unsupported2.cmp(&versioned), Ordering::Less);

  // Unsupported specifiers compare equal to each other
  assert_eq!(unsupported1.cmp(&unsupported2), Ordering::Equal);
}
