use {
  crate::{
    errors::UnsupportedConfigError,
    rcfile::{
      RawRcfile, Rcfile,
      semver_group::AnySemverGroup,
      update_group::{AnyUpdateGroup, UpdateGroup, UpdatePolicy},
    },
    version_group::{AnyVersionGroup, VersionGroup},
  },
  serde_json::json,
  syncpack_specifier::update_target::UpdateTarget,
};

#[test]
fn default_format_bugs_is_false() {
  let rcfile = Rcfile::default();
  assert!(!rcfile.format_bugs);
}

#[test]
fn raw_minimum_release_age_is_none_when_omitted() {
  let raw: RawRcfile = serde_json::from_value(json!({})).unwrap();
  assert_eq!(raw.minimum_release_age, None);
}

#[test]
fn raw_minimum_release_age_captures_user_value() {
  let raw: RawRcfile = serde_json::from_value(json!({ "minimumReleaseAge": 60 })).unwrap();
  assert_eq!(raw.minimum_release_age, Some(60));
}

#[test]
fn try_from_falls_back_to_default_when_user_did_not_set() {
  let raw: RawRcfile = serde_json::from_value(json!({})).unwrap();
  let rcfile = Rcfile::try_from(raw).unwrap();
  assert_eq!(rcfile.minimum_release_age, 1440);
}

#[test]
fn try_from_uses_user_value_when_set() {
  let raw: RawRcfile = serde_json::from_value(json!({ "minimumReleaseAge": 60 })).unwrap();
  let rcfile = Rcfile::try_from(raw).unwrap();
  assert_eq!(rcfile.minimum_release_age, 60);
}

#[test]
fn try_from_preserves_zero_when_user_disables_filter() {
  let raw: RawRcfile = serde_json::from_value(json!({ "minimumReleaseAge": 0 })).unwrap();
  let rcfile = Rcfile::try_from(raw).unwrap();
  assert_eq!(rcfile.minimum_release_age, 0);
}

#[test]
fn default_format_repository_is_false() {
  let rcfile = Rcfile::default();
  assert!(!rcfile.format_repository);
}

#[test]
fn detects_v13_dependency_types_in_config() {
  let config_json = json!({
    "dependencyTypes": ["prod", "dev"],
    "versionGroups": []
  });
  let rcfile: RawRcfile = serde_json::from_value(config_json).unwrap();
  assert!(rcfile.unknown_fields.contains_key("dependencyTypes"));
}

#[test]
fn detects_v13_specifier_types_in_config() {
  let config_json = json!({
    "specifierTypes": ["exact", "range"],
    "versionGroups": []
  });
  let rcfile: RawRcfile = serde_json::from_value(config_json).unwrap();
  assert!(rcfile.unknown_fields.contains_key("specifierTypes"));
}

#[test]
fn detects_v13_lint_formatting_in_config() {
  let config_json = json!({
    "lintFormatting": true,
    "versionGroups": []
  });
  let rcfile: RawRcfile = serde_json::from_value(config_json).unwrap();
  assert!(rcfile.unknown_fields.contains_key("lintFormatting"));
}

#[test]
fn detects_v13_lint_semver_ranges_in_config() {
  let config_json = json!({
    "lintSemverRanges": true,
    "versionGroups": []
  });
  let rcfile: RawRcfile = serde_json::from_value(config_json).unwrap();
  assert!(rcfile.unknown_fields.contains_key("lintSemverRanges"));
}

#[test]
fn detects_v13_lint_versions_in_config() {
  let config_json = json!({
    "lintVersions": true,
    "versionGroups": []
  });
  let rcfile: RawRcfile = serde_json::from_value(config_json).unwrap();
  assert!(rcfile.unknown_fields.contains_key("lintVersions"));
}

