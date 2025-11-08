use {crate::rcfile::Rcfile, serde_json::json};

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
  let rcfile: Rcfile = serde_json::from_value(config_json).unwrap();
  assert!(rcfile.unknown_fields.contains_key("dependencyTypes"));
}

#[test]
fn detects_v13_specifier_types_in_config() {
  let config_json = json!({
    "specifierTypes": ["exact", "range"],
    "versionGroups": []
  });
  let rcfile: Rcfile = serde_json::from_value(config_json).unwrap();
  assert!(rcfile.unknown_fields.contains_key("specifierTypes"));
}

#[test]
fn detects_v13_lint_formatting_in_config() {
  let config_json = json!({
    "lintFormatting": true,
    "versionGroups": []
  });
  let rcfile: Rcfile = serde_json::from_value(config_json).unwrap();
  assert!(rcfile.unknown_fields.contains_key("lintFormatting"));
}

#[test]
fn detects_v13_lint_semver_ranges_in_config() {
  let config_json = json!({
    "lintSemverRanges": true,
    "versionGroups": []
  });
  let rcfile: Rcfile = serde_json::from_value(config_json).unwrap();
  assert!(rcfile.unknown_fields.contains_key("lintSemverRanges"));
}

#[test]
fn detects_v13_lint_versions_in_config() {
  let config_json = json!({
    "lintVersions": true,
    "versionGroups": []
  });
  let rcfile: Rcfile = serde_json::from_value(config_json).unwrap();
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
  let rcfile: Rcfile = serde_json::from_value(config_json).unwrap();
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
  let rcfile: Rcfile = serde_json::from_value(config_json).unwrap();
  assert_eq!(rcfile.unknown_fields.len(), 0);
}
