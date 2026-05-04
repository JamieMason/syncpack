use {
  crate::{
    catalogs,
    disk::{Disk, PackageManager},
    errors::SyncpackError,
    instance::{InstanceState, SuspectInstance::InvalidLocalVersion, ValidInstance::*},
    test::{
      builder::TestBuilder,
      expect::{ExpectedInstance, expect},
      mock_disk::MockDiskIo,
    },
  },
  serde_json::json,
};

#[tokio::test]
async fn bun_top_level_catalog_definition_emits_instance() {
  let ctx = TestBuilder::new()
    .with_bun_catalogs(json!({
      "name": "bun-root",
      "catalog": {"react": "^18.0.0"}
    }))
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "bun-root",
      id: "bun-root in /version of bun-root",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of bun-root",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
    },
  ]);
}

#[tokio::test]
async fn bun_top_level_named_catalog_definition_emits_instance() {
  let ctx = TestBuilder::new()
    .with_bun_catalogs(json!({
      "name": "bun-root",
      "catalogs": {"react18": {"react": "^18.0.0"}}
    }))
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "bun-root",
      id: "bun-root in /version of bun-root",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalogs/react18 of bun-root",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
    },
  ]);
}

#[tokio::test]
async fn bun_workspaces_catalog_definition_emits_instance() {
  let ctx = TestBuilder::new()
    .with_bun_workspaces_catalogs(json!({
      "catalog": {"react": "^18.0.0"}
    }))
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "bun-root",
      id: "bun-root in /version of bun-root",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /workspaces/catalog of bun-root",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
    },
  ]);
}

#[tokio::test]
async fn bun_workspaces_named_catalog_definition_emits_instance() {
  let ctx = TestBuilder::new()
    .with_bun_workspaces_catalogs(json!({
      "catalogs": {"react18": {"react": "^18.0.0"}}
    }))
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "bun-root",
      id: "bun-root in /version of bun-root",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /workspaces/catalogs/react18 of bun-root",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
    },
  ]);
}

#[tokio::test]
async fn bun_dual_path_aborts() {
  // Use the real discovery path via `make_catalog_dep_types(&disk)`. Inject a
  // synthetic bun-root with catalog blocks at BOTH locations + bun.lock.
  let mut io = MockDiskIo::new();
  io.add_json(
    "package.json",
    &json!({
      "name": "bun-root",
      "catalog": {"react": "^18.0.0"},
      "workspaces": {"catalog": {"react": "^18.0.0"}}
    }),
  );
  io.add_file("bun.lock", "{}".to_string());
  let disk = Disk::from_workspace(&io, std::env::current_dir().unwrap().as_path());
  assert_eq!(disk.package_manager, Some(PackageManager::Bun));
  let err = catalogs::make_catalog_dep_types(&disk).unwrap_err();
  assert!(matches!(err, SyncpackError::BunDualCatalogPath));
}

#[tokio::test]
async fn bun_catalog_definition_with_consumer_valid_via_catalogdefs() {
  let ctx = TestBuilder::new()
    .with_bun_catalogs(json!({
      "name": "bun-root",
      "catalog": {"react": "^18.0.0"}
    }))
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "catalog:"}
    })])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::suspect(InvalidLocalVersion),
      dependency_name: "bun-root",
      id: "bun-root in /version of bun-root",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalog),
      dependency_name: "react",
      id: "react in /dependencies of pkg-a",
      actual: "catalog:",
      expected: Some("catalog:"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of bun-root",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
    },
  ]);
}

#[tokio::test]
async fn npm_with_pnpm_workspace_yaml_does_not_discover_catalogs() {
  // PM=Npm + no yaml on disk → no catalog instances; only the local package
  // surfaces.
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
  }]);
}
