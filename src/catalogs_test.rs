use {
  crate::{
    catalogs::make_catalog_dep_types,
    disk::{Disk, PackageManager, parse_json_file, parse_yaml_file},
    errors::SyncpackError,
    instance::{InstanceState, ValidInstance::*},
    source::SourceKind,
    test::{
      builder::TestBuilder,
      expect::{ExpectedInstance, expect},
    },
  },
  serde_json::json,
  std::path::PathBuf,
};

/// Build an empty Disk with a synthetic cwd (`/test`) for catalogs unit tests.
fn empty_disk() -> Disk {
  Disk {
    cwd: PathBuf::from("/test"),
    lerna_json: None,
    package_json_files: Vec::new(),
    package_json_root_idx: None,
    package_manager: None,
    pnpm_workspace: None,
  }
}

#[test]
fn make_catalog_dep_types_returns_pnpm_dep_types_for_yaml() {
  let mut disk = empty_disk();
  disk.package_manager = Some(PackageManager::Pnpm);
  let yaml = "catalog:\n  react: ^18.0.0\ncatalogs:\n  legacy:\n    react: ^17.0.0\n";
  disk.pnpm_workspace = parse_yaml_file(yaml.to_string(), PathBuf::from("/test/pnpm-workspace.yaml"));

  let dep_types = make_catalog_dep_types(&disk).expect("pnpm catalog discovery must not error");
  let names: Vec<&str> = dep_types.iter().map(|dt| dt.name.as_str()).collect();
  assert!(names.contains(&"pnpmCatalog"), "default pnpm catalog dep type missing");
  assert!(names.contains(&"pnpmCatalog:legacy"), "named pnpm catalog dep type missing");
  for dt in &dep_types {
    assert_eq!(dt.source, SourceKind::PnpmWorkspace);
    assert!(dt.is_catalog_definition);
  }
}

#[test]
fn make_catalog_dep_types_returns_bun_dep_types_for_root() {
  let mut disk = empty_disk();
  disk.package_manager = Some(PackageManager::Bun);
  let raw = serde_json::to_string(&json!({
    "name": "bun-root",
    "catalog": {"react": "^18.0.0"},
    "catalogs": {"legacy": {"react": "^17.0.0"}}
  }))
  .unwrap();
  let root = parse_json_file(raw, PathBuf::from("/bun-root/package.json")).unwrap();
  disk.package_json_files.push(root);
  disk.package_json_root_idx = Some(0);

  let dep_types = make_catalog_dep_types(&disk).expect("bun catalog discovery must not error");
  let names: Vec<&str> = dep_types.iter().map(|dt| dt.name.as_str()).collect();
  assert!(names.contains(&"bunCatalog"), "default bun catalog dep type missing");
  assert!(names.contains(&"bunCatalog:legacy"), "named bun catalog dep type missing");
  for dt in &dep_types {
    assert_eq!(dt.source, SourceKind::PackageJson);
    assert!(dt.is_catalog_definition);
    assert!(dt.path.starts_with("/catalog"), "top-level prefix expected: {}", dt.path);
  }
}

#[test]
fn make_catalog_dep_types_returns_empty_for_no_catalogs() {
  let disk = empty_disk();
  let dep_types = make_catalog_dep_types(&disk).expect("no catalogs must not error");
  assert!(dep_types.is_empty(), "no PM + no files → empty dep types");
}

#[test]
fn make_catalog_dep_types_errors_on_bun_dual_path() {
  let mut disk = empty_disk();
  disk.package_manager = Some(PackageManager::Bun);
  let raw = serde_json::to_string(&json!({
    "name": "bun-root",
    "catalog": {"react": "^18.0.0"},
    "workspaces": {"catalog": {"react": "^18.0.0"}}
  }))
  .unwrap();
  let root = parse_json_file(raw, PathBuf::from("/bun-root/package.json")).unwrap();
  disk.package_json_files.push(root);
  disk.package_json_root_idx = Some(0);

  let err = make_catalog_dep_types(&disk).unwrap_err();
  assert!(matches!(err, SyncpackError::BunDualCatalogPath));
}

