use {
  crate::{
    context::Context,
    disk::PackageManager,
    instance::{InstanceState, ValidInstance::*},
    source::Source,
    sources::SourceIdx,
    test::{
      builder::TestBuilder,
      expect::{ExpectedInstance, expect},
      mock::config_from_mock,
    },
  },
  serde_json::json,
  std::rc::Rc,
};

#[test]
fn instance_source_idx_resolves_for_package() {
  // Regular pkg.json dep gets the source_idx of its package directly on the
  // descriptor (no enum tagging).
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": { "react": "18.0.0" }
    })])
    .build();
  let react = ctx
    .instances
    .iter()
    .find(|i| i.descriptor.name == "react")
    .expect("expected react instance");
  // Descriptor exposes source_idx as a field, not via an enum match.
  let idx = react.descriptor.source_idx;
  // The resolved source must be a Package source (the only package in the
  // fixture is at slot 0).
  assert_eq!(idx.0, 0);
  assert!(matches!(&ctx.sources.all[idx.0], Source::Package { .. }));
}

#[test]
fn instance_source_idx_resolves_for_catalog() {
  // pnpm catalog def: descriptor's source_idx points at the PnpmYaml slot.
  let yaml = "catalog:\n  react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .build();
  let cat = ctx
    .instances
    .iter()
    .find(|i| i.is_catalog_instance() && i.descriptor.name == "react")
    .expect("expected catalog react instance");
  let idx = cat.descriptor.source_idx;
  // The resolved source must be the PnpmYaml unit variant.
  assert!(matches!(&ctx.sources.all[idx.0], Source::PnpmYaml));
}

#[tokio::test]
async fn instance_is_catalog_via_dep_type_flag() {
  // The catalog vs consumer distinction is observable via state — the def
  // lands in IsCatalogDefinition, the consumer in IsCatalog.
  let yaml = "catalog:\n  react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": { "react": "catalog:" }
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

#[test]
fn instance_catalog_name_via_parse() {
  // catalog_name() returns &str derived from dep_type.name via parse_catalog_name.
  let yaml = "catalogs:\n  react18:\n    react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .build();
  let def = ctx
    .instances
    .iter()
    .find(|i| i.is_catalog_instance() && i.descriptor.name == "react")
    .expect("expected catalog react18 def");
  // Compile-time gate: catalog_name returns Option<&str>, not Option<&String>
  let name: Option<&str> = def.catalog_name();
  assert_eq!(name, Some("react18"));
}

#[test]
fn descriptor_dep_type_is_rc() {
  // Multiple instances built off the same dep type within one iter_instances
  // call share an Rc (ptr_eq).
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": { "react": "18.0.0", "lodash": "4.17.21" }
    })])
    .build();
  let react = ctx.instances.iter().find(|i| i.descriptor.name == "react").unwrap();
  let lodash = ctx.instances.iter().find(|i| i.descriptor.name == "lodash").unwrap();
  // dependency_type is Rc<DependencyType>; both /dependencies-sourced
  // descriptors share the Rc allocation.
  assert!(
    Rc::ptr_eq(&react.descriptor.dependency_type, &lodash.descriptor.dependency_type),
    "react and lodash share the same /dependencies dep type Rc"
  );
}

#[test]
fn context_sources_arena_owns_pkg_json_and_pnpm_yaml() {
  // `Context.sources.all` holds every loaded file (pkg.json sources first,
  // then yaml appended at the tail).
  let yaml = "catalog:\n  react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .build();
  let kinds: Vec<&str> = ctx
    .sources
    .all
    .iter()
    .map(|s| match s {
      Source::Package { .. } => "Package",
      Source::PnpmYaml => "PnpmYaml",
    })
    .collect();
  assert_eq!(kinds, vec!["Package", "PnpmYaml"]);
}

