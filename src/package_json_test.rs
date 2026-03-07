use {
  crate::{config::Config, package_json::PackageJson, test::mock},
  std::path::PathBuf,
};

fn package_json_from_raw(raw: &str) -> PackageJson {
  PackageJson::from_raw(raw, PathBuf::from("/packages/test/package.json")).expect("Failed to parse test package.json")
}

fn config_with_indent(indent: &str) -> Config {
  mock::config_from_mock(serde_json::json!({ "indent": indent }))
}

fn config_no_indent() -> Config {
  mock::config()
}

// --- Rcfile defaults ---

#[test]
fn default_indent_is_none() {
  let rcfile = mock::rcfile();
  assert!(rcfile.indent.is_none(), "expected indent to be None when not configured");
}

#[test]
fn configured_indent_is_some() {
  let rcfile = mock::rcfile_from_mock(serde_json::json!({ "indent": "\t" }));
  assert_eq!(rcfile.indent, Some("\t".to_string()));
}

// --- Indent detection ---

#[test]
fn serialize_uses_detected_2_space_indent() {
  let raw = "{\n  \"name\": \"pkg\",\n  \"version\": \"1.0.0\"\n}\n";
  let pkg = package_json_from_raw(raw);
  let result = String::from_utf8(pkg.serialize(None)).unwrap();
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
  let result = String::from_utf8(pkg.serialize(None)).unwrap();
  assert!(result.contains("    \"name\""), "expected 4-space indent, got:\n{result}");
}

#[test]
fn serialize_uses_detected_tab_indent() {
  let raw = "{\n\t\"name\": \"pkg\",\n\t\"version\": \"1.0.0\"\n}\n";
  let pkg = package_json_from_raw(raw);
  let result = String::from_utf8(pkg.serialize(None)).unwrap();
  assert!(result.contains("\t\"name\""), "expected tab indent, got:\n{result}");
}

#[test]
fn serialize_config_indent_overrides_detected() {
  // File uses 4-space indent
  let raw = "{\n    \"name\": \"pkg\",\n    \"version\": \"1.0.0\"\n}\n";
  let pkg = package_json_from_raw(raw);
  // Config says 2-space
  let result = String::from_utf8(pkg.serialize(Some("  "))).unwrap();
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
  let result = String::from_utf8(pkg.serialize(None)).unwrap();
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
  let result = String::from_utf8(pkg.serialize(None)).unwrap();
  assert!(
    result.ends_with("\r\n"),
    "expected trailing CRLF, got bytes: {:?}",
    result.as_bytes().iter().rev().take(4).collect::<Vec<_>>()
  );
}

// --- write_to_disk uses detection ---

#[test]
fn write_to_disk_uses_detected_indent_when_config_is_not_set() {
  let raw = "{\n    \"name\": \"pkg\",\n    \"version\": \"1.0.0\"\n}\n";
  let pkg = package_json_from_raw(raw);
  let config = config_no_indent();
  let serialized = String::from_utf8(pkg.serialize(config.rcfile.indent.as_deref())).unwrap();
  assert!(
    serialized.contains("    \"name\""),
    "expected 4-space indent from file detection, got:\n{serialized}"
  );
}

#[test]
fn write_to_disk_uses_config_indent_when_set() {
  let raw = "{\n    \"name\": \"pkg\",\n    \"version\": \"1.0.0\"\n}\n";
  let pkg = package_json_from_raw(raw);
  let config = config_with_indent("  ");
  let serialized = String::from_utf8(pkg.serialize(config.rcfile.indent.as_deref())).unwrap();
  assert!(
    serialized.contains("  \"name\""),
    "expected config 2-space indent, got:\n{serialized}"
  );
}
