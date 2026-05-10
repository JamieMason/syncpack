use {
  crate::{
    commands::{fix, reporter::FixReporter},
    context::Context,
    errors::SyncpackError,
    instance::Instance,
    test::{builder::TestBuilder, mock_disk::MockDiskIo},
    version_group::{DependencyCore, VersionGroup},
  },
  serde_json::json,
};

/// Silent reporter — fix tests assert state, not console output.
struct SilentReporter;
impl FixReporter for SilentReporter {
  fn on_group_header(&self, _: &Context, _: &VersionGroup) {}

  fn on_dependency(&self, _: &Context, _: &DependencyCore, _: &str) {}

  fn on_instance(&self, _: &Context, _: &Instance, _: &str) {}

  fn on_no_issues(&self) {}

  fn on_unfixable_warning(&self) {}
}

/// Runs `fix::run`. On `Ok`, returns the post-fix `Context`. On
/// `Err(IssuesFound)`, the ctx was consumed; tests that need to inspect it
/// after an unfixable issue should structure their setup so all issues are
/// fixable.
fn run_fix_ok(ctx: Context) -> Context {
  let disk = MockDiskIo::new();
  fix::run(ctx, &SilentReporter, &disk).expect("fix should succeed")
}

/// Find a consumer instance by id. Convenience used across the tests.
fn find_instance<'a>(ctx: &'a Context, id: &str) -> &'a Instance {
  ctx
    .instances
    .iter()
    .find(|i| i.id == id)
    .unwrap_or_else(|| panic!("instance not found: {id}"))
}

/// Find the package.json by name from disk.
fn find_package<'a>(ctx: &'a Context, name: &str) -> &'a crate::disk::File<serde_json::Value> {
  ctx
    .disk
    .package_json_files
    .iter()
    .find(|f| crate::disk::package_name(f) == name)
    .unwrap_or_else(|| panic!("package not found: {name}"))
}

/// Locate the loaded yaml from disk. Tests that asserted on the v2
/// `ctx.catalogs` field now reach in here.
fn pnpm_yaml(ctx: &Context) -> Option<&crate::disk::YamlFile> {
  ctx.disk.pnpm_workspace.as_ref()
}

#[test]
fn pnpm_fix_not_using_catalog() {
  // Catalog has react. Sibling has real specifier. Fix → consumer specifier
  // becomes "catalog:". Yaml is unchanged because the catalog already has
  // the dep.
  let yaml = "catalog:\n  react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  let ctx = run_fix_ok(ctx);

  let pkg = find_package(&ctx, "pkg-a");
  assert_eq!(
    pkg.contents.pointer("/dependencies/react").and_then(|v| v.as_str()),
    Some("catalog:"),
    "consumer should now use catalog:"
  );
  assert!(pkg.is_dirty(), "consumer pkg should be marked dirty");
  assert!(
    !pnpm_yaml(&ctx).unwrap().is_dirty(),
    "yaml should NOT be dirty when only NotUsingCatalog fixes ran"
  );
}

#[test]
fn pnpm_fix_missing_from_catalog_inserts_definition() {
  // Catalog exists but lacks `react`. Consumer has real specifier. Fix
  // inserts `react → ^18.0.0` into yaml AND writes `catalog:` into consumer.
  let yaml = "catalog:\n  lodash: ^4.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  let ctx = run_fix_ok(ctx);

  let yaml = pnpm_yaml(&ctx).unwrap();
  assert!(yaml.is_dirty(), "yaml should be dirty after insert");
  let inserted = yaml
    .contents
    .get("catalog")
    .and_then(|m| m.get("react"))
    .and_then(|v| v.as_str())
    .map(str::to_string);
  assert_eq!(inserted, Some("^18.0.0".to_string()), "yaml should now contain react");

  let pkg = find_package(&ctx, "pkg-a");
  assert_eq!(
    pkg.contents.pointer("/dependencies/react").and_then(|v| v.as_str()),
    Some("catalog:"),
    "consumer should now use catalog:"
  );
}

