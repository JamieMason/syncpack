use {
  crate::discovery::{SemverGroup, UpdateGroup, version_group_from_config},
  serde_json::json,
  syncpack_config::{AnyVersionGroup, semver_group::AnySemverGroup, update_group::AnyUpdateGroup},
  syncpack_specifier::update_target::UpdateTarget,
  syncpack_vocab::{config_error::UnsupportedConfigError, update_policy::UpdatePolicy},
};

#[test]
fn context_create_rejects_invalid_dependency_type() {
  use crate::{
    discovery::{Discovered, DiscoveryError},
    rcfile_loader::RcfileError,
    test::mock,
  };
  let config = mock::config_from_mock(json!({
    "versionGroups": [{
      "label": "test",
      "dependencyTypes": ["nonexistent"]
    }]
  }));
  let (disk, sources) = mock::disk_and_sources_from_mocks(vec![json!({"name": "pkg-a", "version": "0.0.0"})]);
  let err = Discovered::create(config, disk, sources, vec![]).unwrap_err();
  let DiscoveryError::RcfileError(RcfileError::UnsupportedConfig(errs)) = err else {
    panic!("expected RcfileError::UnsupportedConfig");
  };
  assert!(
    errs
      .0
      .iter()
      .any(|e| matches!(e, UnsupportedConfigError::InvalidDependencyType { name } if name == "nonexistent"))
  );
}

#[test]
fn semver_group_from_config_rejects_missing_required_fields() {
  let group: AnySemverGroup = serde_json::from_value(json!({
    "label": "bad group"
  }))
  .unwrap();
  let err = SemverGroup::from_config(group).unwrap_err();
  assert!(matches!(err, UnsupportedConfigError::InvalidSemverGroup));
}

#[test]
fn update_group_parses_target_patch() {
  let group: AnyUpdateGroup = serde_json::from_value(json!({ "target": "patch" })).unwrap();
  let parsed = UpdateGroup::from_config(group).unwrap();
  assert!(matches!(parsed.policy, UpdatePolicy::UpTo(UpdateTarget::Patch)));
}

#[test]
fn update_group_parses_target_minor() {
  let group: AnyUpdateGroup = serde_json::from_value(json!({ "target": "minor" })).unwrap();
  let parsed = UpdateGroup::from_config(group).unwrap();
  assert!(matches!(parsed.policy, UpdatePolicy::UpTo(UpdateTarget::Minor)));
}

#[test]
fn update_group_parses_target_latest() {
  let group: AnyUpdateGroup = serde_json::from_value(json!({ "target": "latest" })).unwrap();
  let parsed = UpdateGroup::from_config(group).unwrap();
  assert!(matches!(parsed.policy, UpdatePolicy::UpTo(UpdateTarget::Latest)));
}

#[test]
fn update_group_parses_is_ignored_true() {
  let group: AnyUpdateGroup = serde_json::from_value(json!({ "isIgnored": true })).unwrap();
  let parsed = UpdateGroup::from_config(group).unwrap();
  assert!(matches!(parsed.policy, UpdatePolicy::Skip));
}

#[test]
fn update_group_rejects_neither_field() {
  let group: AnyUpdateGroup = serde_json::from_value(json!({ "label": "x" })).unwrap();
  let err = UpdateGroup::from_config(group).unwrap_err();
  assert!(matches!(err, UnsupportedConfigError::InvalidUpdateGroup));
}

#[test]
fn update_group_rejects_both_fields() {
  let group: AnyUpdateGroup = serde_json::from_value(json!({ "isIgnored": true, "target": "patch" })).unwrap();
  let err = UpdateGroup::from_config(group).unwrap_err();
  assert!(matches!(err, UnsupportedConfigError::InvalidUpdateGroup));
}

#[test]
fn update_group_rejects_unknown_target() {
  let group: AnyUpdateGroup = serde_json::from_value(json!({ "target": "nope" })).unwrap();
  let err = UpdateGroup::from_config(group).unwrap_err();
  assert!(matches!(err, UnsupportedConfigError::InvalidUpdateGroup));
}

#[test]
fn version_group_from_config_rejects_invalid_policy() {
  let group: AnyVersionGroup = serde_json::from_value(json!({
    "label": "bad",
    "policy": "notAPolicy"
  }))
  .unwrap();
  let sources = syncpack_model::sources::Sources::new();
  let err = version_group_from_config(group, 0, &sources).unwrap_err();
  assert!(matches!(err, UnsupportedConfigError::InvalidVersionGroupPolicy(p) if p == "notAPolicy"));
}
