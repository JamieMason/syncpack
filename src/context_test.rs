use {
  crate::{
    context::Context,
    disk::PackageManager,
    instance::{InstanceState, SuspectInstance::InvalidLocalVersion, ValidInstance::*},
    test::{
      self,
      builder::TestBuilder,
      expect::{ExpectedInstance, expect},
    },
  },
  serde_json::json,
};

#[tokio::test]
async fn instance_descriptor_source_resolves_source_idx_for_regular_dep() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": { "react": "18.0.0" }
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
      actual: "18.0.0",
      expected: Some("18.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn context_pnpm_yaml_absent_when_no_catalogs_configured() {
  // No yaml on disk → no catalog instances; only the local package surfaces.
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0"
    })])
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

#[test]
fn context_package_manager_reflects_disk_package_manager_after_create() {
  for pm in [
    PackageManager::Pnpm,
    PackageManager::Bun,
    PackageManager::Npm,
    PackageManager::Yarn,
    PackageManager::Unknown,
  ] {
    let config = test::mock::config_from_mock(json!({}));
    let (mut disk, sources) = test::mock::disk_and_sources_from_mocks(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0"
    })]);
    disk.package_manager = Some(pm);
    let ctx = Context::create(config, disk, sources, vec![]).unwrap();
    assert_eq!(ctx.package_manager(), Some(pm));
  }

  let config = test::mock::config_from_mock(json!({}));
  let (disk, sources) = test::mock::disk_and_sources_from_mocks(vec![json!({
    "name": "pkg-a",
    "version": "0.0.0"
  })]);
  let ctx = Context::create(config, disk, sources, vec![]).unwrap();
  assert!(ctx.package_manager().is_none());
}

#[tokio::test]
async fn context_create_uses_sources_for_pkg_json_instances() {
  // Pkg.json deps come through Sources iteration as regular instances; one
  // package fixture emits exactly local + each declared dep.
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({
      "name": "pkg-a",
      "version": "0.0.0",
      "dependencies": { "react": "^18.0.0", "lodash": "4.17.21" },
      "devDependencies": { "vitest": "^2.0.0" }
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
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "lodash",
      id: "lodash in /dependencies of pkg-a",
      actual: "4.17.21",
      expected: Some("4.17.21"),
      overridden: None,
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "vitest",
      id: "vitest in /devDependencies of pkg-a",
      actual: "^2.0.0",
      expected: Some("^2.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn context_create_uses_sources_for_pnpm_catalog_instances() {
  let yaml = "catalog:\n  react: ^18.0.0\ncatalogs:\n  legacy:\n    react: ^17.0.0\n";
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
      id: "react in /catalogs/legacy of pnpm-workspace.yaml",
      actual: "^17.0.0",
      expected: Some("^17.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn context_create_uses_sources_for_bun_catalog_instances() {
  // Bun catalog defs are sourced from the synthetic root package.json — both
  // default + legacy emit catalog instances against the bun-root source.
  let ctx = TestBuilder::new()
    .with_bun_catalogs(json!({"catalog": {"react": "^18.0.0"}, "catalogs": {"legacy": {"react": "^17.0.0"}}}))
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
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalogs/legacy of bun-root",
      actual: "^17.0.0",
      expected: Some("^17.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn context_create_user_customtype_pnpm_workspace_source_emits_instance() {
  // User customType reading pnpm-workspace.yaml emits a regular (non-catalog)
  // instance — observable via PreferredSemver state, not via the
  // is_catalog_definition flag.
  let yaml = "myStuff:\n  myDep: ^1.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .with_config(json!({
      "customTypes": {
        "myStuff": { "strategy": "versionsByName", "path": "myStuff", "source": "PnpmWorkspace" }
      }
    }))
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
      dependency_name: "myDep",
      id: "myDep in /myStuff of pnpm-workspace.yaml",
      actual: "^1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[tokio::test]
async fn context_create_user_customtype_targetable_in_version_group_via_dependencytypes_array() {
  // CAPABILITY GAP: a user `dependencyTypes: ["myStuff"]` selector claims the
  // customType instance into an `isIgnored` group; observable as Valid::IsIgnored
  // on the emitted myDep instance.
  let yaml = "myStuff:\n  myDep: ^1.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .with_config(json!({
      "customTypes": {
        "myStuff": { "strategy": "versionsByName", "path": "myStuff", "source": "PnpmWorkspace" }
      },
      "versionGroups": [
        { "label": "user-yaml-group", "dependencyTypes": ["myStuff"], "isIgnored": true }
      ]
    }))
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
      state: InstanceState::valid(IsIgnored),
      dependency_name: "myDep",
      id: "myDep in /myStuff of pnpm-workspace.yaml",
      actual: "^1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
      severity: None,
    },
  ]);
}

