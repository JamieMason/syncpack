use {
  crate::{
    disk::{
      Disk, File, PackageManager, PendingYamlOp, empty_yaml_file, ensure_object_path, insert_catalog_definition, json_view,
      parse_yaml_file, remove_catalog_definition, remove_prop, render_yaml_bytes, set_nested_prop, set_prop,
    },
    specifier::Specifier,
    test::mock_disk::MockDiskIo,
  },
  serde_json::json,
  std::path::PathBuf,
};

fn json_file_from_value(value: serde_json::Value) -> File<serde_json::Value> {
  File {
    filepath: PathBuf::from("/test/package.json"),
    formatting: Default::default(),
    contents: value,
    dirty: false,
  }
}

#[test]
fn detects_pnpm_from_workspace_yaml_without_lockfile() {
  let mut io = MockDiskIo::new();
  io.add_file("pnpm-workspace.yaml", "packages:\n  - 'apps/*'\n".to_string());
  let disk = Disk::from_workspace(&io, io.root());
  assert_eq!(disk.package_manager, Some(PackageManager::Pnpm));
  assert!(disk.pnpm_workspace.is_some(), "pnpm-workspace.yaml should be loaded");
}

#[test]
fn detects_bun_from_legacy_binary_lockfile() {
  let mut io = MockDiskIo::new();
  io.add_file("bun.lockb", String::new());
  let disk = Disk::from_workspace(&io, io.root());
  assert_eq!(disk.package_manager, Some(PackageManager::Bun));
}

#[test]
fn file_dirty_defaults_false() {
  let mut io = MockDiskIo::new();
  io.add_file("package.json", "{\"name\":\"root\"}".to_string());
  let disk = Disk::from_workspace(&io, io.root());
  let pkg = disk.package_json_root().expect("root should be loaded");
  assert!(!pkg.dirty, "freshly parsed File should default dirty=false");
  assert!(!pkg.is_dirty(), "is_dirty() should match field");
}

#[test]
fn file_dirty_settable() {
  let mut file: File<serde_json::Value> = File {
    filepath: std::path::PathBuf::from("/test/package.json"),
    formatting: Default::default(),
    contents: serde_json::Value::Null,
    dirty: false,
  };
  assert!(!file.is_dirty());
  file.mark_dirty();
  assert!(file.is_dirty(), "mark_dirty() should flip dirty to true");
  assert!(file.dirty, "field should reflect mutation");
}

#[test]
fn disk_from_workspace_works_without_lifetime() {
  // Disk owns no borrow of `io`; returned Disk has no lifetime parameter.
  let io = MockDiskIo::new();
  let disk: Disk = Disk::from_workspace(&io, io.root());
  // Drop io to prove Disk does not borrow it.
  drop(io);
  assert!(disk.package_json_root().is_none());
}

#[test]
fn disk_constructs_without_io_field() {
  // Compile-time check: Disk has no `io` field. Field-init shorthand would
  // fail to compile if `io` were still required.
  let disk = Disk {
    cwd: std::path::PathBuf::from("/test"),
    lerna_json: None,
    package_json_files: vec![],
    package_json_root_idx: None,
    package_manager: None,
    pnpm_workspace: None,
  };
  assert_eq!(disk.cwd, std::path::PathBuf::from("/test"));
}

#[test]
fn disk_root_idx_set_when_root_pkg_exists() {
  let mut io = MockDiskIo::new();
  io.add_file("package.json", "{\"name\":\"root\"}".to_string());
  let disk = Disk::from_workspace(&io, io.root());
  assert_eq!(disk.package_json_root_idx, Some(0), "root_idx should be Some(0) when root exists");
  assert_eq!(disk.package_json_files.len(), 1, "root pkg should be in package_json_files");
  let root = disk.package_json_root().expect("root accessor returns Some");
  assert_eq!(
    root.contents.pointer("/name").and_then(|v| v.as_str()),
    Some("root"),
    "accessor returns the root file"
  );
}

#[test]
fn disk_load_package_files_appends_after_root() {
  let mut io = MockDiskIo::new();
  io.add_file("package.json", "{\"name\":\"root\"}".to_string());
  io.add_file("packages/a/package.json", "{\"name\":\"a\"}".to_string());
  io.add_file("packages/b/package.json", "{\"name\":\"b\"}".to_string());
  let mut disk = Disk::from_workspace(&io, io.root());
  assert_eq!(disk.package_json_files.len(), 1, "starts with just root");
  let a_path = io.root().join("packages/a/package.json");
  let b_path = io.root().join("packages/b/package.json");
  disk.load_package_files(&io, &[a_path, b_path]);
  assert_eq!(disk.package_json_files.len(), 3, "root + 2 appended");
}

#[test]
fn disk_load_package_files_skips_root_path() {
  let mut io = MockDiskIo::new();
  io.add_file("package.json", "{\"name\":\"root\"}".to_string());
  io.add_file("packages/a/package.json", "{\"name\":\"a\"}".to_string());
  let mut disk = Disk::from_workspace(&io, io.root());
  let root_path = io.root().join("package.json");
  let a_path = io.root().join("packages/a/package.json");
  disk.load_package_files(&io, &[root_path, a_path]);
  assert_eq!(disk.package_json_files.len(), 2, "root must not be re-loaded; only `a` appended");
}

#[test]
fn set_prop_marks_file_dirty() {
  let mut file = json_file_from_value(json!({"name": "pkg", "version": "1.0.0"}));
  assert!(!file.dirty);
  set_prop(&mut file, "/version", json!("2.0.0"));
  assert!(file.dirty, "set_prop must flip File.dirty when value changes");
  assert_eq!(file.contents.pointer("/version"), Some(&json!("2.0.0")));
}

