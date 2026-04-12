use {
  crate::{
    dependency::{DependencyType, Strategy},
    disk::{detect_formatting, get_pretty_json_bytes, DetectedFormatting, File},
    instance::{FixableInstance, Instance},
    package_json::PackageJson,
    packages::PackageIdx,
    specifier::Specifier,
  },
  serde_json::{json, Map, Value},
  std::path::PathBuf,
};

fn package_json_from_raw(raw: &str) -> PackageJson {
  PackageJson::from_raw(raw.to_string(), PathBuf::from("/packages/test/package.json")).expect("Failed to parse test package.json")
}

fn serialise_pkg(pkg: &PackageJson, formatting: DetectedFormatting) -> String {
  let file = File {
    filepath: PathBuf::from("/test"),
    formatting,
    contents: pkg.contents(),
  };
  String::from_utf8(get_pretty_json_bytes(&file).unwrap()).unwrap()
}

fn make_instance(name: &str, dep_type: DependencyType, expected: &str) -> Instance {
  let specifier = Specifier::new(expected);
  let descriptor = crate::instance::InstanceDescriptor {
    dependency_type: dep_type,
    internal_name: name.to_string(),
    is_local_dependency: false,
    matches_cli_filter: false,
    name: name.to_string(),
    package_idx: PackageIdx(0),
    specifier: Specifier::new("0.0.0"), // ignored — overridden by mark_fixable below
  };
  let instance = Instance::new(descriptor, "test-pkg", None);
  instance.mark_fixable(FixableInstance::DiffersToHighestOrLowestSemver, &specifier);
  instance
}

// --- Indent detection ---

#[test]
fn serialize_uses_detected_2_space_indent() {
  let raw = "{\n  \"name\": \"pkg\",\n  \"version\": \"1.0.0\"\n}\n";
  let pkg = package_json_from_raw(raw);
  let fmt = detect_formatting(raw);
  let result = serialise_pkg(&pkg, fmt);
  assert!(result.contains("  \"name\""), "expected 2-space indent, got:\n{result}");
  assert!(
    !result.contains("    \"name\""),
    "expected 2-space indent but got 4-space, got:\n{result}"
  );
}

#[test]
fn serialize_uses_detected_4_space_indent() {
  let raw = "{\n    \"name\": \"pkg\",\n    \"version\": \"1.0.0\"\n}\n";
  let pkg = package_json_from_raw(raw);
  let fmt = detect_formatting(raw);
  let result = serialise_pkg(&pkg, fmt);
  assert!(result.contains("    \"name\""), "expected 4-space indent, got:\n{result}");
}

#[test]
fn serialize_uses_detected_tab_indent() {
  let raw = "{\n\t\"name\": \"pkg\",\n\t\"version\": \"1.0.0\"\n}\n";
  let pkg = package_json_from_raw(raw);
  let fmt = detect_formatting(raw);
  let result = serialise_pkg(&pkg, fmt);
  assert!(result.contains("\t\"name\""), "expected tab indent, got:\n{result}");
}

#[test]
fn serialize_uses_overridden_indent() {
  // File uses 4-space indent
  let raw = "{\n    \"name\": \"pkg\",\n    \"version\": \"1.0.0\"\n}\n";
  let pkg = package_json_from_raw(raw);
  let mut fmt = detect_formatting(raw);
  // Config says 2-space — override indent but keep detected newline
  fmt.indent = "  ".to_string();
  let result = serialise_pkg(&pkg, fmt);
  assert!(
    result.contains("  \"name\""),
    "expected config 2-space indent to win, got:\n{result}"
  );
  assert!(
    !result.contains("    \"name\""),
    "expected config indent to override detected 4-space, got:\n{result}"
  );
}

// --- Newline detection ---