#[test]
fn detects_multiple_v13_properties_in_config() {
  let config_json = json!({
    "dependencyTypes": ["prod", "dev"],
    "specifierTypes": ["exact"],
    "lintFormatting": true,
    "lintSemverRanges": false,
    "lintVersions": true,
    "versionGroups": []
  });
  let rcfile: RawRcfile = serde_json::from_value(config_json).unwrap();
  assert_eq!(rcfile.unknown_fields.len(), 5);
  assert!(rcfile.unknown_fields.contains_key("dependencyTypes"));
  assert!(rcfile.unknown_fields.contains_key("specifierTypes"));
  assert!(rcfile.unknown_fields.contains_key("lintFormatting"));
  assert!(rcfile.unknown_fields.contains_key("lintSemverRanges"));
  assert!(rcfile.unknown_fields.contains_key("lintVersions"));
}

#[test]
fn valid_v14_config_has_no_unknown_fields() {
  let config_json = json!({
    "versionGroups": [],
    "semverGroups": [],
    "indent": "  ",
    "source": ["packages/*/package.json"]
  });
  let rcfile: RawRcfile = serde_json::from_value(config_json).unwrap();
  assert_eq!(rcfile.unknown_fields.len(), 0);
}

#[test]
fn validate_unknown_fields_returns_deprecated_errors() {
  let raw: RawRcfile = serde_json::from_value(json!({
    "dependencyTypes": ["prod"],
    "lintFormatting": true,
  }))
  .unwrap();
  let errors = raw.validate_unknown_fields().unwrap_err();
  assert_eq!(errors.len(), 2);
  assert!(
    errors
      .iter()
      .any(|e| matches!(e, UnsupportedConfigError::DeprecatedProperty { property, .. } if property == "dependencyTypes"))
  );
  assert!(
    errors
      .iter()
      .any(|e| matches!(e, UnsupportedConfigError::DeprecatedProperty { property, .. } if property == "lintFormatting"))
  );
}

#[test]
fn validate_unknown_fields_returns_unrecognised_errors() {
  let raw: RawRcfile = serde_json::from_value(json!({
    "notARealProperty": true,
  }))
  .unwrap();
  let errors = raw.validate_unknown_fields().unwrap_err();
  assert_eq!(errors.len(), 1);
  assert!(matches!(&errors[0], UnsupportedConfigError::UnrecognisedProperty { path } if path == "notARealProperty"));
}

#[test]
fn validate_unknown_fields_returns_nested_unrecognised_errors() {
  let raw: RawRcfile = serde_json::from_value(json!({
    "versionGroups": [{ "label": "test", "notReal": true }],
    "semverGroups": [{ "range": "^", "bogus": 1 }],
  }))
  .unwrap();
  let errors = raw.validate_unknown_fields().unwrap_err();
  assert_eq!(errors.len(), 2);
  assert!(
    errors
      .iter()
      .any(|e| matches!(e, UnsupportedConfigError::UnrecognisedProperty { path } if path == "versionGroups[0].notReal"))
  );
  assert!(
    errors
      .iter()
      .any(|e| matches!(e, UnsupportedConfigError::UnrecognisedProperty { path } if path == "semverGroups[0].bogus"))
  );
}

#[test]
fn validate_unknown_fields_ok_when_valid() {
  let raw: RawRcfile = serde_json::from_value(json!({})).unwrap();
  assert!(raw.validate_unknown_fields().is_ok());
}

#[test]
fn try_from_no_longer_rejects_invalid_dependency_type() {
  // Dep-type validation lives in `Context::create`, not `TryFrom<RawRcfile>`,
  // so user-referenced catalog dep types resolve after discovery. A `RawRcfile`
  // referencing an unknown dep type parses cleanly here.
  let raw: RawRcfile = serde_json::from_value(json!({
    "versionGroups": [{
      "label": "test",
      "dependencyTypes": ["nonexistent"]
    }]
  }))
  .unwrap();
  assert!(Rcfile::try_from(raw).is_ok());
}