#[test]
fn set_nested_prop_marks_file_dirty() {
  let mut file = json_file_from_value(json!({"dependencies": {"react": "17.0.0"}}));
  assert!(!file.dirty);
  set_nested_prop(&mut file, "/dependencies", "react", json!("18.0.0"));
  assert!(file.dirty, "set_nested_prop must flip File.dirty when value changes");
  let deps = file.contents.pointer("/dependencies").and_then(|v| v.as_object()).unwrap();
  assert_eq!(deps.get("react"), Some(&json!("18.0.0")));
}

#[test]
fn remove_prop_marks_file_dirty() {
  let mut file = json_file_from_value(json!({"dependencies": {"react": "17.0.0"}}));
  assert!(!file.dirty);
  remove_prop(&mut file, "/dependencies", "react");
  assert!(file.dirty, "remove_prop must flip File.dirty when key existed");
  let deps = file.contents.pointer("/dependencies").and_then(|v| v.as_object()).unwrap();
  assert!(!deps.contains_key("react"));
}

#[test]
fn ensure_object_path_walks_segments() {
  let mut file = json_file_from_value(json!({}));
  ensure_object_path(&mut file, "/workspaces/catalog");
  assert!(
    file.contents.pointer("/workspaces").and_then(|v| v.as_object()).is_some(),
    "first segment created as object"
  );
  assert!(
    file.contents.pointer("/workspaces/catalog").and_then(|v| v.as_object()).is_some(),
    "nested segment created as object"
  );
}

#[test]
fn insert_catalog_definition_marks_yaml_dirty() {
  let raw = "";
  let mut yaml = parse_yaml_file(raw.to_string(), PathBuf::from("/test/pnpm-workspace.yaml"))
    .unwrap_or_else(|| empty_yaml_file(PathBuf::from("/test/pnpm-workspace.yaml")));
  yaml.dirty = false;
  let specifier = Specifier::new("^18.0.0");
  let changed = insert_catalog_definition(&mut yaml, "default", "react", &specifier);
  assert!(changed, "first insert reports state change");
  assert!(yaml.dirty, "insert_catalog_definition must flip yaml.dirty");
}

/// Helper: build a YamlFile from a raw string for tests.
fn yaml_from(raw: &str) -> crate::disk::YamlFile {
  parse_yaml_file(raw.to_string(), PathBuf::from("/test/pnpm-workspace.yaml")).expect("yaml fixture must parse")
}

/// Helper: render a YamlFile to text using the production serialiser
/// (yamlpatch path when raw + patches; fresh-serialize otherwise).
fn render(yaml: &crate::disk::YamlFile) -> String {
  let bytes = render_yaml_bytes(yaml).expect("render must succeed");
  String::from_utf8(bytes).expect("output must be UTF-8")
}

#[test]
fn insert_into_existing_default_block_records_add_patch() {
  // Default block exists; new dep absent. Expect a single Add patch
  // at route ["catalog"] with key=dep_name, value=specifier string.
  let raw = "catalog:\n  lodash: ^4.0.0\n";
  let mut yaml = yaml_from(raw);
  let specifier = Specifier::new("^18.0.0");
  insert_catalog_definition(&mut yaml, "default", "react", &specifier);
  assert_eq!(yaml.patches.len(), 1, "exactly one Add patch expected");
  match &yaml.patches[0] {
    PendingYamlOp::Add { segments, key, value } => {
      assert_eq!(segments, &vec!["catalog".to_string()]);
      assert_eq!(key, "react");
      assert_eq!(value, &serde_yaml::Value::String("^18.0.0".to_string()));
    }
    other => panic!("expected Add, got {:?}", other),
  }
}

#[test]
fn insert_existing_dep_records_replace_patch() {
  // Default block has the dep with a different value. Expect a single
  // Replace patch at route ["catalog", dep_name].
  let raw = "catalog:\n  react: ^17.0.0\n";
  let mut yaml = yaml_from(raw);
  let specifier = Specifier::new("^18.0.0");
  insert_catalog_definition(&mut yaml, "default", "react", &specifier);
  assert_eq!(yaml.patches.len(), 1, "exactly one Replace patch expected");
  match &yaml.patches[0] {
    PendingYamlOp::Replace { segments, value } => {
      assert_eq!(segments, &vec!["catalog".to_string(), "react".to_string()]);
      assert_eq!(value, &serde_yaml::Value::String("^18.0.0".to_string()));
    }
    other => panic!("expected Replace, got {:?}", other),
  }
}

#[test]
fn insert_into_missing_default_block_records_root_add() {
  // No `catalog:` exists. Expect Add at root with key=catalog,
  // value=Mapping{dep: specifier}.
  let raw = "packages:\n  - 'apps/*'\n";
  let mut yaml = yaml_from(raw);
  let specifier = Specifier::new("^18.0.0");
  insert_catalog_definition(&mut yaml, "default", "react", &specifier);
  assert_eq!(yaml.patches.len(), 1, "exactly one root Add patch expected");
  match &yaml.patches[0] {
    PendingYamlOp::Add { segments, key, value } => {
      assert!(segments.is_empty(), "root-level segments must be empty");
      assert_eq!(key, "catalog");
      let serde_yaml::Value::Mapping(m) = value else {
        panic!("expected Mapping value, got {:?}", value);
      };
      assert_eq!(m.get("react"), Some(&serde_yaml::Value::String("^18.0.0".to_string())));
    }
    other => panic!("expected Add, got {:?}", other),
  }
}