#[test]
fn pnpm_fix_missing_from_named_catalog() {
  let yaml = "catalogs:\n  react18:\n    lodash: ^4.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  let ctx = run_fix_ok(ctx);

  let yaml = pnpm_yaml(&ctx).unwrap();
  let inserted = yaml
    .contents
    .get("catalogs")
    .and_then(|m| m.get("react18"))
    .and_then(|m| m.get("react"))
    .and_then(|v| v.as_str())
    .map(str::to_string);
  assert_eq!(inserted, Some("^18.0.0".to_string()));

  let pkg = find_package(&ctx, "pkg-a");
  assert_eq!(
    pkg.contents.pointer("/dependencies/react").and_then(|v| v.as_str()),
    Some("catalog:react18"),
    "consumer should reference the named catalog"
  );
}

#[test]
fn pnpm_fix_missing_from_catalog_idempotent_inserts() {
  // Two consumers of the same dep. Yaml insert should run once, not twice.
  let yaml = "catalog:\n  lodash: ^4.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![
      json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": "^18.0.0"}}),
      json!({"name": "pkg-b", "version": "0.0.0", "dependencies": {"react": "^18.0.0"}}),
    ])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  let ctx = run_fix_ok(ctx);

  let yaml = pnpm_yaml(&ctx).unwrap();
  let map = yaml.contents.get("catalog").and_then(|v| v.as_mapping()).unwrap();
  assert_eq!(map.len(), 2, "yaml catalog should have lodash + react only");
  assert_eq!(map.get("react").and_then(|v| v.as_str()), Some("^18.0.0"));
  assert!(yaml.is_dirty(), "yaml should be dirty");

  for name in ["pkg-a", "pkg-b"] {
    let pkg = find_package(&ctx, name);
    assert_eq!(
      pkg.contents.pointer("/dependencies/react").and_then(|v| v.as_str()),
      Some("catalog:"),
      "{name} should now use catalog:"
    );
  }
}

#[test]
fn pnpm_fix_writes_yaml_to_disk() {
  // dry_run = false → write loop runs. Use a recording MockDiskIo to confirm
  // the yaml file gets written.
  let yaml = "catalog:\n  lodash: ^4.0.0\n";
  let mut ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  ctx.config.cli.dry_run = false;

  let disk = recording_disk();
  let _ = fix::run(ctx, &SilentReporter, &disk).expect("fix should succeed");

  let writes = disk.recorded_writes();
  assert!(
    writes.iter().any(|p| p.ends_with("pnpm-workspace.yaml")),
    "expected a write to pnpm-workspace.yaml; got {writes:?}"
  );
}

#[test]
fn pnpm_fix_does_not_touch_yaml_when_only_package_json_changes() {
  // Only `NotUsingCatalog` fixes — no inserts into yaml.
  let yaml = "catalog:\n  react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  let ctx = run_fix_ok(ctx);

  assert!(
    !pnpm_yaml(&ctx).unwrap().is_dirty(),
    "yaml should be untouched when only NotUsingCatalog fixes ran"
  );
}

#[test]
fn pnpm_fix_in_memory_state_visible_with_dry_run() {
  // Documents the test infra contract: `dry_run = true` preserves
  // `is_dirty()` and contents for inspection.
  let yaml = "catalog:\n  lodash: ^4.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  assert!(ctx.config.cli.dry_run, "test mock CLI should default to dry_run = true");
  let ctx = run_fix_ok(ctx);

  // Both mutations visible because write_to_disk was skipped.
  assert!(pnpm_yaml(&ctx).unwrap().is_dirty(), "yaml dirty preserved");
  assert!(find_package(&ctx, "pkg-a").is_dirty(), "package dirty preserved");
}