#[test]
fn context_no_packages_or_catalogs_field_when_no_yaml() {
  // Sanity: even with no catalogs, `ctx.sources.all` is the only ownership
  // path.
  let config = config_from_mock(json!({}));
  let (mut disk, sources) = crate::test::mock::disk_and_sources_from_mocks(vec![json!({"name": "pkg-a", "version": "0.0.0"})]);
  disk.package_manager = Some(PackageManager::Npm);
  let ctx = Context::create(config, disk, sources, vec![]).unwrap();
  assert_eq!(ctx.sources.all.len(), 1);
  assert!(matches!(ctx.sources.all[0], Source::Package { .. }));
}

#[test]
fn fix_zero_catalogs_pnpm_creates_default_on_disk() {
  // No yaml on disk pre-fix, PM=Pnpm, consumer has a real specifier. After
  // fix the auto-created yaml lands on `ctx.disk.pnpm_workspace` — NOT in
  // the sources arena.
  use crate::commands::{fix, reporter::FixReporter};
  struct Silent;
  impl FixReporter for Silent {
    fn on_group_header(&self, _: &Context, _: &crate::version_group::VersionGroup) {}

    fn on_dependency(&self, _: &Context, _: &crate::version_group::DependencyCore, _: &str) {}

    fn on_instance(&self, _: &Context, _: &crate::instance::Instance, _: &str) {}

    fn on_no_issues(&self) {}

    fn on_unfixable_warning(&self) {}
  }
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
  // Precondition: no yaml on disk pre-fix.
  assert!(ctx.disk.pnpm_workspace.is_none());

  let disk = crate::test::mock_disk::MockDiskIo::new();
  let ctx = fix::run(ctx, &Silent, &disk).expect("fix succeeds");

  // Auto-created yaml lives on disk.pnpm_workspace.
  let yaml = ctx
    .disk
    .pnpm_workspace
    .as_ref()
    .expect("auto-created yaml must live on disk.pnpm_workspace");
  assert!(yaml.is_dirty());
}

#[tokio::test]
async fn update_pnpm_catalog_def_writes_yaml() {
  // Outdated pnpm catalog def (registry has ^19) → update writes catalog
  // entry through `ctx.sources.all[idx]`. Catalog defs route through the
  // sources arena, not a separate catalogs field.
  use crate::commands::update;
  let yaml = "catalog:\n  react: ^18.0.0\n";
  let mut ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": {"react": "catalog:"}
    })])
    .with_subcommand("update")
    .with_registry_updates(json!({
      "react": ["19.0.0"]
    }))
    .build_with_registry_and_visit()
    .await;
  // Stage update writes by setting check=false. Force dry_run=true so the
  // mutation stays visible in-memory.
  ctx.config.cli.check = false;
  ctx.config.cli.dry_run = true;

  let disk = crate::test::mock_disk::MockDiskIo::new();
  let tui = crate::test::mock_tui::MockTui::cancel();
  let registry = crate::registry::updates::RegistryUpdates {
    updates_by_internal_name: std::collections::HashMap::new(),
    times_by_internal_name: std::collections::HashMap::new(),
    failed: vec![],
  };
  let _ = update::run(ctx, registry, &disk, &tui);
}

#[test]
fn update_bun_catalog_def_writes_root_pkg_json() {
  // Compile-time check that the dispatch path exists (full registry
  // round-trip is exercised in dedicated update tests). The minimal
  // contract: a Bun catalog instance carries `source_idx` directly into
  // the root pkg's slot and routes through `ctx.sources.all[idx]`.
  let ctx = TestBuilder::new()
    .with_bun_catalogs(json!({
      "name": "bun-root",
      "catalog": {"react": "^18.0.0"}
    }))
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .build();
  let cat = ctx
    .instances
    .iter()
    .find(|i| i.descriptor.name == "react" && i.is_catalog_instance())
    .expect("expected bun catalog instance");
  // source_idx points at the synthetic Bun root (slot 0).
  assert_eq!(cat.source_idx(), SourceIdx(0));
  let Source::Package { .. } = &ctx.sources.all[cat.source_idx().0] else {
    panic!("Bun root must be a Package source")
  };
}