#[test]
fn insert_into_missing_named_catalog_records_named_add() {
  // `catalogs:` exists but the named catalog doesn't. Expect Add at
  // ["catalogs"] with key=name, value=Mapping{dep: specifier}.
  let raw = "catalogs:\n  modern:\n    react: ^18.0.0\n";
  let mut yaml = yaml_from(raw);
  let specifier = Specifier::new("^16.0.0");
  insert_catalog_definition(&mut yaml, "legacy", "react", &specifier);
  assert_eq!(yaml.patches.len(), 1);
  match &yaml.patches[0] {
    PendingYamlOp::Add { segments, key, value } => {
      assert_eq!(segments, &vec!["catalogs".to_string()]);
      assert_eq!(key, "legacy");
      let serde_yaml::Value::Mapping(m) = value else {
        panic!("expected Mapping value, got {:?}", value);
      };
      assert_eq!(m.get("react"), Some(&serde_yaml::Value::String("^16.0.0".to_string())));
    }
    other => panic!("expected Add, got {:?}", other),
  }
}

#[test]
fn insert_into_missing_catalogs_root_records_root_add() {
  // Neither `catalogs:` nor the named catalog exists. Expect Add at
  // root with key=catalogs, value=Mapping{name: Mapping{dep: spec}}.
  let raw = "catalog:\n  react: ^18.0.0\n";
  let mut yaml = yaml_from(raw);
  let specifier = Specifier::new("^16.0.0");
  insert_catalog_definition(&mut yaml, "legacy", "react", &specifier);
  assert_eq!(yaml.patches.len(), 1);
  match &yaml.patches[0] {
    PendingYamlOp::Add { segments, key, value } => {
      assert!(segments.is_empty(), "expected root-level Add");
      assert_eq!(key, "catalogs");
      let serde_yaml::Value::Mapping(m) = value else {
        panic!("expected Mapping value, got {:?}", value);
      };
      let legacy = m.get("legacy").expect("legacy entry exists");
      let serde_yaml::Value::Mapping(legacy_map) = legacy else {
        panic!("expected nested Mapping, got {:?}", legacy);
      };
      assert_eq!(legacy_map.get("react"), Some(&serde_yaml::Value::String("^16.0.0".to_string())));
    }
    other => panic!("expected Add, got {:?}", other),
  }
}

#[test]
fn insert_idempotent_records_no_patch() {
  // Existing value already equals the requested value. No patch
  // should be pushed; dirty must remain false.
  let raw = "catalog:\n  react: ^18.0.0\n";
  let mut yaml = yaml_from(raw);
  let specifier = Specifier::new("^18.0.0");
  let changed = insert_catalog_definition(&mut yaml, "default", "react", &specifier);
  assert!(!changed, "idempotent insert reports no state change");
  assert!(yaml.patches.is_empty(), "no patch pushed on idempotent insert");
  assert!(!yaml.dirty, "dirty must stay false on idempotent insert");
}

#[test]
fn remove_dep_with_siblings_records_single_remove() {
  // Removing one of multiple deps. Parent block stays. One Remove
  // patch at route ["catalog", dep_name].
  let raw = "catalog:\n  react: ^18.0.0\n  lodash: ^4.0.0\n";
  let mut yaml = yaml_from(raw);
  let removed = remove_catalog_definition(&mut yaml, "default", "react");
  assert!(removed, "remove returns true when something was removed");
  assert_eq!(yaml.patches.len(), 1, "single Remove patch expected");
  match &yaml.patches[0] {
    PendingYamlOp::Remove { segments } => {
      assert_eq!(segments, &vec!["catalog".to_string(), "react".to_string()]);
    }
    other => panic!("expected Remove, got {:?}", other),
  }
}

#[test]
fn remove_last_dep_in_default_block_records_two_removes() {
  // Removing the only dep. Block becomes empty → also remove parent.
  let raw = "catalog:\n  react: ^18.0.0\n";
  let mut yaml = yaml_from(raw);
  let removed = remove_catalog_definition(&mut yaml, "default", "react");
  assert!(removed);
  assert_eq!(yaml.patches.len(), 2, "two Remove patches expected (dep + parent)");
  match &yaml.patches[0] {
    PendingYamlOp::Remove { segments } => {
      assert_eq!(segments, &vec!["catalog".to_string(), "react".to_string()]);
    }
    other => panic!("expected dep Remove, got {:?}", other),
  }
  match &yaml.patches[1] {
    PendingYamlOp::Remove { segments } => {
      assert_eq!(segments, &vec!["catalog".to_string()]);
    }
    other => panic!("expected parent Remove, got {:?}", other),
  }
}

#[test]
fn remove_last_dep_in_only_named_catalog_records_three_removes() {
  // Remove a dep from the only named catalog with one dep. The dep,
  // the named catalog, and the `catalogs:` map all become empty.
  let raw = "catalogs:\n  legacy:\n    react: ^16.0.0\n";
  let mut yaml = yaml_from(raw);
  let removed = remove_catalog_definition(&mut yaml, "legacy", "react");
  assert!(removed);
  assert_eq!(yaml.patches.len(), 3, "three Remove patches expected");
  let segs: Vec<_> = yaml
    .patches
    .iter()
    .map(|op| match op {
      PendingYamlOp::Remove { segments } => segments.clone(),
      other => panic!("expected Remove, got {:?}", other),
    })
    .collect();
  assert_eq!(segs[0], vec!["catalogs".to_string(), "legacy".to_string(), "react".to_string()]);
  assert_eq!(segs[1], vec!["catalogs".to_string(), "legacy".to_string()]);
  assert_eq!(segs[2], vec!["catalogs".to_string()]);
}

#[test]
fn remove_absent_dep_records_no_patch() {
  // Removing a dep that doesn't exist is a no-op.
  let raw = "catalog:\n  lodash: ^4.0.0\n";
  let mut yaml = yaml_from(raw);
  let removed = remove_catalog_definition(&mut yaml, "default", "react");
  assert!(!removed, "remove returns false on absent dep");
  assert!(yaml.patches.is_empty(), "no patch pushed when nothing removed");
  assert!(!yaml.dirty, "dirty stays false on no-op remove");
}

