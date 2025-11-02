use crate::specifier::Specifier;

#[test]
fn resolves_workspace_caret_with_local_version() {
  let spec = Specifier::new("workspace:^");
  let local_version = Specifier::new("1.2.3");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  assert_eq!(resolved.get_config_identifier(), "range");
  assert_eq!(resolved.get_semver_number(), Some("1.2.3"));
}

#[test]
fn resolves_workspace_tilde_with_local_version() {
  let spec = Specifier::new("workspace:~");
  let local_version = Specifier::new("1.2.3");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  assert_eq!(resolved.get_config_identifier(), "range");
  assert_eq!(resolved.get_semver_number(), Some("1.2.3"));
}

#[test]
fn resolves_workspace_asterisk_with_local_version() {
  let spec = Specifier::new("workspace:*");
  let local_version = Specifier::new("1.2.3");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  assert_eq!(resolved.get_config_identifier(), "latest");
}

#[test]
fn resolves_workspace_gte_with_local_version() {
  let spec = Specifier::new("workspace:>=");
  let local_version = Specifier::new("1.2.3");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  assert_eq!(resolved.get_config_identifier(), "unsupported");
  assert_eq!(resolved.get_semver_number(), None);
}

#[test]
fn resolves_workspace_gt_with_local_version() {
  let spec = Specifier::new("workspace:>");
  let local_version = Specifier::new("1.2.3");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  assert_eq!(resolved.get_config_identifier(), "unsupported");
  assert_eq!(resolved.get_semver_number(), None);
}

#[test]
fn resolves_workspace_lte_with_local_version() {
  let spec = Specifier::new("workspace:<=");
  let local_version = Specifier::new("1.2.3");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  assert_eq!(resolved.get_config_identifier(), "unsupported");
  assert_eq!(resolved.get_semver_number(), None);
}

#[test]
fn resolves_workspace_lt_with_local_version() {
  let spec = Specifier::new("workspace:<");
  let local_version = Specifier::new("1.2.3");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  assert_eq!(resolved.get_config_identifier(), "unsupported");
  assert_eq!(resolved.get_semver_number(), None);
}

#[test]
fn resolves_workspace_with_embedded_exact_version() {
  let spec = Specifier::new("workspace:1.2.3");
  let local_version = Specifier::new("2.0.0");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  // Embedded version takes precedence, local_version is ignored
  assert_eq!(resolved.get_config_identifier(), "exact");
  assert_eq!(resolved.get_semver_number(), Some("1.2.3"));
}

#[test]
fn resolves_workspace_with_embedded_caret_version() {
  let spec = Specifier::new("workspace:^1.2.3");
  let local_version = Specifier::new("2.0.0");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  // Embedded version takes precedence, local_version is ignored
  assert_eq!(resolved.get_config_identifier(), "range");
  assert_eq!(resolved.get_semver_number(), Some("1.2.3"));
}

#[test]
fn resolves_workspace_with_embedded_tilde_version() {
  let spec = Specifier::new("workspace:~1.2.3");
  let local_version = Specifier::new("2.0.0");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  // Embedded version takes precedence, local_version is ignored
  assert_eq!(resolved.get_config_identifier(), "range");
  assert_eq!(resolved.get_semver_number(), Some("1.2.3"));
}

#[test]
fn resolves_workspace_with_embedded_major_version() {
  let spec = Specifier::new("workspace:^1");
  let local_version = Specifier::new("2.0.0");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  // Embedded version takes precedence, local_version is ignored
  assert_eq!(resolved.get_config_identifier(), "range-major");
  assert_eq!(resolved.get_semver_number(), Some("1"));
}

#[test]
fn resolves_workspace_with_embedded_minor_version() {
  let spec = Specifier::new("workspace:~1.2");
  let local_version = Specifier::new("2.0.0");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  // Embedded version takes precedence, local_version is ignored
  assert_eq!(resolved.get_config_identifier(), "range-minor");
  assert_eq!(resolved.get_semver_number(), Some("1.2"));
}

#[test]
fn returns_self_when_not_workspace_protocol_exact() {
  let spec = Specifier::new("1.2.3");
  let local_version = Specifier::new("2.0.0");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  // Should return the same cached instance
  assert_eq!(resolved.get_config_identifier(), "exact");
  assert_eq!(resolved.get_semver_number(), Some("1.2.3"));
  assert!(std::rc::Rc::ptr_eq(&spec, &resolved));
}