#[test]
fn context_disk_field_present() {
  let ctx = TestBuilder::new()
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .with_npm_package_manager()
    .build();
  let _: &crate::disk::Disk = &ctx.disk;
  assert_eq!(ctx.package_manager(), Some(PackageManager::Npm));
}

#[test]
fn context_package_manager_via_disk() {
  type Apply = fn(TestBuilder) -> TestBuilder;
  let cases: [(Apply, PackageManager); 5] = [
    (|b: TestBuilder| b.with_pnpm_package_manager(), PackageManager::Pnpm),
    (|b: TestBuilder| b.with_bun_package_manager(), PackageManager::Bun),
    (|b: TestBuilder| b.with_npm_package_manager(), PackageManager::Npm),
    (|b: TestBuilder| b.with_yarn_package_manager(), PackageManager::Yarn),
    (|b: TestBuilder| b.with_unknown_package_manager(), PackageManager::Unknown),
  ];
  for (with_pm, expected) in cases {
    let builder = TestBuilder::new().with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})]);
    let ctx = with_pm(builder).build();
    assert_eq!(ctx.package_manager(), Some(expected));
    assert_eq!(ctx.package_manager(), ctx.disk.package_manager);
  }
}

#[test]
fn context_create_no_package_name_clone() {
  // Pins the invariant: every Instance.id ends with
  // "of {sources.all[source_idx].name()}" — package_name is read via that
  // lookup at use sites, not stored alongside descriptors as a String tuple.
  let yaml = "catalog:\n  react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![
      json!({
        "name": "pkg-a",
        "version": "0.0.0",
        "dependencies": { "react": "^18.0.0", "lodash": "4.17.21" }
      }),
      json!({
        "name": "pkg-b",
        "version": "0.0.0",
        "devDependencies": { "vitest": "^2.0.0" }
      }),
    ])
    .build();

  for instance in &ctx.instances {
    let expected = ctx.sources.all[instance.source_idx().0].name();
    let suffix = format!(" of {expected}");
    assert!(
      instance.id.ends_with(&suffix),
      "instance id `{}` must end with `{}` (looked up from sources arena, not from a String tuple)",
      instance.id,
      suffix
    );
  }
}

#[test]
fn catalog_defs_group_not_injected_when_no_catalog_dep_types() {
  use crate::version_group::VersionGroup;

  let config = test::mock::config_from_mock(json!({
    "versionGroups": [
      { "label": "pinned", "pinVersion": "1.0.0" }
    ]
  }));
  let (disk, sources) = test::mock::disk_and_sources_from_mocks(vec![json!({
    "name": "pkg-a",
    "version": "0.0.0",
    "dependencies": { "react": "18.0.0" }
  })]);
  let ctx = Context::create(config, disk, sources, vec![]).unwrap();

  let has_catalog_defs = ctx.version_groups.iter().any(|g| matches!(g, VersionGroup::CatalogDefs(_)));
  assert!(
    !has_catalog_defs,
    "CatalogDefs group should not be injected when no catalog dep types exist"
  );
}
