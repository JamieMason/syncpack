use crate::{
  semver_range::SemverRange,
  specifier::{workspace_specifier::WorkspaceSpecifier, Specifier, WorkspaceProtocol},
};

#[test]
fn workspace_caret_creates_range_only() {
  let raw = "workspace:^".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  assert!(matches!(wp.inner_specifier, WorkspaceSpecifier::RangeOnly(SemverRange::Minor)));
}

#[test]
fn workspace_tilde_creates_range_only() {
  let raw = "workspace:~".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  assert!(matches!(wp.inner_specifier, WorkspaceSpecifier::RangeOnly(SemverRange::Patch)));
}

#[test]
fn workspace_star_creates_range_only() {
  let raw = "workspace:*".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  assert!(matches!(wp.inner_specifier, WorkspaceSpecifier::RangeOnly(SemverRange::Any)));
}

#[test]
fn workspace_version_creates_resolved() {
  let raw = "workspace:1.2.3".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  assert!(matches!(wp.inner_specifier, WorkspaceSpecifier::Resolved(_)));
}

#[test]
fn workspace_with_range_only_needs_resolution() {
  let raw = "workspace:^".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  assert!(wp.needs_resolution());
}

#[test]
fn workspace_with_star_needs_resolution() {
  let raw = "workspace:*".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  assert!(wp.needs_resolution());
}

#[test]
fn resolve_caret_with_local_version() {
  let raw = "workspace:^".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  let resolved = wp.resolve_with("1.2.3").unwrap();

  // Should create "^1.2.3" which is a Range variant
  assert!(matches!(*resolved, Specifier::Range(_)));
  assert_eq!(resolved.get_semver_number(), Some("1.2.3"));
}

#[test]
fn resolve_tilde_with_local_version() {
  let raw = "workspace:~".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  let resolved = wp.resolve_with("1.2.3").unwrap();

  // Should create "~1.2.3" which is a Range variant
  assert!(matches!(*resolved, Specifier::Range(_)));
  assert_eq!(resolved.get_semver_number(), Some("1.2.3"));
}

#[test]
fn resolve_star_with_local_version() {
  let raw = "workspace:*".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  let resolved = wp.resolve_with("1.2.3").unwrap();

  // Should create "1.2.3" (exact version)
  assert!(matches!(*resolved, Specifier::Exact(_)));
  assert_eq!(resolved.get_semver_number(), Some("1.2.3"));
}

#[test]
fn resolve_returns_none_for_already_resolved() {
  let raw = "workspace:1.2.3".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  assert_eq!(wp.resolve_with("1.2.3"), None);
}

#[test]
fn as_resolved_returns_some_for_complete_specifier() {
  let raw = "workspace:1.2.3".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  assert!(wp.as_resolved().is_some());
}

#[test]
fn as_resolved_returns_none_for_star() {
  let raw = "workspace:*".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  assert!(wp.as_resolved().is_none());
}

#[test]
fn as_resolved_returns_none_for_range_only() {
  let raw = "workspace:^".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  assert!(wp.as_resolved().is_none());
}

#[test]
fn version_str_contains_caret_for_workspace_caret() {
  let raw = "workspace:^".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  assert_eq!(wp.version_str, "^");
}

#[test]
fn version_str_contains_tilde_for_workspace_tilde() {
  let raw = "workspace:~".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  assert_eq!(wp.version_str, "~");
}

#[test]
fn version_str_contains_star_for_workspace_star() {
  let raw = "workspace:*".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  assert_eq!(wp.version_str, "*");
}

#[test]
fn version_str_contains_version_for_workspace_version() {
  let raw = "workspace:1.2.3".to_string();
  let wp = WorkspaceProtocol::new(raw).unwrap();
  assert_eq!(wp.version_str, "1.2.3");
}

#[test]
fn raw_is_preserved() {
  let raw = "workspace:^1.2.3".to_string();
  let wp = WorkspaceProtocol::new(raw.clone()).unwrap();
  assert_eq!(wp.raw, raw);
}
