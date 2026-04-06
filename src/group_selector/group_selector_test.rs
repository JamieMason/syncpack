use {
  crate::{
    dependency::{DependencyType, Strategy},
    group_selector::GroupSelector,
    instance::InstanceDescriptor,
    packages::PackageIdx,
    specifier::Specifier,
  },
};

fn make_dep_type() -> DependencyType {
  DependencyType {
    name: "prod".to_string(),
    name_path: None,
    path: "/dependencies".to_string(),
    strategy: Strategy::VersionsByName,
  }
}

fn descriptor(name: &str, is_local_dependency: bool) -> InstanceDescriptor {
  InstanceDescriptor {
    dependency_type: make_dep_type(),
    internal_name: name.to_string(),
    is_local_dependency,
    matches_cli_filter: false,
    name: name.to_string(),
    package_idx: PackageIdx(0),
    specifier: Specifier::new("1.0.0"),
  }
}

fn selector(deps: Vec<&str>) -> GroupSelector {
  GroupSelector::new(
    deps.into_iter().map(|s| s.to_string()).collect(),
    vec![],
    "test".to_string(),
    vec![],
    vec![],
  )
}

const PKG_NAME: &str = "owner-pkg";

#[test]
fn local_keyword_includes_local_deps() {
  let s = selector(vec!["$LOCAL"]);
  assert!(s.can_add(&descriptor("bar", true), PKG_NAME));
  assert!(!s.can_add(&descriptor("baz", false), PKG_NAME));
}

#[test]
fn not_local_keyword_excludes_local_deps() {
  let s = selector(vec!["!$LOCAL"]);
  assert!(!s.can_add(&descriptor("bar", true), PKG_NAME));
  assert!(s.can_add(&descriptor("baz", false), PKG_NAME));
}

#[test]
fn local_plus_named_pattern_matches_either() {
  // $LOCAL OR named pattern — either is sufficient
  let s = selector(vec!["$LOCAL", "react"]);
  assert!(s.can_add(&descriptor("bar", true), PKG_NAME)); // local dep
  assert!(s.can_add(&descriptor("react", false), PKG_NAME)); // matches pattern
  assert!(!s.can_add(&descriptor("webpack", false), PKG_NAME)); // neither
}

#[test]
fn not_local_excludes_even_if_named_pattern_matches() {
  // !$LOCAL exclusion wins over named include patterns
  let s = selector(vec!["!$LOCAL", "react"]);
  assert!(!s.can_add(&descriptor("react", true), PKG_NAME)); // local: excluded
  assert!(s.can_add(&descriptor("react", false), PKG_NAME)); // not local: ok
  assert!(!s.can_add(&descriptor("webpack", false), PKG_NAME)); // neither matches include
}

#[test]
fn empty_includes_with_not_local_includes_non_local_excludes_local() {
  // !$LOCAL only → non-local included by default, local excluded
  let s = selector(vec!["!$LOCAL"]);
  assert!(s.can_add(&descriptor("anything", false), PKG_NAME));
  assert!(!s.can_add(&descriptor("local-pkg", true), PKG_NAME));
}

#[test]
fn dollar_local_literal_not_matched_as_pattern() {
  // A dep literally named "$LOCAL" should not be matched by the $LOCAL keyword
  // (it won't exist in practice, but verify it's not treated as a glob pattern)
  let s = selector(vec!["$LOCAL"]);
  // "$LOCAL" as a name with is_local_dependency=false should NOT match
  assert!(!s.can_add(&descriptor("$LOCAL", false), PKG_NAME));
  // A real local dep should match
  assert!(s.can_add(&descriptor("some-local-pkg", true), PKG_NAME));
}