#[test]
fn returns_self_when_not_workspace_protocol_range() {
  let spec = Specifier::new("^1.2.3");
  let local_version = Specifier::new("2.0.0");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  // Should return the same cached instance
  assert_eq!(resolved.get_config_identifier(), "range");
  assert_eq!(resolved.get_semver_number(), Some("1.2.3"));
  assert!(std::rc::Rc::ptr_eq(&spec, &resolved));
}

#[test]
fn returns_self_when_not_workspace_protocol_alias() {
  let spec = Specifier::new("npm:lodash@^4.17.21");
  let local_version = Specifier::new("2.0.0");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  // Should return the same cached instance
  assert_eq!(resolved.get_config_identifier(), "alias");
  assert_eq!(resolved.get_alias_name(), Some("lodash"));
  assert!(std::rc::Rc::ptr_eq(&spec, &resolved));
}

#[test]
fn returns_self_when_not_workspace_protocol_git() {
  let spec = Specifier::new("github:user/repo#v1.2.3");
  let local_version = Specifier::new("2.0.0");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  // Should return the same cached instance
  assert_eq!(resolved.get_config_identifier(), "git");
  assert!(std::rc::Rc::ptr_eq(&spec, &resolved));
}

#[test]
fn returns_self_when_not_workspace_protocol_tag() {
  let spec = Specifier::new("alpha");
  let local_version = Specifier::new("2.0.0");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  // Should return the same cached instance
  assert_eq!(resolved.get_config_identifier(), "tag");
  assert!(std::rc::Rc::ptr_eq(&spec, &resolved));
}

#[test]
fn returns_self_when_not_workspace_protocol_latest() {
  let spec = Specifier::new("*");
  let local_version = Specifier::new("2.0.0");
  let resolved = spec.resolve_workspace_protocol(&local_version).unwrap();

  // Should return the same cached instance
  assert_eq!(resolved.get_config_identifier(), "latest");
  assert!(std::rc::Rc::ptr_eq(&spec, &resolved));
}

#[test]
fn caching_works_correctly_after_resolution() {
  let spec1 = Specifier::new("workspace:^");
  let local_version = Specifier::new("1.2.3");
  let resolved1 = spec1.resolve_workspace_protocol(&local_version).unwrap();

  // Resolving the same workspace protocol with the same version should return
  // the same cached result
  let spec2 = Specifier::new("workspace:^");
  let resolved2 = spec2.resolve_workspace_protocol(&local_version).unwrap();

  // spec1 and spec2 should be the same cached instance
  assert!(std::rc::Rc::ptr_eq(&spec1, &spec2));

  // resolved1 and resolved2 should be the same cached instance
  assert!(std::rc::Rc::ptr_eq(&resolved1, &resolved2));
  assert_eq!(resolved1.get_semver_number(), Some("1.2.3"));
}

#[test]
fn different_local_versions_produce_different_resolved_specifiers() {
  let spec = Specifier::new("workspace:^");

  let local_v1 = Specifier::new("1.2.3");
  let resolved1 = spec.resolve_workspace_protocol(&local_v1).unwrap();

  let local_v2 = Specifier::new("2.0.0");
  let resolved2 = spec.resolve_workspace_protocol(&local_v2).unwrap();

  // Should produce different resolved specifiers
  assert!(!std::rc::Rc::ptr_eq(&resolved1, &resolved2));
  assert_eq!(resolved1.get_semver_number(), Some("1.2.3"));
  assert_eq!(resolved2.get_semver_number(), Some("2.0.0"));
}

#[test]
fn returns_none_when_local_version_is_not_exact_range() {
  let spec = Specifier::new("workspace:^");
  let local_version = Specifier::new("^1.2.3"); // Not Exact!
  let result = spec.resolve_workspace_protocol(&local_version);
  assert!(result.is_none());
}

#[test]
fn returns_none_when_local_version_is_not_exact_tag() {
  let spec = Specifier::new("workspace:^");
  let local_version = Specifier::new("latest"); // Not Exact!
  let result = spec.resolve_workspace_protocol(&local_version);
  assert!(result.is_none());
}

#[test]
fn returns_none_when_local_version_is_not_exact_alias() {
  let spec = Specifier::new("workspace:^");
  let local_version = Specifier::new("npm:foo@1.2.3"); // Not Exact!
  let result = spec.resolve_workspace_protocol(&local_version);
  assert!(result.is_none());
}