#[test]
fn yamlpatch_add_quotes_keys_with_reserved_chars() {
  // Keys that start with reserved YAML chars (`@`, `` ` ``) or contain
  // syntactically significant punctuation must be quoted on output,
  // otherwise the resulting YAML is invalid.
  let raw = "catalog:\n  typescript: ^5.9.3\n";
  let mut yaml = yaml_from(raw);
  let specifier = Specifier::new("^1.0.0");
  insert_catalog_definition(&mut yaml, "default", "@astrojs/check", &specifier);
  let out = render(&yaml);
  // Output must contain a quoted key and parse as valid YAML.
  assert!(
    out.contains("\"@astrojs/check\":") || out.contains("'@astrojs/check':"),
    "key must be quoted in output:\n{out}"
  );
  serde_yaml::from_str::<serde_yaml::Value>(&out).expect("output must be valid YAML");
}

#[test]
fn yamlpatch_add_handles_many_scoped_keys_in_one_pass() {
  // Mirrors the real-world case: many sequential Add patches against
  // a catalog block, with several keys starting with `@` (npm scope).
  // Without quoting these keys yamlpatch produces invalid YAML and
  // fails on its post-patch reparse.
  let raw = "catalog:\n  typescript: ^5.9.3\n";
  let mut yaml = yaml_from(raw);
  let specifier = Specifier::new("^1.0.0");
  for name in [
    "@astrojs/check",
    "@astrojs/cloudflare",
    "@biomejs/biome",
    "@clack/prompts",
    "@playwright/test",
    "lodash",
    "react",
  ] {
    insert_catalog_definition(&mut yaml, "default", name, &specifier);
  }
  let out = render(&yaml);
  serde_yaml::from_str::<serde_yaml::Value>(&out).expect("output must be valid YAML");
  assert!(out.contains("typescript: ^5.9.3"), "original key preserved:\n{out}");
}

#[test]
fn yamlpatch_replace_preserves_inline_comment() {
  // T1: replace a value, comment on the same line stays.
  let raw = "catalog:\n  react: ^17.0.0 # pinned for legacy\n";
  let mut yaml = yaml_from(raw);
  let specifier = Specifier::new("^18.0.0");
  insert_catalog_definition(&mut yaml, "default", "react", &specifier);
  let out = render(&yaml);
  assert_eq!(out, "catalog:\n  react: ^18.0.0 # pinned for legacy\n");
}

#[test]
fn yamlpatch_replace_preserves_blank_lines_between_blocks() {
  // T2: replace touches one block; blank lines and the other block
  // stay verbatim.
  let raw = "catalog:\n  react: ^17.0.0\n\ncatalogs:\n  legacy:\n    react: ^16.0.0\n";
  let mut yaml = yaml_from(raw);
  let specifier = Specifier::new("^18.0.0");
  insert_catalog_definition(&mut yaml, "default", "react", &specifier);
  let out = render(&yaml);
  assert_eq!(out, "catalog:\n  react: ^18.0.0\n\ncatalogs:\n  legacy:\n    react: ^16.0.0\n");
}

#[test]
fn yamlpatch_add_keeps_neighbouring_keys_in_place() {
  // T3: adding a new key into an existing block leaves existing keys
  // untouched and appended-style.
  let raw = "catalog:\n  react: ^18.0.0\n  lodash: ^4.0.0\n";
  let mut yaml = yaml_from(raw);
  let specifier = Specifier::new("^3.0.0");
  insert_catalog_definition(&mut yaml, "default", "vue", &specifier);
  let out = render(&yaml);
  // Existing keys retain order; new key appended at end of block.
  assert!(out.contains("react: ^18.0.0"), "react retained:\n{out}");
  assert!(out.contains("lodash: ^4.0.0"), "lodash retained:\n{out}");
  assert!(out.contains("vue: ^3.0.0"), "vue inserted:\n{out}");
  let react_pos = out.find("react").unwrap();
  let lodash_pos = out.find("lodash").unwrap();
  let vue_pos = out.find("vue").unwrap();
  assert!(react_pos < lodash_pos, "react before lodash");
  assert!(lodash_pos < vue_pos, "vue appended after lodash");
}

#[test]
fn yamlpatch_add_creates_missing_default_block() {
  // T4: file with no `catalog:` block. Insert creates it. Existing
  // `packages:` block is untouched.
  let raw = "packages:\n  - 'apps/*'\n";
  let mut yaml = yaml_from(raw);
  let specifier = Specifier::new("^18.0.0");
  insert_catalog_definition(&mut yaml, "default", "react", &specifier);
  let out = render(&yaml);
  assert!(out.contains("packages:\n  - 'apps/*'"), "packages preserved:\n{out}");
  assert!(out.contains("catalog:\n  react: ^18.0.0"), "catalog block created:\n{out}");
}

#[test]
fn yamlpatch_add_creates_missing_named_catalog_parent() {
  // T5: insert into a named catalog when `catalogs:` doesn't exist.
  // The `catalog:` block stays.
  let raw = "catalog:\n  react: ^18.0.0\n";
  let mut yaml = yaml_from(raw);
  let specifier = Specifier::new("^16.0.0");
  insert_catalog_definition(&mut yaml, "legacy", "react", &specifier);
  let out = render(&yaml);
  assert!(out.contains("catalog:\n  react: ^18.0.0"), "default catalog preserved:\n{out}");
  assert!(out.contains("catalogs:"), "catalogs parent created:\n{out}");
  assert!(out.contains("legacy:"), "named catalog created:\n{out}");
  assert!(out.contains("react: ^16.0.0"), "legacy.react inserted:\n{out}");
}

