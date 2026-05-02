use {
  crate::{
    dependency::{DependencyType, Strategy},
    group_selector::GroupSelector,
    instance::InstanceDescriptor,
    source::SourceKind,
    sources::SourceIdx,
    specifier::Specifier,
  },
  std::rc::Rc,
};

fn make_dep_type() -> DependencyType {
  DependencyType {
    name: "prod".to_string(),
    name_path: None,
    path: "/dependencies".to_string(),
    strategy: Strategy::VersionsByName,
    source: SourceKind::PackageJson,
    is_catalog_definition: false,
  }
}

fn descriptor(name: &str, is_local_dependency: bool) -> InstanceDescriptor {
  InstanceDescriptor {
    dependency_type: Rc::new(make_dep_type()),
    internal_name: name.to_string(),
    is_local_dependency,
    name: name.to_string(),
    source_idx: SourceIdx(0),
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
  let s = selector(vec!["$LOCAL", "react"]);
  assert!(s.can_add(&descriptor("bar", true), PKG_NAME));
  assert!(s.can_add(&descriptor("react", false), PKG_NAME));
  assert!(!s.can_add(&descriptor("webpack", false), PKG_NAME));
}

#[test]
fn not_local_excludes_even_if_named_pattern_matches() {
  let s = selector(vec!["!$LOCAL", "react"]);
  assert!(!s.can_add(&descriptor("react", true), PKG_NAME));
  assert!(s.can_add(&descriptor("react", false), PKG_NAME));
  assert!(!s.can_add(&descriptor("webpack", false), PKG_NAME));
}

#[test]
fn empty_includes_with_not_local_includes_non_local_excludes_local() {
  let s = selector(vec!["!$LOCAL"]);
  assert!(s.can_add(&descriptor("anything", false), PKG_NAME));
  assert!(!s.can_add(&descriptor("local-pkg", true), PKG_NAME));
}

#[test]
fn dollar_local_literal_not_matched_as_pattern() {
  let s = selector(vec!["$LOCAL"]);
  assert!(!s.can_add(&descriptor("$LOCAL", false), PKG_NAME));
  assert!(s.can_add(&descriptor("some-local-pkg", true), PKG_NAME));
}