#[test]
fn bun_fix_not_using_catalog() {
  // Bun catalog already has react. Consumer has real specifier. Fix →
  // consumer becomes catalog:. Bun catalog file (root pkg) unchanged.
  let ctx = TestBuilder::new()
    .with_bun_catalogs(json!({
      "name": "bun-root",
      "catalog": {"react": "^18.0.0"},
    }))
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  let ctx = run_fix_ok(ctx);

  let pkg = find_package(&ctx, "pkg-a");
  assert_eq!(
    pkg.contents.pointer("/dependencies/react").and_then(|v| v.as_str()),
    Some("catalog:"),
    "consumer should now use catalog:"
  );

  // Bun root unchanged (def already had `^18.0.0`).
  let root = find_package(&ctx, "bun-root");
  assert_eq!(root.contents.pointer("/catalog/react").and_then(|v| v.as_str()), Some("^18.0.0"),);
}

#[test]
fn bun_fix_missing_from_catalog_top_level() {
  // Bun catalog (top-level) lacks `react`. Fix should insert into
  // `/catalog` AND write `catalog:` to consumer.
  let ctx = TestBuilder::new()
    .with_bun_catalogs(json!({
      "name": "bun-root",
      "catalog": {"lodash": "^4.0.0"},
    }))
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  let ctx = run_fix_ok(ctx);

  let root = find_package(&ctx, "bun-root");
  assert_eq!(
    root.contents.pointer("/catalog/react").and_then(|v| v.as_str()),
    Some("^18.0.0"),
    "react should have been inserted into bun /catalog",
  );

  let pkg = find_package(&ctx, "pkg-a");
  assert_eq!(
    pkg.contents.pointer("/dependencies/react").and_then(|v| v.as_str()),
    Some("catalog:"),
  );
}

#[test]
fn bun_fix_missing_from_catalog_workspaces() {
  // Bun catalog at `/workspaces/catalog`.
  let ctx = TestBuilder::new()
    .with_bun_workspaces_catalogs(json!({
      "catalog": {"lodash": "^4.0.0"},
    }))
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  let ctx = run_fix_ok(ctx);

  let root = find_package(&ctx, "bun-root");
  assert_eq!(
    root.contents.pointer("/workspaces/catalog/react").and_then(|v| v.as_str()),
    Some("^18.0.0"),
    "react should have been inserted into /workspaces/catalog",
  );
}

#[test]
fn bun_fix_missing_from_named_catalog() {
  let ctx = TestBuilder::new()
    .with_bun_catalogs(json!({
      "name": "bun-root",
      "catalogs": {"react18": {"lodash": "^4.0.0"}},
    }))
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  let ctx = run_fix_ok(ctx);

  let root = find_package(&ctx, "bun-root");
  assert_eq!(
    root.contents.pointer("/catalogs/react18/react").and_then(|v| v.as_str()),
    Some("^18.0.0"),
  );

  let pkg = find_package(&ctx, "pkg-a");
  assert_eq!(
    pkg.contents.pointer("/dependencies/react").and_then(|v| v.as_str()),
    Some("catalog:react18"),
  );
}

#[test]
fn bun_fix_insert_idempotent() {
  let ctx = TestBuilder::new()
    .with_bun_catalogs(json!({
      "name": "bun-root",
      "catalog": {"lodash": "^4.0.0"},
    }))
    .with_packages(vec![
      json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": "^18.0.0"}}),
      json!({"name": "pkg-b", "version": "0.0.0", "dependencies": {"react": "^18.0.0"}}),
    ])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  let ctx = run_fix_ok(ctx);

  let root = find_package(&ctx, "bun-root");
  let catalog_obj = root.contents.pointer("/catalog").and_then(|v| v.as_object()).unwrap();
  assert_eq!(catalog_obj.len(), 2, "catalog should hold lodash + react only");
  assert_eq!(catalog_obj.get("react").and_then(|v| v.as_str()), Some("^18.0.0"));
}

#[test]
fn bun_fix_picks_root_via_fallback_chain() {
  // No existing BunCatalog instance (catalog is empty so no instances), no
  // cwd match — fallback resolves via the `workspaces` field.
  let ctx = TestBuilder::new()
    .with_bun_workspaces_catalogs(json!({
      "catalog": {"lodash": "^4.0.0"},
    }))
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  let ctx = run_fix_ok(ctx);

  // Insert landed under /workspaces/catalog (not at top-level).
  let root = find_package(&ctx, "bun-root");
  assert_eq!(
    root.contents.pointer("/workspaces/catalog/react").and_then(|v| v.as_str()),
    Some("^18.0.0"),
  );
  assert!(
    root.contents.pointer("/catalog/react").is_none(),
    "should NOT have created /catalog when /workspaces/catalog already existed"
  );
}

