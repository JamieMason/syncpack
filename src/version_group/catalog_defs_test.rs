use {
  crate::{
    instance::{FixableInstance::*, InstanceState, ValidInstance::*},
    test::{
      builder::TestBuilder,
      expect::{ExpectedInstance, expect},
    },
  },
  serde_json::json,
};

#[tokio::test]
async fn catalog_def_dep_has_expected_specifier_set() {
  // The catalog def's `expected` is set to its own specifier (so list view
  // can render the version). Observable as the `expected` field on the
  // emitted instance.
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
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
    },
  ]);
}

#[tokio::test]
async fn catalog_defs_catch_all_marks_defs_valid() {
  // No user groups, no consumer. Every catalog def is claimed by the
  // auto-injected `CatalogDefs` catch-all → `IsCatalogDefinition`.
  let yaml = "catalog:\n  react: ^18.0.0\n  react-dom: ^18.0.0\n";
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
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react-dom",
      id: "react-dom in /catalog of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
    },
  ]);
}

#[tokio::test]
async fn user_banned_group_can_claim_catalog_def_first() {
  // A user-defined `banned` group with `dependencyTypes: ['pnpmCatalog']`
  // claims defs before the auto-injected `CatalogDefs` catch-all
  // (first-match-wins) → catalog def is IsBanned, not IsCatalogDefinition.
  let yaml = "catalog:\n  react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .with_version_group(json!({
      "label": "ban catalog defs",
      "dependencyTypes": ["pnpmCatalog"],
      "isBanned": true,
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
    },
    ExpectedInstance {
      state: InstanceState::fixable(IsBanned),
      dependency_name: "react",
      id: "react in /catalog of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some(""),
      overridden: None,
    },
  ]);
}

#[tokio::test]
async fn multi_catalog_defs_no_consumer() {
  // `catalog: { react: ^17 }` + `catalogs.react18: { react: ^18 }`. Both defs
  // claimed by `CatalogDefs` catch-all → both `IsCatalogDefinition`. Sibling
  // at ^17 (no `catalog:`) reaches PreferredSemver alone → `IsHighestOrLowestSemver`.
  // NOT `DiffersToCatalog` (defs never reach PreferredSemver).
  let yaml = "catalog:\n  react: ^17.0.0\ncatalogs:\n  react18:\n    react: ^18.0.0\n";
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
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "react",
      id: "react in /dependencies of pkg-a",
      actual: "^17.0.0",
      expected: Some("^17.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of pnpm-workspace.yaml",
      actual: "^17.0.0",
      expected: Some("^17.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalogs/react18 of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
    },
  ]);
}

#[tokio::test]
async fn multi_catalog_defs_with_consumer_uses_consumer_target() {
  // Same defs, pkg-a uses `catalog:react18`. Defs claimed by catch-all.
  // Consumer (pkg-a) is IsCatalog; sibling (pkg-b at ^17) is DiffersToCatalog
  // with target = consumer's `catalog:react18`.
  let yaml = "catalog:\n  react: ^17.0.0\ncatalogs:\n  react18:\n    react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![
      json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": "catalog:react18"}}),
      json!({"name": "pkg-b", "version": "0.0.0", "dependencies": {"react": "^17.0.0"}}),
    ])
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
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-b",
      id: "pkg-b in /version of pkg-b",
      actual: "0.0.0",
      expected: Some("0.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalog),
      dependency_name: "react",
      id: "react in /dependencies of pkg-a",
      actual: "catalog:react18",
      expected: Some("catalog:react18"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToCatalog),
      dependency_name: "react",
      id: "react in /dependencies of pkg-b",
      actual: "^17.0.0",
      expected: Some("catalog:react18"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of pnpm-workspace.yaml",
      actual: "^17.0.0",
      expected: Some("^17.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalogs/react18 of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
    },
  ]);
}

#[tokio::test]
async fn pnpm_catalog_definition_with_local_does_not_pull_def_into_local_branch() {
  // Local instance + def for the same dep. Def claimed by catch-all
  // → IsCatalogDefinition. Local instance reaches PreferredSemver alone
  // → IsLocalAndValid. The def must NOT be pulled into the local branch.
  let yaml = "catalog:\n  pkg-a: ^1.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![json!({"name": "pkg-a", "version": "1.0.0"})])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "pkg-a",
      id: "pkg-a in /version of pkg-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "pkg-a",
      id: "pkg-a in /catalog of pnpm-workspace.yaml",
      actual: "^1.0.0",
      expected: Some("^1.0.0"),
      overridden: None,
    },
  ]);
}

