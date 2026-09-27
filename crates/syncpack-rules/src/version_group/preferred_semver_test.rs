use {
  crate::{
    test_fixtures::{arena_and_dependency, dep_type, instance_at},
    version_group::PreferredSemverGroup,
  },
  syncpack_model::group_selector::GroupSelector,
  syncpack_specifier::semver_range::SemverRange,
  syncpack_vocab::severity::SeverityMap,
};

fn group(prefer_highest: bool) -> PreferredSemverGroup {
  PreferredSemverGroup {
    selector: GroupSelector::new(vec![], vec![], "test".into(), vec![], vec![]),
    dependencies: vec![],
    prefer_highest,
    severity: SeverityMap::new(),
  }
}

#[test]
fn picks_the_highest_raw_version_when_no_semver_group_is_involved() {
  let deps_type = dep_type("/dependencies");
  let (arena, dependency) = arena_and_dependency(vec![
    instance_at("pkg-a", &deps_type, "lodash", "1.2.0"),
    instance_at("pkg-b", &deps_type, "lodash", "1.5.0"),
  ]);

  let highest = group(true).get_highest_or_lowest_specifier(&dependency, &arena);

  assert_eq!(highest.as_deref().map(|s| s.get_raw()), Some("1.5.0"));
}

#[test]
fn a_semver_group_widening_the_range_breaks_a_tie_at_the_same_version_number() {
  let deps_type = dep_type("/dependencies");
  let mut instances = vec![
    instance_at("pkg-a", &deps_type, "lodash", "1.2.0"),
    instance_at("pkg-b", &deps_type, "lodash", "1.2.0"),
  ];
  // Both instances are on 1.2.0, but pkg-b belongs to a semver group
  // preferring `^` — widening it to ^1.2.0 before comparison. Greedy-range
  // tiebreaker: see ".notes/reference/version-group-implementation-rules.md"
  // and `PreferredSemverGroup::get_highest_or_lowest_specifier`.
  instances[1].preferred_semver_range = Some(SemverRange::Minor);
  let (arena, dependency) = arena_and_dependency(instances);

  let highest = group(true).get_highest_or_lowest_specifier(&dependency, &arena);

  assert_eq!(highest.as_deref().map(|s| s.get_raw()), Some("^1.2.0"));
}

#[test]
fn prefer_lowest_picks_the_lowest_raw_version() {
  let deps_type = dep_type("/dependencies");
  let (arena, dependency) = arena_and_dependency(vec![
    instance_at("pkg-a", &deps_type, "lodash", "1.2.0"),
    instance_at("pkg-b", &deps_type, "lodash", "1.5.0"),
  ]);

  let lowest = group(false).get_highest_or_lowest_specifier(&dependency, &arena);

  assert_eq!(lowest.as_deref().map(|s| s.get_raw()), Some("1.2.0"));
}