#[test]
fn fix_creates_default_catalog_when_zero_catalogs_pnpm() {
  // No pnpm-workspace.yaml present, PM=Pnpm. Fix should auto-create the
  // wrapper at `<cwd>/pnpm-workspace.yaml` and insert the dep.
  let ctx = TestBuilder::new()
    .with_pnpm_package_manager()
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  assert!(pnpm_yaml(&ctx).is_none(), "precondition: no catalog wrapper before fix");
  let ctx = run_fix_ok(ctx);

  let yaml = pnpm_yaml(&ctx).expect("yaml wrapper auto-created by fix");
  assert!(yaml.filepath.ends_with("pnpm-workspace.yaml"));
  let inserted = yaml
    .contents
    .get("catalog")
    .and_then(|m| m.get("react"))
    .and_then(|v| v.as_str())
    .map(str::to_string);
  assert_eq!(inserted, Some("^18.0.0".to_string()));

  let pkg = find_package(&ctx, "pkg-a");
  assert_eq!(
    pkg.contents.pointer("/dependencies/react").and_then(|v| v.as_str()),
    Some("catalog:"),
  );
}

#[test]
fn fix_creates_default_catalog_when_zero_catalogs_bun_no_workspaces() {
  // No catalogs in root pkg, no `workspaces` key, PM=Bun → write top-level
  // `/catalog` on the resolved root pkg.
  let ctx = TestBuilder::new()
    .with_bun_catalogs(json!({"name": "bun-root"}))
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  let ctx = run_fix_ok(ctx);

  let root = find_package(&ctx, "bun-root");
  assert_eq!(
    root.contents.pointer("/catalog/react").and_then(|v| v.as_str()),
    Some("^18.0.0"),
    "react should have been inserted into top-level /catalog",
  );
  let pkg = find_package(&ctx, "pkg-a");
  assert_eq!(
    pkg.contents.pointer("/dependencies/react").and_then(|v| v.as_str()),
    Some("catalog:"),
  );
}

#[test]
fn fix_creates_default_catalog_when_zero_catalogs_bun_with_workspaces() {
  // Bun root pkg has empty `workspaces` (no catalog content anywhere) →
  // discovery returns None and the auto-create path falls back to top-level
  // `/catalog`. The single `detect_bun_catalogs` helper is used by both
  // discovery and fix-time so the locations agree.
  let ctx = TestBuilder::new()
    .with_bun_catalogs(json!({"name": "bun-root", "workspaces": {}}))
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  let ctx = run_fix_ok(ctx);

  let root = find_package(&ctx, "bun-root");
  assert_eq!(
    root.contents.pointer("/catalog/react").and_then(|v| v.as_str()),
    Some("^18.0.0"),
    "react should have been inserted into top-level /catalog (no catalogs anywhere → top-level fallback)",
  );
  assert!(
    root.contents.pointer("/workspaces/catalog").is_none(),
    "should NOT have created /workspaces/catalog when no Bun catalogs existed"
  );
}

#[test]
fn cannot_infer_catalog_file_when_zero_catalogs_npm_yarn_or_unknown() {
  // PM=npm/yarn/unknown → state is `Unfixable::CannotInferCatalogFile`.
  // `fix::run` skips Unfixable issues; nothing should be written.
  let ctx = TestBuilder::new()
    .with_npm_package_manager()
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  // Verify the unfixable state survives a fix run (no auto-create).
  let consumer = find_instance(&ctx, "react in /dependencies of pkg-a");
  assert_eq!(
    consumer.state.borrow().get_status_type(),
    "Unfixable",
    "consumer should already be Unfixable::CannotInferCatalogFile",
  );

  // fix::run returns Err(IssuesFound) for unfixable states. Verify no catalog
  // file was created on disk (we can't inspect ctx after Err — it's consumed).
  let disk = MockDiskIo::new();
  let result = fix::run(ctx, &SilentReporter, &disk);
  assert!(
    matches!(result, Err(SyncpackError::IssuesFound)),
    "expected IssuesFound, got {result:?}"
  );
}