mod registry_updates {
  use super::*;

  #[tokio::test]
  async fn pnpm_catalog_def_marked_outdated_when_registry_has_newer_version() {
    let yaml = "catalog:\n  react: ^17.0.0\n";
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![json!({
        "name": "pkg-a",
        "version": "0.0.0",
        "dependencies": {"react": "catalog:"},
      })])
      .with_registry_updates(json!({"react": ["17.0.0", "18.5.0"]}))
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
        state: InstanceState::fixable(DiffersToNpmRegistry),
        dependency_name: "react",
        id: "react in /catalog of pnpm-workspace.yaml",
        actual: "^17.0.0",
        expected: Some("^18.5.0"),
        overridden: None,
      },
    ]);
  }

  #[tokio::test]
  async fn pnpm_named_catalog_def_marked_outdated_when_registry_has_newer_version() {
    let yaml = "catalogs:\n  react18:\n    react: ^17.0.0\n";
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![json!({
        "name": "pkg-a",
        "version": "0.0.0",
        "dependencies": {"react": "catalog:react18"},
      })])
      .with_registry_updates(json!({"react": ["17.0.0", "18.5.0"]}))
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
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalog),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "catalog:react18",
        expected: Some("catalog:react18"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(DiffersToNpmRegistry),
        dependency_name: "react",
        id: "react in /catalogs/react18 of pnpm-workspace.yaml",
        actual: "^17.0.0",
        expected: Some("^18.5.0"),
        overridden: None,
      },
    ]);
  }

  #[tokio::test]
  async fn bun_catalog_def_marked_outdated_when_registry_has_newer_version() {
    let ctx = TestBuilder::new()
      .with_bun_catalogs(json!({
        "name": "bun-root",
        "version": "0.0.0",
        "catalog": {"react": "^17.0.0"},
      }))
      .with_packages(vec![json!({
        "name": "pkg-a",
        "version": "0.0.0",
        "dependencies": {"react": "catalog:"},
      })])
      .with_registry_updates(json!({"react": ["17.0.0", "18.5.0"]}))
      .run()
      .await;
    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "bun-root",
        id: "bun-root in /version of bun-root",
        actual: "0.0.0",
        expected: Some("0.0.0"),
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
        state: InstanceState::fixable(DiffersToNpmRegistry),
        dependency_name: "react",
        id: "react in /catalog of bun-root",
        actual: "^17.0.0",
        expected: Some("^18.5.0"),
        overridden: None,
      },
    ]);
  }

  #[tokio::test]
  async fn pnpm_catalog_def_with_registry_match_stays_valid() {
    // Catalog def already matches the highest registry version → IsCatalogDefinition.
    let yaml = "catalog:\n  react: ^18.5.0\n";
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![json!({
        "name": "pkg-a",
        "version": "0.0.0",
        "dependencies": {"react": "catalog:"},
      })])
      .with_registry_updates(json!({"react": ["17.0.0", "18.5.0"]}))
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
        id: "react in /catalog of pnpm-workspace.yaml",
        actual: "^18.5.0",
        expected: Some("^18.5.0"),
        overridden: None,
      },
    ]);
  }
}

#[tokio::test]
async fn defensive_catalog_def_short_circuit_in_preferred_semver() {
  // A user-defined `policy: highest-semver` group with dependencyTypes:
  // ['pnpmCatalog'] would claim the def into a non-CatalogDefs group. The
  // defensive short-circuit inside PreferredSemver should still mark it
  // IsCatalogDefinition rather than DiffersToHighestOrLowestSemver.
  let yaml = "catalog:\n  react: ^18.0.0\n";
  let ctx = TestBuilder::new()
    .with_pnpm_catalogs(yaml)
    .with_packages(vec![
      json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": "^19.0.0"}}),
    ])
    .with_version_group(json!({
      "label": "force defs into preferred semver",
      "dependencyTypes": ["pnpmCatalog"],
      "preferVersion": "highestSemver",
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
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "react",
      id: "react in /dependencies of pkg-a",
      actual: "^19.0.0",
      expected: Some("^19.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsCatalogDefinition),
      dependency_name: "react",
      id: "react in /catalog of pnpm-workspace.yaml",
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
    },
  ]);
}
