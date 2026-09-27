use {super::*, serde_json::json};

#[test]
fn detects_two_space_indent_and_lf_newline() {
  let formatting = detect_formatting("{\n  \"a\": 1\n}\n");
  assert_eq!(formatting.indent, "  ");
  assert_eq!(formatting.newline, "\n");
}

#[test]
fn detects_tab_indent_and_crlf_newline() {
  let formatting = detect_formatting("{\r\n\t\"a\": 1\r\n}\r\n");
  assert_eq!(formatting.indent, "\t");
  assert_eq!(formatting.newline, "\r\n");
}

#[test]
fn falls_back_to_two_spaces_when_indent_cannot_be_detected() {
  // A single-line file gives detect_indent nothing to infer an indent from.
  let formatting = detect_formatting("{\"a\":1}");
  assert_eq!(formatting.indent, "  ");
}

#[test]
fn json_values_differ_is_order_aware_for_objects() {
  // serde_json's `preserve_order` `Value::eq` ignores key order, but
  // syncpack's sort rules care about it — hence this separate helper.
  let a = json!({ "a": 1, "b": 2 });
  let b = json!({ "b": 2, "a": 1 });
  assert!(json_values_differ(&a, &b));
}

#[test]
fn json_values_differ_is_false_for_identical_objects() {
  let a = json!({ "a": 1, "b": 2 });
  let b = json!({ "a": 1, "b": 2 });
  assert!(!json_values_differ(&a, &b));
}

#[test]
fn json_values_differ_detects_array_order_changes() {
  let a = json!(["react", "vue"]);
  let b = json!(["vue", "react"]);
  assert!(json_values_differ(&a, &b));
}

fn json_file(contents: serde_json::Value) -> File<JsonValue> {
  File {
    filepath: PathBuf::from("package.json"),
    formatting: DetectedFormatting::default(),
    contents,
    dirty: false,
  }
}

#[test]
fn set_prop_is_a_noop_when_the_value_is_unchanged() {
  let mut file = json_file(json!({ "version": "1.0.0" }));
  set_prop(&mut file, "/version", json!("1.0.0"));
  assert!(!file.dirty);
}

#[test]
fn set_prop_marks_dirty_and_writes_through_when_the_value_changes() {
  let mut file = json_file(json!({ "version": "1.0.0" }));
  set_prop(&mut file, "/version", json!("2.0.0"));
  assert!(file.dirty);
  assert_eq!(file.contents["version"], json!("2.0.0"));
}

#[test]
fn set_nested_prop_is_a_noop_when_the_value_is_unchanged() {
  let mut file = json_file(json!({ "dependencies": { "react": "^18.0.0" } }));
  set_nested_prop(&mut file, "/dependencies", "react", json!("^18.0.0"));
  assert!(!file.dirty);
}

#[test]
fn set_nested_prop_marks_dirty_when_the_value_changes() {
  let mut file = json_file(json!({ "dependencies": { "react": "^18.0.0" } }));
  set_nested_prop(&mut file, "/dependencies", "react", json!("^19.0.0"));
  assert!(file.dirty);
  assert_eq!(file.contents["dependencies"]["react"], json!("^19.0.0"));
}

#[test]
fn render_yaml_bytes_preserves_comments_around_a_replaced_value() {
  let raw = "# top comment\ncatalog:\n  react: ^18.0.0 # inline comment\n";
  let mut yaml = parse_yaml_file(raw.to_string(), PathBuf::from("pnpm-workspace.yaml")).expect("valid yaml");
  yaml.patches.push(PendingYamlOp::Replace {
    segments: vec!["catalog".to_string(), "react".to_string()],
    value: yaml_serde::Value::String("^19.0.0".to_string()),
  });
  yaml.dirty = true;

  let bytes = render_yaml_bytes(&yaml).expect("patch applies");
  let out = String::from_utf8(bytes).unwrap();

  assert!(out.contains("# top comment"), "leading comment preserved:\n{out}");
  assert!(out.contains("# inline comment"), "inline comment preserved:\n{out}");
  assert!(out.contains("react: ^19.0.0"), "value replaced:\n{out}");
}

#[test]
fn render_yaml_bytes_skips_a_remove_whose_route_no_longer_resolves() {
  // Removing the only key under `catalog:` collapses the parent block, so a
  // second Remove that still targets `catalog` no longer resolves. Per
  // `render_yaml_bytes`'s documented contract this is skipped, not an error.
  let raw = "catalog:\n  react: ^18.0.0\n";
  let mut yaml = parse_yaml_file(raw.to_string(), PathBuf::from("pnpm-workspace.yaml")).expect("valid yaml");
  yaml.patches.push(PendingYamlOp::Remove {
    segments: vec!["catalog".to_string(), "react".to_string()],
  });
  yaml.patches.push(PendingYamlOp::Remove {
    segments: vec!["catalog".to_string()],
  });
  yaml.dirty = true;

  let result = render_yaml_bytes(&yaml);

  assert!(result.is_ok(), "dangling remove should be skipped, not error: {result:?}");
}

#[test]
fn render_yaml_bytes_surfaces_a_genuine_missing_route_as_an_error() {
  let raw = "catalog:\n  react: ^18.0.0\n";
  let mut yaml = parse_yaml_file(raw.to_string(), PathBuf::from("pnpm-workspace.yaml")).expect("valid yaml");
  yaml.patches.push(PendingYamlOp::Replace {
    segments: vec!["catalog".to_string(), "missing-dep".to_string()],
    value: yaml_serde::Value::String("^1.0.0".to_string()),
  });
  yaml.dirty = true;

  let result = render_yaml_bytes(&yaml);

  assert!(
    matches!(result, Err(DiskIoError::YamlPatchApply(_))),
    "expected YamlPatchApply error, got: {result:?}"
  );
}
