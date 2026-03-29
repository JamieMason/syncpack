use {
  crate::{
    context::ConfigError,
    rcfile::{semver_group::AnySemverGroup, RawRcfile, Rcfile},
    version_group::{AnyVersionGroup, VersionGroup},
  },
  serde_json::json,
};

#[test]
fn default_format_bugs_is_false() {
  let rcfile = Rcfile::default();
  assert!(!rcfile.format_bugs);
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
  assert!(errors
    .iter()
    .any(|e| matches!(e, ConfigError::DeprecatedProperty { property, .. } if property == "dependencyTypes")));
  assert!(errors
    .iter()
    .any(|e| matches!(e, ConfigError::DeprecatedProperty { property, .. } if property == "lintFormatting")));
}

#[test]
fn validate_unknown_fields_returns_unrecognised_errors() {
  let raw: RawRcfile = serde_json::from_value(json!({
    "notARealProperty": true,
  }))
  .unwrap();
  let errors = raw.validate_unknown_fields().unwrap_err();
  assert_eq!(errors.len(), 1);
  assert!(matches!(&errors[0], ConfigError::UnrecognisedProperty { path } if path == "notARealProperty"));
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
  assert!(errors
    .iter()
    .any(|e| matches!(e, ConfigError::UnrecognisedProperty { path } if path == "versionGroups[0].notReal")));
  assert!(errors
    .iter()
    .any(|e| matches!(e, ConfigError::UnrecognisedProperty { path } if path == "semverGroups[0].bogus")));
}

#[test]
fn validate_unknown_fields_ok_when_valid() {
  let raw: RawRcfile = serde_json::from_value(json!({})).unwrap();
  assert!(raw.validate_unknown_fields().is_ok());
}

#[test]
fn try_from_rejects_invalid_dependency_type() {
  let raw: RawRcfile = serde_json::from_value(json!({
    "versionGroups": [{
      "label": "test",
      "dependencyTypes": ["nonexistent"]
    }]
  }))
  .unwrap();
  let err = Rcfile::try_from(raw).unwrap_err();
  assert!(matches!(err, ConfigError::InvalidDependencyType { name } if name == "nonexistent"));
}

#[test]
fn semver_group_from_config_rejects_missing_required_fields() {
  let group: AnySemverGroup = serde_json::from_value(json!({
    "label": "bad group"
  }))
  .unwrap();
  let err = crate::rcfile::semver_group::SemverGroup::from_config(group).unwrap_err();
  assert!(matches!(err, ConfigError::InvalidSemverGroup));
}

#[test]
fn version_group_from_config_rejects_invalid_policy() {
  let group: AnyVersionGroup = serde_json::from_value(json!({
    "label": "bad",
    "policy": "notAPolicy"
  }))
  .unwrap();
  let packages = crate::packages::Packages::new();
  let err = VersionGroup::from_config(group, &packages).unwrap_err();
  assert!(matches!(err, ConfigError::InvalidVersionGroupPolicy(p) if p == "notAPolicy"));
}

mod comment_properties {
  use {
    crate::{context::ConfigError, rcfile::RawRcfile},
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
    assert!(matches!(&errors[0], ConfigError::UnrecognisedProperty { path } if path == "notARealProperty"));
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
    assert!(matches!(&errors[0], ConfigError::UnrecognisedProperty { path } if path == "customTypes.myType.bogus"));
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
    assert!(matches!(&errors[0], ConfigError::UnrecognisedProperty { path } if path == "dependencyGroups[0].bogus"));
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
    assert!(matches!(&errors[0], ConfigError::UnrecognisedProperty { path } if path == "semverGroups[0].bogus"));
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
    assert!(matches!(&errors[0], ConfigError::UnrecognisedProperty { path } if path == "versionGroups[0].notReal"));
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