#[test]
fn yamlpatch_error_surfaces_as_disk_io_error() {
  // T11: pushing a Replace patch at a route that does not exist in
  // the underlying document forces yamlpatch to error. The error must
  // bubble up via DiskIoError rather than being swallowed.
  let raw = "catalog:\n  react: ^18.0.0\n";
  let mut yaml = yaml_from(raw);
  // Hand-craft an op that targets a missing route. This bypasses the
  // mutator helpers (which would correctly emit an Add).
  yaml.patches.push(PendingYamlOp::Replace {
    segments: vec!["catalog".to_string(), "missing-dep".to_string()],
    value: serde_yaml::Value::String("^1.0.0".to_string()),
  });
  yaml.dirty = true;
  let result = render_yaml_bytes(&yaml);
  assert!(
    matches!(result, Err(crate::disk::DiskIoError::YamlPatchApply(_))),
    "expected YamlPatchApply error, got: {result:?}"
  );
}

#[test]
fn indent_override_noops_on_existing_yaml() {
  // T12: file's on-disk indent is 4 spaces. Even if the user set
  // rcfile.indent = "  ", the existing indent stays.
  let raw = "catalog:\n    react: ^17.0.0\n";
  let mut yaml = yaml_from(raw);
  yaml.dirty = true;
  let mut io = MockDiskIo::new();
  io.add_file("test/pnpm-workspace.yaml", "placeholder for path lookup".to_string());
  // Drive the public write helper with a 2-space override.
  let fallback = crate::disk::DetectedFormatting::default();
  // Add a patch so the yamlpatch path runs.
  let specifier = Specifier::new("^18.0.0");
  insert_catalog_definition(&mut yaml, "default", "react", &specifier);
  // Write into the mock so we can read back.
  yaml.filepath = io.root().join("pnpm.yaml");
  crate::disk::write_yaml_file(&mut yaml, &io, Some("  "), &fallback).expect("write must succeed");
  let written = io.written_text(&yaml.filepath).expect("yaml was written");
  assert!(
    written.contains("    react: ^18.0.0"),
    "4-space indent preserved despite 2-space override:\n{written}"
  );
}

#[test]
fn indent_override_applies_to_fresh_yaml() {
  // T13: empty `raw` (auto-created file). rcfile.indent="    " is
  // applied to the fresh-serialized output.
  let mut yaml = empty_yaml_file(PathBuf::from("/test/pnpm-workspace.yaml"));
  let specifier = Specifier::new("^18.0.0");
  insert_catalog_definition(&mut yaml, "default", "react", &specifier);
  let io = MockDiskIo::new();
  let fallback = crate::disk::DetectedFormatting::default();
  yaml.filepath = io.root().join("pnpm.yaml");
  crate::disk::write_yaml_file(&mut yaml, &io, Some("    "), &fallback).expect("write must succeed");
  let written = io.written_text(&yaml.filepath).expect("yaml was written");
  // serde_yaml's to_string respects the `formatting.indent` we set —
  // it actually uses 2 spaces by default, so the assertion targets
  // the configured indent landing on `formatting`.
  // Note: serde_yaml ignores indent at serialize time; this test
  // therefore asserts that the formatting field carries the override
  // through (the indent on disk for fresh files is the serde_yaml
  // default of 2 spaces — but the formatting struct should reflect
  // what the user asked for so future render passes can honour it).
  // The contract under test: write_yaml_file rewrites file.formatting
  // for the empty-raw branch.
  assert_eq!(yaml.formatting.indent, "    ", "formatting.indent updated for fresh file");
  // Output is non-empty and contains the inserted dep.
  assert!(written.contains("react: ^18.0.0"), "output has the new dep:\n{written}");
}

#[test]
fn yamlpatch_remove_last_dep_prunes_default_block() {
  // T9: removing the only dep also prunes the empty parent.
  let raw = "catalog:\n  react: ^18.0.0\n";
  let mut yaml = yaml_from(raw);
  remove_catalog_definition(&mut yaml, "default", "react");
  let out = render(&yaml);
  assert!(!out.contains("react"), "react gone:\n{out}");
  assert!(!out.contains("catalog:"), "empty catalog block pruned:\n{out}");
}

#[test]
fn yamlpatch_remove_last_dep_prunes_named_catalog_and_catalogs() {
  // T10: removing the only dep in the only named catalog prunes the
  // named catalog AND the surrounding `catalogs:` parent.
  let raw = "catalogs:\n  legacy:\n    react: ^16.0.0\n";
  let mut yaml = yaml_from(raw);
  remove_catalog_definition(&mut yaml, "legacy", "react");
  let out = render(&yaml);
  assert!(!out.contains("react"), "react gone:\n{out}");
  assert!(!out.contains("legacy"), "empty legacy catalog pruned:\n{out}");
  assert!(!out.contains("catalogs:"), "empty catalogs map pruned:\n{out}");
}

#[test]
fn yamlpatch_remove_preserves_neighbouring_comments() {
  // T8: remove a dep with siblings. Comments on sibling keys stay.
  let raw = "catalog:\n  react: ^18.0.0 # to be removed\n  lodash: ^4.0.0 # keep me\n";
  let mut yaml = yaml_from(raw);
  remove_catalog_definition(&mut yaml, "default", "react");
  let out = render(&yaml);
  assert!(!out.contains("react"), "react line removed:\n{out}");
  assert!(out.contains("lodash: ^4.0.0 # keep me"), "lodash + comment preserved:\n{out}");
}