#[test]
fn serialize_preserves_lf_newline() {
  let raw = "{\n  \"name\": \"pkg\"\n}\n";
  let pkg = package_json_from_raw(raw);
  let fmt = detect_formatting(raw);
  let result = serialise_pkg(&pkg, fmt);
  assert!(result.ends_with('\n'), "expected trailing LF");
  assert!(
    !result.ends_with("\r\n"),
    "expected LF only, not CRLF, got bytes: {:?}",
    result.as_bytes().iter().rev().take(4).collect::<Vec<_>>()
  );
}

#[test]
fn serialize_preserves_crlf_newline() {
  let raw = "{\r\n  \"name\": \"pkg\"\r\n}\r\n";
  let pkg = package_json_from_raw(raw);
  let fmt = detect_formatting(raw);
  let result = serialise_pkg(&pkg, fmt);
  assert!(
    result.ends_with("\r\n"),
    "expected trailing CRLF, got bytes: {:?}",
    result.as_bytes().iter().rev().take(4).collect::<Vec<_>>()
  );
}

// --- Dirty flag: set_prop ---

#[test]
fn from_raw_is_not_dirty() {
  let pkg = package_json_from_raw("{\"name\": \"pkg\", \"version\": \"1.0.0\"}");
  assert!(!pkg.is_dirty());
}

#[test]
fn set_prop_marks_dirty_when_value_changes() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"version\": \"1.0.0\"}");
  pkg.set_prop("/version", json!("2.0.0"));
  assert!(pkg.is_dirty());
  assert_eq!(pkg.get_prop("/version"), Some(json!("2.0.0")));
}

#[test]
fn set_prop_does_not_mark_dirty_when_value_unchanged() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"version\": \"1.0.0\"}");
  pkg.set_prop("/version", json!("1.0.0"));
  assert!(!pkg.is_dirty());
}

#[test]
fn set_prop_detects_object_key_reorder() {
  let mut pkg = package_json_from_raw("{\"b\": 1, \"a\": 2}");
  let mut reordered = Map::new();
  reordered.insert("a".to_string(), json!(2));
  reordered.insert("b".to_string(), json!(1));
  pkg.set_prop("/", Value::Object(reordered));
  assert!(pkg.is_dirty(), "reordering object keys should mark dirty");
}

#[test]
fn set_prop_does_not_mark_dirty_when_object_key_order_same() {
  let mut pkg = package_json_from_raw("{\"a\": 1, \"b\": 2}");
  let same_contents = pkg.contents().clone();
  pkg.set_prop("/", same_contents);
  assert!(!pkg.is_dirty());
}

// --- Dirty flag: set_nested_prop ---

#[test]
fn set_nested_prop_marks_dirty_when_value_changes() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"react\": \"17.0.0\"}}");
  pkg.set_nested_prop("/dependencies", "react", json!("18.0.0"));
  assert!(pkg.is_dirty());
  assert_eq!(pkg.get_prop("/dependencies/react"), Some(json!("18.0.0")));
}

#[test]
fn set_nested_prop_does_not_mark_dirty_when_value_unchanged() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"react\": \"17.0.0\"}}");
  pkg.set_nested_prop("/dependencies", "react", json!("17.0.0"));
  assert!(!pkg.is_dirty());
}

#[test]
fn set_nested_prop_works_with_slash_in_key() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"@scope/lib\": \"1.0.0\"}}");
  pkg.set_nested_prop("/dependencies", "@scope/lib", json!("2.0.0"));
  assert!(pkg.is_dirty());
  let deps = pkg.contents().pointer("/dependencies").unwrap().as_object().unwrap();
  assert_eq!(deps.get("@scope/lib"), Some(&json!("2.0.0")));
}

// --- Dirty flag: remove_prop ---

#[test]
fn remove_prop_marks_dirty_when_key_exists() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"react\": \"17.0.0\"}}");
  pkg.remove_prop("/dependencies", "react");
  assert!(pkg.is_dirty());
  let deps = pkg.contents().pointer("/dependencies").unwrap().as_object().unwrap();
  assert!(!deps.contains_key("react"));
}

#[test]
fn remove_prop_does_not_mark_dirty_when_key_missing() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"react\": \"17.0.0\"}}");
  pkg.remove_prop("/dependencies", "lodash");
  assert!(!pkg.is_dirty());
}