#[test]
fn make_catalog_dep_types_picks_top_when_workspaces_present_but_empty() {
  // Root with `{"workspaces": {"packages": [...]}, "catalog": {...}}`
  // (workspaces is Object but holds no catalog block) discovers the top-level
  // `/catalog` dep types, NOT empty.
  let mut disk = empty_disk();
  disk.package_manager = Some(PackageManager::Bun);
  let raw = serde_json::to_string(&json!({
    "name": "bun-root",
    "workspaces": {"packages": ["packages/*"]},
    "catalog": {"react": "^18.0.0"}
  }))
  .unwrap();
  let root = parse_json_file(raw, PathBuf::from("/bun-root/package.json")).unwrap();
  disk.package_json_files.push(root);
  disk.package_json_root_idx = Some(0);

  let dep_types = make_catalog_dep_types(&disk).expect("must discover top-level catalog");
  assert_eq!(dep_types.len(), 1, "exactly one top-level bun catalog dep type expected");
  let dt = &dep_types[0];
  assert_eq!(dt.name, "bunCatalog");
  assert_eq!(dt.path, "/catalog", "must use top-level path, not /workspaces/catalog");
}

#[tokio::test]
async fn pnpm_catalog_definition_emits_instance() {
  let yaml = "catalog:\n  react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn pnpm_named_catalog_definition_emits_instance() {
  let yaml = "catalogs:\n  react18:\n    react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalogs/react18 of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn multiple_pnpm_catalog_definitions_emit_multiple_instances() {
  let yaml = "catalog:\n  react: ^18.0.0\n  react-dom: ^18.0.0\ncatalogs:\n  testing:\n    jest: 30.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react-dom",
      id: "react-dom in /catalog of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "jest",
      id: "jest in /catalogs/testing of pnpm-workspace.yaml",
      actual: "30.0.0",
      expected: Some("30.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn pnpm_catalog_yaml_loaded_when_present() {
  let yaml = "catalog:\n  react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn pnpm_workspace_yaml_without_catalog_blocks_still_loads() {
  let yaml = "packages:\n  - 'packages/*'\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![ExpectedInstance {
    state: InstanceState::valid(IsLocalAndValid),
    dependency_name: "pkg-a",
    id: "pkg-a in /version of pkg-a",
    actual: "0.0.0",
    expected: Some("0.0.0"),
    overridden: None,
    severity: None,
  }]);
}

#[tokio::test]
async fn catalog_dep_types_appended_to_all_dependency_types() {
  let yaml = "catalog:\n  react: ^18.0.0\ncatalogs:\n  react18:\n    react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalogs/react18 of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn catalog_dep_type_with_dot_in_name_preserves_dot() {
  let yaml = "catalogs:\n  react.18:\n    react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalogs/react.18 of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn non_pnpm_non_bun_pm_skips_catalog_discovery() {
  // PM=Npm with no yaml on disk → no catalog discovery, only the local
  // package instance appears.
  let ctx = TestBuilder::new()
    .with_npm_package_manager()
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![ExpectedInstance {
    state: InstanceState::valid(IsLocalAndValid),
    dependency_name: "pkg-a",
    id: "pkg-a in /version of pkg-a",
    actual: "0.0.0",
    expected: Some("0.0.0"),
    overridden: None,
    severity: None,
  }]);
}

#[tokio::test]
async fn pnpm_catalog_definition_with_consumer_marked_valid() {
  let yaml = "catalog:\n  react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "catalog:"}
    })])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalog),
      dependency_name: "react",
      id: "react in /dependencies of pkg-a",
      actual: "catalog:",
      expected: Some("catalog:"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn pnpm_catalog_definition_without_consumer_does_not_flag_sibling() {
  // No consumer uses `catalog:`. The def is claimed by the `CatalogDefs`
  // catch-all → IsCatalogDefinition. The sibling (^17) reaches PreferredSemver
  // alone → IsHighestOrLowestSemver. NOT DiffersToCatalog (preserves docs).
  let yaml = "catalog:\n  react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "^17.0.0"}
    })])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "react",
      id: "react in /dependencies of pkg-a",
      actual: "^17.0.0",
      expected: Some("^17.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn auto_gen_pnpm_catalog_dep_type_has_pnpm_workspace_source_and_catalog_flag() {
  // Auto-generated pnpmCatalog dep type sources from pnpm-workspace.yaml — the
  // resulting instance's id encodes both the catalog path and the yaml source.
  let yaml = "catalog:\n  react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn auto_gen_bun_catalog_dep_type_has_package_json_source_and_catalog_flag() {
  // Auto-generated bunCatalog dep type sources from the root package.json —
  // the instance's id encodes the catalog path and the bun-root source name.
  let ctx = TestBuilder::new()
    .with_bun_catalogs(json!({"catalog": {"react": "^18.0.0"}}))
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(crate::instance::SuspectInstance::InvalidLocalVersion),
      dependency_name: "bun-root",
      id: "bun-root in /version of bun-root",
      actual: "",
      expected: Some(""),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of bun-root",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn pnpm_catalog_definition_with_no_siblings_still_valid() {
  let yaml = "catalog:\n  react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}