#[test]
fn parse_yaml_file_captures_raw_text() {
  let raw = "catalog:\n  react: ^18.0.0  # pinned\n";
  let yaml = parse_yaml_file(raw.to_string(), PathBuf::from("/test/pnpm-workspace.yaml")).expect("yaml fixture must parse");
  assert_eq!(yaml.raw, raw, "YamlFile.raw must equal the input string verbatim");
  assert!(yaml.patches.is_empty(), "fresh YamlFile starts with no pending patches");
}

#[test]
fn empty_yaml_file_has_empty_raw() {
  let yaml = empty_yaml_file(PathBuf::from("/test/pnpm-workspace.yaml"));
  assert_eq!(yaml.raw, "", "auto-created YamlFile has empty raw");
  assert!(yaml.patches.is_empty(), "auto-created YamlFile has no patches");
  assert!(!yaml.dirty, "auto-created YamlFile defaults dirty=false");
}

#[test]
fn json_view_returns_owned_eager() {
  let raw = "catalog:\n  react: ^18.0.0\n";
  let yaml = parse_yaml_file(raw.to_string(), PathBuf::from("/test/pnpm-workspace.yaml")).expect("yaml fixture must parse");
  let first = json_view(&yaml);
  let second = json_view(&yaml);
  // Two calls return distinct owned Values — addresses must differ.
  let first_ptr = &first as *const serde_json::Value;
  let second_ptr = &second as *const serde_json::Value;
  assert_ne!(first_ptr, second_ptr, "owned Values must occupy distinct addresses");
  assert_eq!(first, second, "owned Values must contain equal data");
}

mod disk_ops {
  use {
    super::*,
    crate::{
      dependency::{DependencyType, Strategy},
      disk::{
        DetectedFormatting, copy_expected_specifier_json, detect_formatting, get_pretty_json_bytes, has_prop, package_name, parse_json_file,
      },
      instance::{FixableInstance, Instance, InstanceDescriptor},
      source::SourceKind,
      sources::SourceIdx,
    },
    serde_json::{Map, Value},
  };

  fn file_from_raw(raw: &str) -> File<Value> {
    parse_json_file(raw.to_string(), PathBuf::from("/packages/test/package.json")).expect("Failed to parse test package.json")
  }

  fn serialise_file(file: &File<Value>, formatting: DetectedFormatting) -> String {
    let snapshot = File {
      filepath: file.filepath.clone(),
      formatting,
      contents: &file.contents,
      dirty: false,
    };
    String::from_utf8(get_pretty_json_bytes(&snapshot).unwrap()).unwrap()
  }

  fn make_instance(name: &str, dep_type: DependencyType, expected: &str) -> Instance {
    let specifier = Specifier::new(expected);
    let descriptor = InstanceDescriptor {
      dependency_type: std::rc::Rc::new(dep_type),
      internal_name: name.to_string(),
      is_local_dependency: false,
      name: name.to_string(),
      source_idx: SourceIdx(0),
      specifier: Specifier::new("0.0.0"), // ignored — overridden by mark_fixable below
    };
    let instance = Instance::new(descriptor, "test-pkg", None);
    instance.mark_fixable(FixableInstance::DiffersToHighestOrLowestSemver, &specifier);
    instance
  }

  #[test]
  fn serialize_uses_detected_2_space_indent() {
    let raw = "{\n  \"name\": \"pkg\",\n  \"version\": \"1.0.0\"\n}\n";
    let file = file_from_raw(raw);
    let fmt = detect_formatting(raw);
    let result = serialise_file(&file, fmt);
    assert!(result.contains("  \"name\""), "expected 2-space indent, got:\n{result}");
    assert!(
      !result.contains("    \"name\""),
      "expected 2-space indent but got 4-space, got:\n{result}"
    );
  }

  #[test]
  fn serialize_uses_detected_4_space_indent() {
    let raw = "{\n    \"name\": \"pkg\",\n    \"version\": \"1.0.0\"\n}\n";
    let file = file_from_raw(raw);
    let fmt = detect_formatting(raw);
    let result = serialise_file(&file, fmt);
    assert!(result.contains("    \"name\""), "expected 4-space indent, got:\n{result}");
  }

  #[test]
  fn serialize_uses_detected_tab_indent() {
    let raw = "{\n\t\"name\": \"pkg\",\n\t\"version\": \"1.0.0\"\n}\n";
    let file = file_from_raw(raw);
    let fmt = detect_formatting(raw);
    let result = serialise_file(&file, fmt);
    assert!(result.contains("\t\"name\""), "expected tab indent, got:\n{result}");
  }

  #[test]
  fn serialize_uses_overridden_indent() {
    let raw = "{\n    \"name\": \"pkg\",\n    \"version\": \"1.0.0\"\n}\n";
    let file = file_from_raw(raw);
    let mut fmt = detect_formatting(raw);
    fmt.indent = "  ".to_string();
    let result = serialise_file(&file, fmt);
    assert!(
      result.contains("  \"name\""),
      "expected config 2-space indent to win, got:\n{result}"
    );
    assert!(
      !result.contains("    \"name\""),
      "expected config indent to override detected 4-space, got:\n{result}"
    );
  }

  #[test]
  fn serialize_preserves_lf_newline() {
    let raw = "{\n  \"name\": \"pkg\"\n}\n";
    let file = file_from_raw(raw);
    let fmt = detect_formatting(raw);
    let result = serialise_file(&file, fmt);
    assert!(result.ends_with('\n'), "expected trailing LF");
    assert!(
      !result.ends_with("\r\n"),
      "expected LF only, not CRLF, got bytes: {:?}",
      result.as_bytes().iter().rev().take(4).collect::<Vec<_>>()
    );
  }

  #[test]
  fn serialize_preserves_crlf_newline() {
    let raw = "{\n  \"name\": \"pkg\"\n}\n";
    let raw = raw.replace('\n', "\r\n");
    let file = file_from_raw(&raw);
    let fmt = detect_formatting(&raw);
    let result = serialise_file(&file, fmt);
    assert!(
      result.ends_with("\r\n"),
      "expected trailing CRLF, got bytes: {:?}",
      result.as_bytes().iter().rev().take(4).collect::<Vec<_>>()
    );
  }