#[test]
fn remove_prop_works_with_slash_in_key() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"@scope/lib\": \"1.0.0\"}}");
  pkg.remove_prop("/dependencies", "@scope/lib");
  assert!(pkg.is_dirty());
  let deps = pkg.contents().pointer("/dependencies").unwrap().as_object().unwrap();
  assert!(!deps.contains_key("@scope/lib"));
}

// --- copy_expected_specifier: VersionsByName ---

#[test]
fn copy_expected_specifier_versions_by_name() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"react\": \"17.0.0\"}}");
  let dep_type = DependencyType {
    name_path: None,
    name: "prod".to_string(),
    path: "/dependencies".to_string(),
    strategy: Strategy::VersionsByName,
  };
  let instance = make_instance("react", dep_type, "18.0.0");
  pkg.copy_expected_specifier(&instance);
  assert!(pkg.is_dirty());
  assert_eq!(pkg.get_prop("/dependencies/react"), Some(json!("18.0.0")));
}

#[test]
fn copy_expected_specifier_versions_by_name_scoped_package() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"@scope/lib\": \"1.0.0\"}}");
  let dep_type = DependencyType {
    name_path: None,
    name: "prod".to_string(),
    path: "/dependencies".to_string(),
    strategy: Strategy::VersionsByName,
  };
  let instance = make_instance("@scope/lib", dep_type, "2.0.0");
  pkg.copy_expected_specifier(&instance);
  assert!(pkg.is_dirty(), "scoped package specifier was not applied");
  let deps = pkg.contents().pointer("/dependencies").unwrap().as_object().unwrap();
  assert_eq!(deps.get("@scope/lib"), Some(&json!("2.0.0")));
}

#[test]
fn copy_expected_specifier_versions_by_name_deeply_nested_path() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"config\": {\"custom\": {\"dependencies\": {\"@scope/lib\": \"1.0.0\"}}}}");
  let dep_type = DependencyType {
    name_path: None,
    name: "customDeps".to_string(),
    path: "/config/custom/dependencies".to_string(),
    strategy: Strategy::VersionsByName,
  };
  let instance = make_instance("@scope/lib", dep_type, "2.0.0");
  pkg.copy_expected_specifier(&instance);
  assert!(pkg.is_dirty(), "deeply nested scoped package specifier was not applied");
  let deps = pkg.contents().pointer("/config/custom/dependencies").unwrap().as_object().unwrap();
  assert_eq!(deps.get("@scope/lib"), Some(&json!("2.0.0")));
}

#[test]
fn copy_expected_specifier_versions_by_name_no_op() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"react\": \"18.0.0\"}}");
  let dep_type = DependencyType {
    name_path: None,
    name: "prod".to_string(),
    path: "/dependencies".to_string(),
    strategy: Strategy::VersionsByName,
  };
  let instance = make_instance("react", dep_type, "18.0.0");
  pkg.copy_expected_specifier(&instance);
  assert!(!pkg.is_dirty(), "same value should not mark dirty");
}

// --- copy_expected_specifier: NameAndVersionProps ---

#[test]
fn copy_expected_specifier_name_and_version_props() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"version\": \"1.0.0\"}");
  let dep_type = DependencyType {
    name_path: Some("/name".to_string()),
    name: "local".to_string(),
    path: "/version".to_string(),
    strategy: Strategy::NameAndVersionProps,
  };
  let instance = make_instance("pkg", dep_type, "2.0.0");
  pkg.copy_expected_specifier(&instance);
  assert!(pkg.is_dirty());
  assert_eq!(pkg.get_prop("/version"), Some(json!("2.0.0")));
}

// --- copy_expected_specifier: NamedVersionString ---