#[test]
fn context_create_rejects_invalid_dependency_type() {
  use crate::{
    context::{Context, ContextError},
    rcfile::from_disk::RcfileError,
    test::mock,
  };
  let config = mock::config_from_mock(json!({
    "versionGroups": [{
      "label": "test",
      "dependencyTypes": ["nonexistent"]
    }]
  }));
  let (disk, sources) = mock::disk_and_sources_from_mocks(vec![json!({"name": "pkg-a", "version": "0.0.0"})]);
  let err = Context::create(config, disk, sources, vec![]).unwrap_err();
  let ContextError::RcfileError(RcfileError::UnsupportedConfig(errs)) = err else {
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
fn custom_type_with_explicit_source_parses() {
  use crate::{rcfile::compute_all_dependency_types, source::SourceKind};
  let raw: RawRcfile = serde_json::from_value(json!({
    "customTypes": {
      "x": { "strategy": "versionsByName", "path": "/x", "source": "PnpmWorkspace" }
    }
  }))
  .unwrap();
  let dep_types = compute_all_dependency_types(&raw.custom_types).expect("compute should succeed");
  let dt = dep_types
    .iter()
    .find(|dt| dt.name == "x")
    .expect("custom type x should produce a dep type");
  assert_eq!(dt.source, SourceKind::PnpmWorkspace);
  assert!(!dt.is_catalog_definition);
}

#[test]
fn custom_type_with_invalid_source_returns_error() {
  use crate::rcfile::compute_all_dependency_types;
  let raw: RawRcfile = serde_json::from_value(json!({
    "customTypes": {
      "x": { "strategy": "versionsByName", "path": "/x", "source": "BunYaml" }
    }
  }))
  .unwrap();
  let err = compute_all_dependency_types(&raw.custom_types).unwrap_err();
  match err {
    UnsupportedConfigError::InvalidSource { value } => assert_eq!(value, "BunYaml"),
    other => panic!("expected InvalidSource, got {other:?}"),
  }
}

#[test]
fn custom_type_without_source_field_defaults_package_json() {
  use crate::{rcfile::compute_all_dependency_types, source::SourceKind};
  let raw: RawRcfile = serde_json::from_value(json!({
    "customTypes": {
      "x": { "strategy": "versionsByName", "path": "/x" }
    }
  }))
  .unwrap();
  let dep_types = compute_all_dependency_types(&raw.custom_types).expect("compute should succeed");
  let dt = dep_types
    .iter()
    .find(|dt| dt.name == "x")
    .expect("custom type x should produce a dep type");
  assert_eq!(dt.source, SourceKind::PackageJson);
}

#[test]
fn rcfile_catalog_dep_type_names_uses_flag() {
  // Even a customType named "pnpmCatalogLike" (matching v2's prefix filter)
  // must NOT be treated as a catalog dep type — it isn't auto-generated and
  // its `is_catalog_definition` flag is false.
  use crate::dependency::{DependencyType, Strategy};
  let mut rcfile = Rcfile::default();
  // Add a regular user dep type whose name happens to start with "pnpmCatalog".
  rcfile.all_dependency_types.push(DependencyType {
    name_path: None,
    name: "pnpmCatalogLike".to_string(),
    path: "/x".to_string(),
    strategy: Strategy::VersionsByName,
    source: crate::source::SourceKind::PackageJson,
    is_catalog_definition: false,
  });
  // Add an actual auto-gen catalog dep type.
  rcfile.all_dependency_types.push(DependencyType {
    name_path: None,
    name: "pnpmCatalog".to_string(),
    path: "/catalog".to_string(),
    strategy: Strategy::VersionsByName,
    source: crate::source::SourceKind::PnpmWorkspace,
    is_catalog_definition: true,
  });
  let names = rcfile.catalog_dep_type_names_for_test();
  assert_eq!(names, vec!["pnpmCatalog".to_string()]);
}

#[test]
fn semver_group_from_config_rejects_missing_required_fields() {
  let group: AnySemverGroup = serde_json::from_value(json!({
    "label": "bad group"
  }))
  .unwrap();
  let err = crate::rcfile::semver_group::SemverGroup::from_config(group).unwrap_err();
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
fn update_groups_unknown_field_raises_unrecognised_property() {
  let raw: RawRcfile = serde_json::from_value(json!({
    "updateGroups": [{ "target": "patch", "bogus": 1 }]
  }))
  .unwrap();
  let errors = raw.validate_unknown_fields().unwrap_err();
  assert_eq!(errors.len(), 1);
  assert!(matches!(&errors[0], UnsupportedConfigError::UnrecognisedProperty { path } if path == "updateGroups[0].bogus"));
}

#[test]
fn version_group_from_config_rejects_invalid_policy() {
  let group: AnyVersionGroup = serde_json::from_value(json!({
    "label": "bad",
    "policy": "notAPolicy"
  }))
  .unwrap();
  let sources = crate::sources::Sources::new();
  let err = VersionGroup::from_config(group, &sources).unwrap_err();
  assert!(matches!(err, UnsupportedConfigError::InvalidVersionGroupPolicy(p) if p == "notAPolicy"));
}

mod source_mode {
  use {
    crate::rcfile::{RawRcfile, Rcfile, SourceMode},
    serde_json::json,
  };

  #[test]
  fn raw_defaults_to_replace_when_omitted() {
    let raw: RawRcfile = serde_json::from_value(json!({})).unwrap();
    assert_eq!(raw.source_mode, SourceMode::Replace);
  }

  #[test]
  fn raw_parses_extend() {
    let raw: RawRcfile = serde_json::from_value(json!({ "sourceMode": "extend" })).unwrap();
    assert_eq!(raw.source_mode, SourceMode::Extend);
  }

  #[test]
  fn raw_parses_replace() {
    let raw: RawRcfile = serde_json::from_value(json!({ "sourceMode": "replace" })).unwrap();
    assert_eq!(raw.source_mode, SourceMode::Replace);
  }

  #[test]
  fn raw_rejects_invalid_value() {
    let result: Result<RawRcfile, _> = serde_json::from_value(json!({ "sourceMode": "merge" }));
    assert!(result.is_err(), "unknown sourceMode value must not deserialize");
  }

  #[test]
  fn not_flagged_as_unknown_field() {
    let raw: RawRcfile = serde_json::from_value(json!({ "sourceMode": "extend" })).unwrap();
    assert!(!raw.unknown_fields.contains_key("sourceMode"));
    assert!(raw.validate_unknown_fields().is_ok());
  }

  #[test]
  fn try_from_propagates_value() {
    let raw: RawRcfile = serde_json::from_value(json!({ "sourceMode": "extend" })).unwrap();
    let rcfile = Rcfile::try_from(raw).unwrap();
    assert_eq!(rcfile.source_mode, SourceMode::Extend);
  }

  #[test]
  fn try_from_default_is_replace() {
    let raw: RawRcfile = serde_json::from_value(json!({})).unwrap();
    let rcfile = Rcfile::try_from(raw).unwrap();
    assert_eq!(rcfile.source_mode, SourceMode::Replace);
  }
}

mod comment_properties {
  use {
    crate::{errors::UnsupportedConfigError, rcfile::RawRcfile},
    serde_json::json,
  };

  #[test]
  fn ignored_at_root() {
    let raw: RawRcfile = serde_json::from_value(json!({
      "// why we pin react": "see issue #123",
      "//": "this is a comment",
      "// another note": true,
    }))
    .unwrap();
    assert!(raw.validate_unknown_fields().is_ok());
  }

  #[test]
  fn coexist_with_real_config() {
    let raw: RawRcfile = serde_json::from_value(json!({
      "// note": "explanation",
      "indent": "  ",
      "source": ["packages/*/package.json"],
    }))
    .unwrap();
    assert!(raw.validate_unknown_fields().is_ok());
    assert_eq!(raw.indent, Some("  ".to_string()));
  }

  #[test]
  fn do_not_mask_errors_at_root() {
    let raw: RawRcfile = serde_json::from_value(json!({
      "// note": "this is fine",
      "notARealProperty": true,
    }))
    .unwrap();
    let errors = raw.validate_unknown_fields().unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(matches!(&errors[0], UnsupportedConfigError::UnrecognisedProperty { path } if path == "notARealProperty"));
  }

  #[test]
  fn ignored_in_custom_types() {
    let raw: RawRcfile = serde_json::from_value(json!({
      "customTypes": {
        "myType": {
          "strategy": "versionsByName",
          "path": "myDeps",
          "// note": "this explains the custom type"
        }
      }
    }))
    .unwrap();
    assert!(raw.validate_unknown_fields().is_ok());
  }

  #[test]
  fn do_not_mask_errors_in_custom_types() {
    let raw: RawRcfile = serde_json::from_value(json!({
      "customTypes": {
        "myType": {
          "strategy": "versionsByName",
          "path": "myDeps",
          "// note": "fine",
          "bogus": true
        }
      }
    }))
    .unwrap();
    let errors = raw.validate_unknown_fields().unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(matches!(&errors[0], UnsupportedConfigError::UnrecognisedProperty { path } if path == "customTypes.myType.bogus"));
  }

  #[test]
  fn ignored_in_dependency_groups() {
    let raw: RawRcfile = serde_json::from_value(json!({
      "dependencyGroups": [{
        "aliasName": "group1",
        "// reason": "explain grouping"
      }]
    }))
    .unwrap();
    assert!(raw.validate_unknown_fields().is_ok());
  }

  #[test]
  fn do_not_mask_errors_in_dependency_groups() {
    let raw: RawRcfile = serde_json::from_value(json!({
      "dependencyGroups": [{
        "aliasName": "group1",
        "// reason": "fine",
        "bogus": true
      }]
    }))
    .unwrap();
    let errors = raw.validate_unknown_fields().unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(matches!(&errors[0], UnsupportedConfigError::UnrecognisedProperty { path } if path == "dependencyGroups[0].bogus"));
  }

  #[test]
  fn ignored_in_semver_groups() {
    let raw: RawRcfile = serde_json::from_value(json!({
      "semverGroups": [{
        "range": "^",
        "// why": "pin everything"
      }]
    }))
    .unwrap();
    assert!(raw.validate_unknown_fields().is_ok());
  }

  #[test]
  fn do_not_mask_errors_in_semver_groups() {
    let raw: RawRcfile = serde_json::from_value(json!({
      "semverGroups": [{
        "range": "^",
        "// why": "fine",
        "bogus": 1
      }]
    }))
    .unwrap();
    let errors = raw.validate_unknown_fields().unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(matches!(&errors[0], UnsupportedConfigError::UnrecognisedProperty { path } if path == "semverGroups[0].bogus"));
  }

  #[test]
  fn ignored_in_version_groups() {
    let raw: RawRcfile = serde_json::from_value(json!({
      "versionGroups": [{
        "label": "test",
        "// note": "explanation"
      }]
    }))
    .unwrap();
    assert!(raw.validate_unknown_fields().is_ok());
  }

  #[test]
  fn do_not_mask_errors_in_version_groups() {
    let raw: RawRcfile = serde_json::from_value(json!({
      "versionGroups": [{
        "label": "test",
        "// note": "fine",
        "notReal": true
      }]
    }))
    .unwrap();
    let errors = raw.validate_unknown_fields().unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(matches!(&errors[0], UnsupportedConfigError::UnrecognisedProperty { path } if path == "versionGroups[0].notReal"));
  }

  #[test]
  fn ignored_across_all_nested_locations() {
    let raw: RawRcfile = serde_json::from_value(json!({
      "// root comment": "ok",
      "customTypes": {
        "myType": {
          "strategy": "versionsByName",
          "path": "myDeps",
          "// ct comment": "ok"
        }
      },
      "dependencyGroups": [{ "aliasName": "g", "// dg comment": "ok" }],
      "semverGroups": [{ "range": "^", "// sg comment": "ok" }],
      "versionGroups": [{ "label": "v", "// vg comment": "ok" }],
    }))
    .unwrap();
    assert!(raw.validate_unknown_fields().is_ok());
  }
}