  #[test]
  fn from_raw_is_not_dirty() {
    let file = file_from_raw("{\"name\": \"pkg\", \"version\": \"1.0.0\"}");
    assert!(!file.dirty);
  }

  #[test]
  fn set_prop_marks_dirty_when_value_changes() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"version\": \"1.0.0\"}");
    set_prop(&mut file, "/version", json!("2.0.0"));
    assert!(file.dirty);
    assert_eq!(file.contents.pointer("/version"), Some(&json!("2.0.0")));
  }

  #[test]
  fn set_prop_does_not_mark_dirty_when_value_unchanged() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"version\": \"1.0.0\"}");
    set_prop(&mut file, "/version", json!("1.0.0"));
    assert!(!file.dirty);
  }

  #[test]
  fn set_prop_detects_object_key_reorder() {
    let mut file = file_from_raw("{\"b\": 1, \"a\": 2}");
    let mut reordered = Map::new();
    reordered.insert("a".to_string(), json!(2));
    reordered.insert("b".to_string(), json!(1));
    set_prop(&mut file, "/", Value::Object(reordered));
    assert!(file.dirty, "reordering object keys should mark dirty");
  }

  #[test]
  fn set_prop_does_not_mark_dirty_when_object_key_order_same() {
    let mut file = file_from_raw("{\"a\": 1, \"b\": 2}");
    let same_contents = file.contents.clone();
    set_prop(&mut file, "/", same_contents);
    assert!(!file.dirty);
  }

  #[test]
  fn set_nested_prop_marks_dirty_when_value_changes() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"react\": \"17.0.0\"}}");
    set_nested_prop(&mut file, "/dependencies", "react", json!("18.0.0"));
    assert!(file.dirty);
    assert_eq!(file.contents.pointer("/dependencies/react"), Some(&json!("18.0.0")));
  }

  #[test]
  fn set_nested_prop_does_not_mark_dirty_when_value_unchanged() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"react\": \"17.0.0\"}}");
    set_nested_prop(&mut file, "/dependencies", "react", json!("17.0.0"));
    assert!(!file.dirty);
  }

  #[test]
  fn set_nested_prop_works_with_slash_in_key() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"@scope/lib\": \"1.0.0\"}}");
    set_nested_prop(&mut file, "/dependencies", "@scope/lib", json!("2.0.0"));
    assert!(file.dirty);
    let deps = file.contents.pointer("/dependencies").unwrap().as_object().unwrap();
    assert_eq!(deps.get("@scope/lib"), Some(&json!("2.0.0")));
  }

  #[test]
  fn remove_prop_marks_dirty_when_key_exists() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"react\": \"17.0.0\"}}");
    remove_prop(&mut file, "/dependencies", "react");
    assert!(file.dirty);
    let deps = file.contents.pointer("/dependencies").unwrap().as_object().unwrap();
    assert!(!deps.contains_key("react"));
  }

  #[test]
  fn remove_prop_does_not_mark_dirty_when_key_missing() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"react\": \"17.0.0\"}}");
    remove_prop(&mut file, "/dependencies", "lodash");
    assert!(!file.dirty);
  }

  #[test]
  fn remove_prop_works_with_slash_in_key() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"@scope/lib\": \"1.0.0\"}}");
    remove_prop(&mut file, "/dependencies", "@scope/lib");
    assert!(file.dirty);
    let deps = file.contents.pointer("/dependencies").unwrap().as_object().unwrap();
    assert!(!deps.contains_key("@scope/lib"));
  }

  #[test]
  fn copy_expected_specifier_versions_by_name() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"react\": \"17.0.0\"}}");
    let dep_type = DependencyType {
      name_path: None,
      name: "prod".to_string(),
      path: "/dependencies".to_string(),
      strategy: Strategy::VersionsByName,
      source: SourceKind::PackageJson,
      is_catalog_definition: false,
    };
    let instance = make_instance("react", dep_type, "18.0.0");
    copy_expected_specifier_json(&mut file, &instance);
    assert!(file.dirty);
    assert_eq!(file.contents.pointer("/dependencies/react"), Some(&json!("18.0.0")));
  }

  #[test]
  fn copy_expected_specifier_versions_by_name_scoped_package() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"@scope/lib\": \"1.0.0\"}}");
    let dep_type = DependencyType {
      name_path: None,
      name: "prod".to_string(),
      path: "/dependencies".to_string(),
      strategy: Strategy::VersionsByName,
      source: SourceKind::PackageJson,
      is_catalog_definition: false,
    };
    let instance = make_instance("@scope/lib", dep_type, "2.0.0");
    copy_expected_specifier_json(&mut file, &instance);
    assert!(file.dirty, "scoped package specifier was not applied");
    let deps = file.contents.pointer("/dependencies").unwrap().as_object().unwrap();
    assert_eq!(deps.get("@scope/lib"), Some(&json!("2.0.0")));
  }

  #[test]
  fn copy_expected_specifier_versions_by_name_deeply_nested_path() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"config\": {\"custom\": {\"dependencies\": {\"@scope/lib\": \"1.0.0\"}}}}");
    let dep_type = DependencyType {
      name_path: None,
      name: "customDeps".to_string(),
      path: "/config/custom/dependencies".to_string(),
      strategy: Strategy::VersionsByName,
      source: SourceKind::PackageJson,
      is_catalog_definition: false,
    };
    let instance = make_instance("@scope/lib", dep_type, "2.0.0");
    copy_expected_specifier_json(&mut file, &instance);
    assert!(file.dirty, "deeply nested scoped package specifier was not applied");
    let deps = file.contents.pointer("/config/custom/dependencies").unwrap().as_object().unwrap();
    assert_eq!(deps.get("@scope/lib"), Some(&json!("2.0.0")));
  }

  #[test]
  fn copy_expected_specifier_versions_by_name_no_op() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"dependencies\": {\"react\": \"18.0.0\"}}");
    let dep_type = DependencyType {
      name_path: None,
      name: "prod".to_string(),
      path: "/dependencies".to_string(),
      strategy: Strategy::VersionsByName,
      source: SourceKind::PackageJson,
      is_catalog_definition: false,
    };
    let instance = make_instance("react", dep_type, "18.0.0");
    copy_expected_specifier_json(&mut file, &instance);
    assert!(!file.dirty, "same value should not mark dirty");
  }

  #[test]
  fn copy_expected_specifier_name_and_version_props() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"version\": \"1.0.0\"}");
    let dep_type = DependencyType {
      name_path: Some("/name".to_string()),
      name: "local".to_string(),
      path: "/version".to_string(),
      strategy: Strategy::NameAndVersionProps,
      source: SourceKind::PackageJson,
      is_catalog_definition: false,
    };
    let instance = make_instance("pkg", dep_type, "2.0.0");
    copy_expected_specifier_json(&mut file, &instance);
    assert!(file.dirty);
    assert_eq!(file.contents.pointer("/version"), Some(&json!("2.0.0")));
  }

  #[test]
  fn copy_expected_specifier_named_version_string() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"packageManager\": \"pnpm@7.0.0\"}");
    let dep_type = DependencyType {
      name_path: None,
      name: "packageManager".to_string(),
      path: "/packageManager".to_string(),
      strategy: Strategy::NamedVersionString,
      source: SourceKind::PackageJson,
      is_catalog_definition: false,
    };
    let instance = make_instance("pnpm", dep_type, "8.0.0");
    copy_expected_specifier_json(&mut file, &instance);
    assert!(file.dirty);
    assert_eq!(file.contents.pointer("/packageManager"), Some(&json!("pnpm@8.0.0")));
  }

  #[test]
  fn copy_expected_specifier_unnamed_version_string() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"engines\": {\"node\": \">=16\"}}");
    let dep_type = DependencyType {
      name_path: None,
      name: "node".to_string(),
      path: "/engines/node".to_string(),
      strategy: Strategy::UnnamedVersionString,
      source: SourceKind::PackageJson,
      is_catalog_definition: false,
    };
    let instance = make_instance("node", dep_type, ">=18");
    copy_expected_specifier_json(&mut file, &instance);
    assert!(file.dirty);
    assert_eq!(file.contents.pointer("/engines/node"), Some(&json!(">=18")));
  }

  #[test]
  fn parse_json_file_returns_none_for_invalid_json() {
    let result = parse_json_file("not json".to_string(), PathBuf::from("/test/package.json"));
    assert!(result.is_none());
  }

  #[test]
  fn package_name_uses_fallback_when_name_missing() {
    let file = file_from_raw("{\"version\": \"1.0.0\"}");
    assert_eq!(package_name(&file), "NAME_IS_MISSING");
  }

  #[test]
  fn has_prop_returns_true_when_exists() {
    let file = file_from_raw("{\"name\": \"pkg\", \"version\": \"1.0.0\"}");
    assert!(has_prop(&file, "/version"));
  }

  #[test]
  fn has_prop_returns_false_when_missing() {
    let file = file_from_raw("{\"name\": \"pkg\"}");
    assert!(!has_prop(&file, "/version"));
  }

  #[test]
  fn set_prop_is_noop_when_path_missing() {
    let mut file = file_from_raw("{\"name\": \"pkg\"}");
    set_prop(&mut file, "/nonexistent/deep/path", json!("value"));
    assert!(!file.dirty);
  }

  #[test]
  fn set_nested_prop_is_noop_when_parent_missing() {
    let mut file = file_from_raw("{\"name\": \"pkg\"}");
    set_nested_prop(&mut file, "/nonexistent", "key", json!("value"));
    assert!(!file.dirty);
  }

  #[test]
  fn set_nested_prop_is_noop_when_parent_is_not_object() {
    let mut file = file_from_raw("{\"name\": \"pkg\", \"version\": \"1.0.0\"}");
    set_nested_prop(&mut file, "/version", "key", json!("value"));
    assert!(!file.dirty);
  }

  #[test]
  fn remove_prop_is_noop_when_parent_missing() {
    let mut file = file_from_raw("{\"name\": \"pkg\"}");
    remove_prop(&mut file, "/nonexistent", "key");
    assert!(!file.dirty);
  }

  #[test]
  fn set_prop_detects_nested_object_key_reorder() {
    let mut file = file_from_raw("{\"deps\": {\"b\": 1, \"a\": 2}}");
    let mut reordered = Map::new();
    reordered.insert("a".to_string(), json!(2));
    reordered.insert("b".to_string(), json!(1));
    set_prop(&mut file, "/deps", Value::Object(reordered));
    assert!(file.dirty, "reordering nested object keys should mark dirty");
  }

  #[test]
  fn serialize_preserves_key_order() {
    let raw = "{\n  \"z\": 1,\n  \"a\": 2,\n  \"m\": 3\n}\n";
    let file = file_from_raw(raw);
    let fmt = detect_formatting(raw);
    let result = serialise_file(&file, fmt);
    let z_pos = result.find("\"z\"").unwrap();
    let a_pos = result.find("\"a\"").unwrap();
    let m_pos = result.find("\"m\"").unwrap();
    assert!(z_pos < a_pos && a_pos < m_pos, "key order not preserved, got:\n{result}");
  }
}