#[test]
fn fix_bun_catalog_routes_to_root_idx_no_fallback() {
  // Bun fix-time uses ctx.disk.package_json_root_idx directly. The synthetic
  // root is at disk.package_json_root_idx, so insert lands there.
  let ctx = TestBuilder::new()
    .with_bun_catalogs(json!({"name": "bun-root"}))
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  // Precondition: root_idx is set on disk.
  let root_idx = ctx
    .disk
    .package_json_root_idx
    .expect("synthetic Bun root must populate disk.package_json_root_idx");
  let ctx = run_fix_ok(ctx);
  // The insert lands on the file at the disk's root_idx slot.
  let root_file = &ctx.disk.package_json_files[root_idx];
  assert_eq!(
    root_file.contents.pointer("/catalog/react").and_then(|v| v.as_str()),
    Some("^18.0.0"),
    "Bun fix path must write through disk.package_json_root_idx"
  );
}

#[test]
fn fix_pnpm_catalog_creates_yaml_when_absent() {
  // 0-catalogs auto-create writes to disk.pnpm_workspace — NOT to a sources
  // arena entry.
  let ctx = TestBuilder::new()
    .with_pnpm_package_manager()
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^18.0.0"},
    })])
    .with_version_group(json!({
      "label": "enforce catalog",
      "dependencies": ["react"],
      "policy": "catalog",
    }))
    .build_and_visit_packages();
  // Precondition: no yaml on disk.
  assert!(ctx.disk.pnpm_workspace.is_none(), "no yaml on disk pre-fix");
  let ctx = run_fix_ok(ctx);
  // The auto-created yaml lives on disk; sources arena does NOT gain an entry.
  let yaml = ctx
    .disk
    .pnpm_workspace
    .as_ref()
    .expect("auto-created yaml must live on disk.pnpm_workspace");
  assert!(yaml.is_dirty(), "auto-created yaml should be dirty after insert");
  let inserted = yaml.contents.get("catalog").and_then(|m| m.get("react")).and_then(|v| v.as_str());
  assert_eq!(inserted, Some("^18.0.0"), "react inserted into auto-created yaml");
}

use std::{
  cell::RefCell,
  path::{Path, PathBuf},
};

/// MockDiskIo wrapper that records `write_*_file` paths. Used by the
/// `pnpm_fix_writes_yaml_to_disk` test to verify the disk write loop fires.
struct RecordingDiskIo {
  inner: MockDiskIo,
  writes: RefCell<Vec<PathBuf>>,
}

impl RecordingDiskIo {
  fn recorded_writes(&self) -> Vec<PathBuf> {
    self.writes.borrow().clone()
  }
}

fn recording_disk() -> RecordingDiskIo {
  RecordingDiskIo {
    inner: MockDiskIo::new(),
    writes: RefCell::new(vec![]),
  }
}

impl crate::disk::DiskIo for RecordingDiskIo {
  fn exec_node_command(&self, cwd: &Path, args: &[&str]) -> Result<String, crate::disk::NodeJsError> {
    self.inner.exec_node_command(cwd, args)
  }

  fn path_exists(&self, p: &Path) -> bool {
    self.inner.path_exists(p)
  }

  fn read_dir(&self, p: &Path) -> Result<Vec<crate::disk::DiskDirEntry>, std::io::Error> {
    self.inner.read_dir(p)
  }

  fn read_json_file<V: serde::de::DeserializeOwned>(&self, p: &Path) -> Option<Result<crate::disk::File<V>, crate::disk::DiskIoError>> {
    self.inner.read_json_file(p)
  }

  fn read_bytes(&self, p: &Path) -> Option<Result<Vec<u8>, crate::disk::DiskIoError>> {
    self.inner.read_bytes(p)
  }

