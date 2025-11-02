use {
  crate::{
    semver_range::SemverRange,
    specifier::{workspace_specifier::WorkspaceSpecifier, Specifier},
  },
  std::rc::Rc,
};

#[test]
fn range_only_needs_resolution() {
  let ws = WorkspaceSpecifier::RangeOnly(SemverRange::Minor);
  assert!(ws.needs_resolution());
}

#[test]
fn resolved_does_not_need_resolution() {
  let spec = Specifier::new("1.2.3");
  let ws = WorkspaceSpecifier::Resolved(spec);
  assert!(!ws.needs_resolution());
}

#[test]
fn resolve_major_range_with_version() {
  let ws = WorkspaceSpecifier::RangeOnly(SemverRange::Minor);
  let resolved = ws.resolve_with("1.2.3").unwrap();

  // Should create "^1.2.3" which is a Range variant
  assert!(matches!(*resolved, Specifier::Range(_)));
  assert_eq!(resolved.get_semver_number(), Some("1.2.3"));
}

#[test]
fn resolve_minor_range_with_version() {
  let ws = WorkspaceSpecifier::RangeOnly(SemverRange::Patch);
  let resolved = ws.resolve_with("1.2.3").unwrap();

  // Should create "~1.2.3" which is a Range variant
  assert!(matches!(*resolved, Specifier::Range(_)));
  assert_eq!(resolved.get_semver_number(), Some("1.2.3"));
}

#[test]
fn resolve_returns_none_for_already_resolved() {
  let spec = Specifier::new("1.2.3");
  let ws = WorkspaceSpecifier::Resolved(spec);
  assert_eq!(ws.resolve_with("1.2.3"), None);
}

#[test]
fn as_resolved_returns_some_for_resolved() {
  let spec = Specifier::new("1.2.3");
  let ws = WorkspaceSpecifier::Resolved(Rc::clone(&spec));
  assert!(ws.as_resolved().is_some());
}

#[test]
fn as_resolved_returns_none_for_range_only() {
  let ws = WorkspaceSpecifier::RangeOnly(SemverRange::Minor);
  assert!(ws.as_resolved().is_none());
}