#[test]
fn copy_expected_specifier_named_version_string() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"packageManager\": \"pnpm@7.0.0\"}");
  let dep_type = DependencyType {
    name_path: None,
    name: "packageManager".to_string(),
    path: "/packageManager".to_string(),
    strategy: Strategy::NamedVersionString,
  };
  let instance = make_instance("pnpm", dep_type, "8.0.0");
  pkg.copy_expected_specifier(&instance);
  assert!(pkg.is_dirty());
  assert_eq!(pkg.get_prop("/packageManager"), Some(json!("pnpm@8.0.0")));
}

// --- copy_expected_specifier: UnnamedVersionString ---

#[test]
fn copy_expected_specifier_unnamed_version_string() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"engines\": {\"node\": \">=16\"}}");
  let dep_type = DependencyType {
    name_path: None,
    name: "node".to_string(),
    path: "/engines/node".to_string(),
    strategy: Strategy::UnnamedVersionString,
  };
  let instance = make_instance("node", dep_type, ">=18");
  pkg.copy_expected_specifier(&instance);
  assert!(pkg.is_dirty());
  assert_eq!(pkg.get_prop("/engines/node"), Some(json!(">=18")));
}

// --- from_raw edge cases ---

#[test]
fn from_raw_returns_none_for_invalid_json() {
  let result = PackageJson::from_raw("not json".to_string(), PathBuf::from("/test/package.json"));
  assert!(result.is_none());
}

#[test]
fn from_raw_uses_fallback_name_when_name_missing() {
  let pkg = package_json_from_raw("{\"version\": \"1.0.0\"}");
  assert_eq!(pkg.name, "NAME_IS_MISSING");
}

// --- has_prop ---

#[test]
fn has_prop_returns_true_when_exists() {
  let pkg = package_json_from_raw("{\"name\": \"pkg\", \"version\": \"1.0.0\"}");
  assert!(pkg.has_prop("/version"));
}

#[test]
fn has_prop_returns_false_when_missing() {
  let pkg = package_json_from_raw("{\"name\": \"pkg\"}");
  assert!(!pkg.has_prop("/version"));
}

// --- No-op when path does not exist ---

#[test]
fn set_prop_is_noop_when_path_missing() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\"}");
  pkg.set_prop("/nonexistent/deep/path", json!("value"));
  assert!(!pkg.is_dirty());
}

#[test]
fn set_nested_prop_is_noop_when_parent_missing() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\"}");
  pkg.set_nested_prop("/nonexistent", "key", json!("value"));
  assert!(!pkg.is_dirty());
}

#[test]
fn set_nested_prop_is_noop_when_parent_is_not_object() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\", \"version\": \"1.0.0\"}");
  pkg.set_nested_prop("/version", "key", json!("value"));
  assert!(!pkg.is_dirty());
}

#[test]
fn remove_prop_is_noop_when_parent_missing() {
  let mut pkg = package_json_from_raw("{\"name\": \"pkg\"}");
  pkg.remove_prop("/nonexistent", "key");
  assert!(!pkg.is_dirty());
}

// --- values_differ: nested reordering ---

#[test]
fn set_prop_detects_nested_object_key_reorder() {
  let mut pkg = package_json_from_raw("{\"deps\": {\"b\": 1, \"a\": 2}}");
  let mut reordered = Map::new();
  reordered.insert("a".to_string(), json!(2));
  reordered.insert("b".to_string(), json!(1));
  pkg.set_prop("/deps", Value::Object(reordered));
  assert!(pkg.is_dirty(), "reordering nested object keys should mark dirty");
}

// --- Serialisation key order ---

#[test]
fn serialize_preserves_key_order() {
  let raw = "{\n  \"z\": 1,\n  \"a\": 2,\n  \"m\": 3\n}\n";
  let pkg = package_json_from_raw(raw);
  let fmt = detect_formatting(raw);
  let result = serialise_pkg(&pkg, fmt);
  let z_pos = result.find("\"z\"").unwrap();
  let a_pos = result.find("\"a\"").unwrap();
  let m_pos = result.find("\"m\"").unwrap();
  assert!(z_pos < a_pos && a_pos < m_pos, "key order not preserved, got:\n{result}");
}