  fn read_textfile(&self, p: &Path) -> Option<Result<crate::disk::File<String>, crate::disk::DiskIoError>> {
    self.inner.read_textfile(p)
  }

  fn read_yaml_file(&self, p: &Path) -> Option<Result<crate::disk::YamlFile, crate::disk::DiskIoError>> {
    self.inner.read_yaml_file(p)
  }

  fn read_yaml_typed<V: serde::de::DeserializeOwned>(&self, p: &Path) -> Option<Result<crate::disk::File<V>, crate::disk::DiskIoError>> {
    self.inner.read_yaml_typed(p)
  }

  fn write_bytes(&self, filepath: &Path, _bytes: &[u8]) -> Result<(), crate::disk::DiskIoError> {
    self.writes.borrow_mut().push(filepath.to_path_buf());
    Ok(())
  }

  fn write_json_file<V: serde::ser::Serialize>(&self, file: &crate::disk::File<V>) -> Result<(), crate::disk::DiskIoError> {
    self.writes.borrow_mut().push(file.filepath.clone());
    Ok(())
  }

  fn write_yaml_file(&self, file: &crate::disk::YamlFile) -> Result<(), crate::disk::DiskIoError> {
    self.writes.borrow_mut().push(file.filepath.clone());
    Ok(())
  }
}

/// RED: Banned pnpm catalog defs are correctly marked `IsBanned` by visit, but
/// `apply_fix_actions` skips the `PnpmYaml` branch (`if let Some(fi) =
/// consumer_file_idx` returns None for yaml-sourced instances). Result: fix
/// reports success but the yaml is unchanged. After the fix, the yaml should
/// have the banned dep removed via `disk::remove_catalog_definition`.
#[test]
fn pnpm_fix_banned_removes_from_yaml() {
  let yaml = "catalog:\n  foo: ^1.0.0\n  bar: ^2.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .with_version_group(json!({
      "label": "ban foo",
      "dependencies": ["foo"],
      "dependencyTypes": ["pnpmCatalog"],
      "isBanned": true,
    }))
    .build_and_visit_packages();
  let ctx = run_fix_ok(ctx);

  let yaml = pnpm_yaml(&ctx).unwrap();
  let catalog_block = yaml.contents.get("catalog").and_then(|v| v.as_mapping()).unwrap();
  assert!(
    catalog_block.get("foo").is_none(),
    "banned `foo` should have been removed from yaml /catalog; got {catalog_block:?}"
  );
  assert_eq!(
    catalog_block.get("bar").and_then(|v| v.as_str()),
    Some("^2.0.0"),
    "non-banned `bar` should still be present"
  );
  assert!(yaml.is_dirty(), "yaml should be marked dirty after banned removal");
}

/// Bun parity for the banned-catalog removal. Bun catalog instances are sourced
/// from `Source::Package` (the synthetic root package.json), so the existing
/// `remove_instance_from_disk` path with `Strategy::VersionsByName` should
/// already remove them. This test pins that behaviour.
#[test]
fn bun_fix_banned_removes_from_pkg_json() {
  let ctx = TestBuilder::new()
    .with_bun_catalogs(json!({
      "name": "bun-root",
      "catalog": {"foo": "^1.0.0", "bar": "^2.0.0"}
    }))
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .with_version_group(json!({
      "label": "ban foo",
      "dependencies": ["foo"],
      "dependencyTypes": ["bunCatalog"],
      "isBanned": true,
    }))
    .build_and_visit_packages();
  let ctx = run_fix_ok(ctx);

  let root = find_package(&ctx, "bun-root");
  assert!(
    root.contents.pointer("/catalog/foo").is_none(),
    "banned `foo` should have been removed from bun root /catalog"
  );
  assert_eq!(
    root.contents.pointer("/catalog/bar").and_then(|v| v.as_str()),
    Some("^2.0.0"),
    "non-banned `bar` should still be present"
  );
  assert!(root.is_dirty(), "bun root should be marked dirty after banned removal");
}
