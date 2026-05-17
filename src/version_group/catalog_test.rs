use {
  crate::{
    instance::{FixableInstance::*, InstanceState, SuspectInstance::*, UnfixableInstance::*, ValidInstance::*},
    specifier::Specifier,
    test::{
      builder::TestBuilder,
      expect::{ExpectedInstance, expect},
    },
  },
  serde_json::json,
  std::rc::Rc,
};

mod definition_and_consumer {
  use super::*;

  #[tokio::test]
  async fn catalog_group_definition_valid() {
    // pnpm catalog has react. Sibling uses `catalog:`. Both claimed by the
    // user-defined `policy: "catalog"` group. Def → `IsCatalogDefinition`,
    // sibling → `IsCatalog`.
    let yaml = "catalog:\n  react: ^18.0.0\n";
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![json!({
        "name": "pkg-a",
        "version": "0.0.0",
        "dependencies": {"react": "catalog:"},
      })])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
  async fn catalog_group_aliased_dep_via_dependency_groups_uses_internal_name_lookup() {
    // dependency_groups aliases the catalog def + its consumer to the same
    // alias label. CatalogGroup queries `catalog_defs_for(&dep.internal_name)`
    // — not `descriptor.name` — so the def lookup matches even though both
    // instances now carry the alias label as their internal_name.
    let yaml = "catalog:\n  react: ^18.0.0\n";
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![json!({
        "name": "pkg-a",
        "version": "0.0.0",
        "dependencies": {"react": "^18.0.0"},
      })])
      .with_config(json!({
        "dependencyGroups": [{
          "aliasName": "react-alias",
          "dependencies": ["react"],
        }],
        "versionGroups": [{
          "label": "enforce catalog",
          "dependencies": ["react-alias"],
          "policy": "catalog",
        }],
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
        state: InstanceState::fixable(NotUsingCatalog("default".to_string())),
        dependency_name: "react-alias",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "react-alias",
        id: "react in /catalog of pnpm-workspace.yaml",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_is_catalog_via_bun_in_user_policy_catalog_group() {
    // Bun catalog def + `catalog:` consumer, both claimed by user `policy:"catalog"`.
    // Mirrors `catalog_group_definition_valid` under PM=Bun. The existing
    // `bun_catalog_definition_with_consumer_valid_via_catalogdefs` flows through
    // CatalogDefs/PreferredSemver, NOT the user-policy CatalogGroup — this test
    // exercises the Bun branch 1 + branch 3 path inside CatalogGroup itself.
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
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
      }))
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
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "react",
        id: "react in /catalog of bun-root",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
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
        state: InstanceState::valid(IsCatalog),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "catalog:",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
    ]);
  }
}

mod refuse_to_catalog_local {
  use super::*;

  #[tokio::test]
  async fn catalog_group_refuse_to_catalog_local() {
    // Local instance is the package's own /version property. Catalogs cannot
    // contain local-package references, so a CatalogGroup that claims the local
    // → Suspect::RefuseToCatalogLocal. Mirrors RefuseToBanLocal / RefuseToPinLocal.
    // The /dependencies/package-a consumer in package-b also lands in CatalogGroup
    // (matches dep filter): catalog_defs for package-a empty + 1 catalog (lodash)
    // → MissingFromCatalog{default, "1.0.0"}.
    let yaml = "catalog:\n  lodash: ^4.0.0\n";
    let winning = Specifier::new("1.0.0");
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![
        json!({"name": "package-a", "version": "1.0.0"}),
        json!({"name": "package-b", "dependencies": {"package-a": "1.0.0"}}),
      ])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["package-a"],
        "policy": "catalog",
      }))
      .run()
      .await;

    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(RefuseToCatalogLocal),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "1.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "lodash",
        id: "lodash in /catalog of pnpm-workspace.yaml",
        actual: "^4.0.0",
        expected: Some("^4.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_refuse_to_catalog_local_bun() {
    // Local instance under PM=Bun. Mirror of `catalog_group_refuse_to_catalog_local`.
    let winning = Specifier::new("1.0.0");
    let ctx = TestBuilder::new()
      .with_bun_catalogs(json!({
        "name": "bun-root",
        "catalog": {"lodash": "^4.0.0"}
      }))
      .with_packages(vec![
        json!({"name": "package-a", "version": "1.0.0"}),
        json!({"name": "package-b", "dependencies": {"package-a": "1.0.0"}}),
      ])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["package-a"],
        "policy": "catalog",
      }))
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
        state: InstanceState::suspect(InvalidLocalVersion),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "",
        expected: Some(""),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "lodash",
        id: "lodash in /catalog of bun-root",
        actual: "^4.0.0",
        expected: Some("^4.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::suspect(RefuseToCatalogLocal),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "1.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
    ]);
  }
}

mod missing_catalog_definition_reference {
  use super::*;

  #[tokio::test]
  async fn catalog_group_depends_on_missing_catalog_definition_no_catalog_named() {
    // Consumer uses `catalog:foo` but no catalog named `foo` exists →
    // Suspect::DependsOnMissingCatalogDefinition.
    let yaml = "catalog:\n  react: ^18.0.0\n";
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![json!({
        "name": "pkg-a",
        "version": "0.0.0",
        "dependencies": {"react": "catalog:foo"},
      })])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::suspect(DependsOnMissingCatalogDefinition),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "catalog:foo",
        expected: Some("catalog:foo"),
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
  async fn catalog_group_depends_on_missing_catalog_definition_dep_not_in_catalog() {
    // Default catalog exists but does NOT define `react`. Consumer uses bare
    // `catalog:` (= "default") → Suspect::DependsOnMissingCatalogDefinition.
    let yaml = "catalog:\n  lodash: ^4.0.0\n";
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![json!({
        "name": "pkg-a",
        "version": "0.0.0",
        "dependencies": {"react": "catalog:"},
      })])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::suspect(DependsOnMissingCatalogDefinition),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "catalog:",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "lodash",
        id: "lodash in /catalog of pnpm-workspace.yaml",
        actual: "^4.0.0",
        expected: Some("^4.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_depends_on_missing_catalog_definition_dep_in_other_catalog() {
    // Consumer uses `catalog:foo`, catalog `foo` exists but does NOT define
    // `react`; `react` lives in catalog `bar` instead → still
    // DependsOnMissingCatalogDefinition (the consumer points at the wrong place).
    let yaml = "catalogs:\n  foo:\n    lodash: ^4.0.0\n  bar:\n    react: ^18.0.0\n";
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![json!({
        "name": "pkg-a",
        "version": "0.0.0",
        "dependencies": {"react": "catalog:foo"},
      })])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::suspect(DependsOnMissingCatalogDefinition),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "catalog:foo",
        expected: Some("catalog:foo"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "lodash",
        id: "lodash in /catalogs/foo of pnpm-workspace.yaml",
        actual: "^4.0.0",
        expected: Some("^4.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "react",
        id: "react in /catalogs/bar of pnpm-workspace.yaml",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_depends_on_missing_catalog_definition_bun() {
    // Bun + consumer points at `catalog:foo`, no such catalog → DependsOnMissingCatalogDefinition.
    let ctx = TestBuilder::new()
      .with_bun_catalogs(json!({
        "name": "bun-root",
        "catalog": {"react": "^18.0.0"}
      }))
      .with_packages(vec![json!({
        "name": "pkg-a",
        "version": "0.0.0",
        "dependencies": {"react": "catalog:foo"}
      })])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
      }))
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
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "react",
        id: "react in /catalog of bun-root",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
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
        state: InstanceState::suspect(DependsOnMissingCatalogDefinition),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "catalog:foo",
        expected: Some("catalog:foo"),
        overridden: None,
        severity: None,
      },
    ]);
  }
}

mod not_using_catalog {
  use super::*;

  #[tokio::test]
  async fn catalog_group_not_using_catalog_fixable() {
    // pnpm default catalog has react: ^18.0.0. Sibling has react: ^18.0.0
    // (real specifier, not `catalog:`). CatalogGroup → sibling is
    // `Fixable::NotUsingCatalog("default")`.
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
        state: InstanceState::fixable(NotUsingCatalog("default".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
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
  async fn catalog_group_named_not_using_catalog_fixable() {
    // pnpm named catalog `react18` has react: ^18.0.0. Sibling has
    // ^18.0.0 (real specifier). CatalogGroup → `NotUsingCatalog("react18")`.
    let yaml = "catalogs:\n  react18:\n    react: ^18.0.0\n";
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
        state: InstanceState::fixable(NotUsingCatalog("react18".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("catalog:react18"),
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
  async fn catalog_group_two_catalogs_dep_in_exactly_one_fixable() {
    // 2+ catalogs project-wide, dep `react` defined in exactly one (`react17`)
    // → branch 4 fires via `catalog_defs.len() == 1` → NotUsingCatalog("react17").
    // Without this test the multi-catalog/single-def Fixable path is structurally
    // untested even though the same code line serves the 1-catalog case.
    let yaml = "catalogs:\n  react17:\n    react: ^17.0.0\n  legacy:\n    underscore: ^1.0.0\n";
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![json!({
        "name": "pkg-a",
        "version": "0.0.0",
        "dependencies": {"react": "^17.0.0"},
      })])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::fixable(NotUsingCatalog("react17".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^17.0.0",
        expected: Some("catalog:react17"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "react",
        id: "react in /catalogs/react17 of pnpm-workspace.yaml",
        actual: "^17.0.0",
        expected: Some("^17.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "underscore",
        id: "underscore in /catalogs/legacy of pnpm-workspace.yaml",
        actual: "^1.0.0",
        expected: Some("^1.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_not_using_catalog_bun_fixable() {
    // Bun + 1 catalog (default) containing react + consumer using real specifier
    // → NotUsingCatalog("default"). Mirror of `catalog_group_not_using_catalog_fixable`.
    let ctx = TestBuilder::new()
      .with_bun_catalogs(json!({
        "name": "bun-root",
        "catalog": {"react": "^18.0.0"}
      }))
      .with_packages(vec![json!({
        "name": "pkg-a",
        "version": "0.0.0",
        "dependencies": {"react": "^18.0.0"}
      })])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
      }))
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
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "react",
        id: "react in /catalog of bun-root",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
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
        state: InstanceState::fixable(NotUsingCatalog("default".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
    ]);
  }
}

mod missing_from_catalog {
  use super::*;

  #[tokio::test]
  async fn catalog_group_missing_single_catalog_fixable() {
    // 1 catalog (default) exists but does NOT define `react`. A consumer with
    // a real specifier in the CatalogGroup → MissingFromCatalog("default") with
    // the consumer's specifier as `winning_specifier`.
    let yaml = "catalog:\n  lodash: ^4.0.0\n";
    let winning = Specifier::new("^18.0.0");
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
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "lodash",
        id: "lodash in /catalog of pnpm-workspace.yaml",
        actual: "^4.0.0",
        expected: Some("^4.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_missing_zero_catalogs_pnpm() {
    // No catalogs configured, PM=pnpm → MissingFromCatalog("default").
    let winning = Specifier::new("^18.0.0");
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
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_missing_zero_catalogs_bun() {
    // No catalogs configured, PM=bun → MissingFromCatalog("default").
    let winning = Specifier::new("^18.0.0");
    let ctx = TestBuilder::new()
      .with_bun_package_manager()
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
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_missing_specifier_all_semver_highest_wins() {
    // 1 catalog exists but no `react` def. Multiple semver consumers → fixable
    // with highest specifier as `winning_specifier`.
    let yaml = "catalog:\n  lodash: ^4.0.0\n";
    let winning = Specifier::new("^18.0.0");
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![
        json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": "^17.0.0"}}),
        json!({"name": "pkg-b", "version": "0.0.0", "dependencies": {"react": "^18.0.0"}}),
      ])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "pkg-b",
        id: "pkg-b in /version of pkg-b",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^17.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-b",
        actual: "^18.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "lodash",
        id: "lodash in /catalog of pnpm-workspace.yaml",
        actual: "^4.0.0",
        expected: Some("^4.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_missing_specifier_all_identical_non_semver_fixable() {
    // 1 catalog exists but no `react` def. Multiple consumers all using the
    // same byte-identical non-semver specifier → fixable; that specifier is
    // the winning one.
    let yaml = "catalog:\n  lodash: ^4.0.0\n";
    let url = "git+https://github.com/facebook/react.git#abc";
    let winning = Specifier::new(url);
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![
        json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": url}}),
        json!({"name": "pkg-b", "version": "0.0.0", "dependencies": {"react": url}}),
      ])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "pkg-b",
        id: "pkg-b in /version of pkg-b",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "git+https://github.com/facebook/react.git#abc",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-b",
        actual: "git+https://github.com/facebook/react.git#abc",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "lodash",
        id: "lodash in /catalog of pnpm-workspace.yaml",
        actual: "^4.0.0",
        expected: Some("^4.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_missing_specifier_conflict_mixed() {
    // 1 catalog exists but no `react` def. Two consumers: one semver, one
    // non-semver (git URL) → unfixable (cannot pick a winner).
    let yaml = "catalog:\n  lodash: ^4.0.0\n";
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![
        json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": "^18.0.0"}}),
        json!({"name": "pkg-b", "version": "0.0.0", "dependencies": {"react": "git+https://github.com/facebook/react.git"}}),
      ])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "pkg-b",
        id: "pkg-b in /version of pkg-b",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(MissingFromCatalogAndNonSemverMismatch("default".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(MissingFromCatalogAndNonSemverMismatch("default".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-b",
        actual: "git+https://github.com/facebook/react.git",
        expected: Some("git+https://github.com/facebook/react.git"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "lodash",
        id: "lodash in /catalog of pnpm-workspace.yaml",
        actual: "^4.0.0",
        expected: Some("^4.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_missing_specifier_conflict_all_non_semver_different() {
    // 1 catalog exists but no `react` def. Two consumers, both non-semver but
    // different (two distinct git URLs) → unfixable.
    let yaml = "catalog:\n  lodash: ^4.0.0\n";
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![
        json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": "git+https://github.com/facebook/react.git#a"}}),
        json!({"name": "pkg-b", "version": "0.0.0", "dependencies": {"react": "git+https://github.com/facebook/react.git#b"}}),
      ])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "pkg-b",
        id: "pkg-b in /version of pkg-b",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(MissingFromCatalogAndNonSemverMismatch("default".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "git+https://github.com/facebook/react.git#a",
        expected: Some("git+https://github.com/facebook/react.git#a"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(MissingFromCatalogAndNonSemverMismatch("default".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-b",
        actual: "git+https://github.com/facebook/react.git#b",
        expected: Some("git+https://github.com/facebook/react.git#b"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "lodash",
        id: "lodash in /catalog of pnpm-workspace.yaml",
        actual: "^4.0.0",
        expected: Some("^4.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_zero_catalogs_pnpm_all_semver_highest_wins_fixable() {
    // 0 catalogs project-wide + PM=pnpm + multi semver consumers → highest wins.
    let winning = Specifier::new("^18.0.0");
    let ctx = TestBuilder::new()
      .with_pnpm_package_manager()
      .with_packages(vec![
        json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": "^17.0.0"}}),
        json!({"name": "pkg-b", "version": "0.0.0", "dependencies": {"react": "^18.0.0"}}),
      ])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "pkg-b",
        id: "pkg-b in /version of pkg-b",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^17.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-b",
        actual: "^18.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_zero_catalogs_pnpm_all_identical_non_semver_fixable() {
    // 0 catalogs + PM=pnpm + multi consumers all using byte-identical non-semver
    // url → that url wins.
    let url = "git+https://github.com/facebook/react.git#abc";
    let winning = Specifier::new(url);
    let ctx = TestBuilder::new()
      .with_pnpm_package_manager()
      .with_packages(vec![
        json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": url}}),
        json!({"name": "pkg-b", "version": "0.0.0", "dependencies": {"react": url}}),
      ])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "pkg-b",
        id: "pkg-b in /version of pkg-b",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "git+https://github.com/facebook/react.git#abc",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-b",
        actual: "git+https://github.com/facebook/react.git#abc",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_zero_catalogs_pnpm_conflict_mixed() {
    // 0 catalogs + PM=pnpm + mixed semver / non-semver → unfixable.
    let ctx = TestBuilder::new()
      .with_pnpm_package_manager()
      .with_packages(vec![
        json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": "^18.0.0"}}),
        json!({"name": "pkg-b", "version": "0.0.0", "dependencies": {"react": "git+https://github.com/facebook/react.git"}}),
      ])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "pkg-b",
        id: "pkg-b in /version of pkg-b",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(MissingFromCatalogAndNonSemverMismatch("default".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(MissingFromCatalogAndNonSemverMismatch("default".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-b",
        actual: "git+https://github.com/facebook/react.git",
        expected: Some("git+https://github.com/facebook/react.git"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_zero_catalogs_bun_all_semver_highest_wins_fixable() {
    // 0 catalogs + PM=bun + multi semver → highest wins.
    let winning = Specifier::new("^18.0.0");
    let ctx = TestBuilder::new()
      .with_bun_package_manager()
      .with_packages(vec![
        json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": "^17.0.0"}}),
        json!({"name": "pkg-b", "version": "0.0.0", "dependencies": {"react": "^18.0.0"}}),
      ])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "pkg-b",
        id: "pkg-b in /version of pkg-b",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^17.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-b",
        actual: "^18.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_zero_catalogs_bun_all_identical_non_semver_fixable() {
    // 0 catalogs + PM=bun + multi consumers all using byte-identical non-semver
    // url → that url wins.
    let url = "git+https://github.com/facebook/react.git#abc";
    let winning = Specifier::new(url);
    let ctx = TestBuilder::new()
      .with_bun_package_manager()
      .with_packages(vec![
        json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": url}}),
        json!({"name": "pkg-b", "version": "0.0.0", "dependencies": {"react": url}}),
      ])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "pkg-b",
        id: "pkg-b in /version of pkg-b",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "git+https://github.com/facebook/react.git#abc",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-b",
        actual: "git+https://github.com/facebook/react.git#abc",
        expected: Some("catalog:"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_zero_catalogs_bun_conflict_mixed() {
    // 0 catalogs + PM=bun + mixed → unfixable.
    let ctx = TestBuilder::new()
      .with_bun_package_manager()
      .with_packages(vec![
        json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": "^18.0.0"}}),
        json!({"name": "pkg-b", "version": "0.0.0", "dependencies": {"react": "git+https://github.com/facebook/react.git"}}),
      ])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "pkg-b",
        id: "pkg-b in /version of pkg-b",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(MissingFromCatalogAndNonSemverMismatch("default".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(MissingFromCatalogAndNonSemverMismatch("default".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-b",
        actual: "git+https://github.com/facebook/react.git",
        expected: Some("git+https://github.com/facebook/react.git"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_named_catalog_missing_specifier_all_semver_highest_wins_fixable() {
    // 1 named catalog (`react18`) exists, dep `react` absent. Multi-consumer
    // all-semver → MissingFromCatalog{"react18", highest}, expected = catalog:react18.
    // Verifies catalog_name carry-through (every existing dep-absent merge test
    // hard-coded the default catalog).
    let yaml = "catalogs:\n  react18:\n    lodash: ^4.0.0\n";
    let winning = Specifier::new("^18.0.0");
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![
        json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": "^17.0.0"}}),
        json!({"name": "pkg-b", "version": "0.0.0", "dependencies": {"react": "^18.0.0"}}),
      ])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "pkg-b",
        id: "pkg-b in /version of pkg-b",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "react18".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^17.0.0",
        expected: Some("catalog:react18"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "react18".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-b",
        actual: "^18.0.0",
        expected: Some("catalog:react18"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "lodash",
        id: "lodash in /catalogs/react18 of pnpm-workspace.yaml",
        actual: "^4.0.0",
        expected: Some("^4.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_named_catalog_missing_specifier_conflict() {
    // 1 named catalog (`react18`) exists, dep `react` absent. Multi-consumer
    // conflict → MissingFromCatalogAndNonSemverMismatch("react18").
    let yaml = "catalogs:\n  react18:\n    lodash: ^4.0.0\n";
    let ctx = TestBuilder::new()
      .with_pnpm_catalogs(yaml)
      .with_packages(vec![
        json!({"name": "pkg-a", "version": "0.0.0", "dependencies": {"react": "^18.0.0"}}),
        json!({"name": "pkg-b", "version": "0.0.0", "dependencies": {"react": "git+https://github.com/facebook/react.git"}}),
      ])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
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
        state: InstanceState::valid(IsLocalAndValid),
        dependency_name: "pkg-b",
        id: "pkg-b in /version of pkg-b",
        actual: "0.0.0",
        expected: Some("0.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(MissingFromCatalogAndNonSemverMismatch("react18".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(MissingFromCatalogAndNonSemverMismatch("react18".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-b",
        actual: "git+https://github.com/facebook/react.git",
        expected: Some("git+https://github.com/facebook/react.git"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "lodash",
        id: "lodash in /catalogs/react18 of pnpm-workspace.yaml",
        actual: "^4.0.0",
        expected: Some("^4.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }
}

mod ambiguous_catalog {
  use super::*;

  #[tokio::test]
  async fn catalog_group_dep_not_in_any_of_multiple_catalogs() {
    // 2+ catalogs exist, dep `react` defined in 0 of them → cannot infer which
    // catalog the user intended → Unfixable::NotUsingCatalogAndCatalogUnknown.
    let yaml = "catalog:\n  lodash: ^4.0.0\ncatalogs:\n  legacy:\n    underscore: ^1.0.0\n";
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
        state: InstanceState::unfixable(NotUsingCatalogAndCatalogUnknown),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "lodash",
        id: "lodash in /catalog of pnpm-workspace.yaml",
        actual: "^4.0.0",
        expected: Some("^4.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "underscore",
        id: "underscore in /catalogs/legacy of pnpm-workspace.yaml",
        actual: "^1.0.0",
        expected: Some("^1.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_dep_in_multiple_catalogs() {
    // 2+ catalogs exist, dep `react` defined in 2+ of them → ambiguous which
    // catalog to point the consumer at → Unfixable::NotUsingCatalogAndCatalogUnknown.
    let yaml = "catalogs:\n  react17:\n    react: ^17.0.0\n  react18:\n    react: ^18.0.0\n";
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
        state: InstanceState::unfixable(NotUsingCatalogAndCatalogUnknown),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "react",
        id: "react in /catalogs/react17 of pnpm-workspace.yaml",
        actual: "^17.0.0",
        expected: Some("^17.0.0"),
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
  async fn catalog_group_dep_in_multiple_catalogs_bun() {
    // Bun + 2 named catalogs both defining react → NotUsingCatalogAndCatalogUnknown.
    // Mirror of `catalog_group_dep_in_multiple_catalogs`.
    let ctx = TestBuilder::new()
      .with_bun_catalogs(json!({
        "name": "bun-root",
        "catalogs": {
          "react17": {"react": "^17.0.0"},
          "react18": {"react": "^18.0.0"}
        }
      }))
      .with_packages(vec![json!({
        "name": "pkg-a",
        "version": "0.0.0",
        "dependencies": {"react": "^18.0.0"}
      })])
      .with_version_group(json!({
        "label": "enforce catalog",
        "dependencies": ["react"],
        "policy": "catalog",
      }))
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
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "react",
        id: "react in /catalogs/react17 of bun-root",
        actual: "^17.0.0",
        expected: Some("^17.0.0"),
        overridden: None,
        severity: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "react",
        id: "react in /catalogs/react18 of bun-root",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
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
        state: InstanceState::unfixable(NotUsingCatalogAndCatalogUnknown),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }
}

mod cannot_infer_catalog_file {
  use super::*;

  #[tokio::test]
  async fn catalog_group_zero_catalogs_npm() {
    // 0 catalogs configured + PM=npm → Syncpack cannot pick which catalog file
    // to create on fix → Unfixable::CannotInferCatalogFile.
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
        state: InstanceState::unfixable(CannotInferCatalogFile),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_zero_catalogs_yarn() {
    let ctx = TestBuilder::new()
      .with_yarn_package_manager()
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
        state: InstanceState::unfixable(CannotInferCatalogFile),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }

  #[tokio::test]
  async fn catalog_group_zero_catalogs_unknown_pm() {
    let ctx = TestBuilder::new()
      .with_unknown_package_manager()
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
        state: InstanceState::unfixable(CannotInferCatalogFile),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
        severity: None,
      },
    ]);
  }
}

/// Severity tests — opt out of auto-fix per status (issue #216).
/// Catalog permits `NotUsingCatalog`, `MissingFromCatalog`.
mod severity {
  use {super::*, crate::instance::Severity};

  /// Scenario: pnpm yaml has catalog react ^18.0.0; pkg-a uses `^18.0.0`
  /// (real specifier, not `catalog:`) → `NotUsingCatalog("default")` Fixable.
  /// severity downgrades to `Warn`.
  #[tokio::test]
  async fn not_using_catalog_warn() {
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
        "severity": {"NotUsingCatalog": "warn"}
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
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(NotUsingCatalog("default".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: Some(Severity::Warn),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "react",
        id: "react in /catalog of pnpm-workspace.yaml",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
    ]);
  }

  /// `severity: { NotUsingCatalog: "error" }` → `Error`.
  #[tokio::test]
  async fn not_using_catalog_error() {
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
        "severity": {"NotUsingCatalog": "error"}
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
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(NotUsingCatalog("default".to_string())),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: Some(Severity::Error),
      },
      ExpectedInstance {
        state: InstanceState::valid(IsCatalogDefinition),
        dependency_name: "react",
        id: "react in /catalog of pnpm-workspace.yaml",
        actual: "^18.0.0",
        expected: Some("^18.0.0"),
        overridden: None,
        severity: Some(Severity::None),
      },
    ]);
  }

  /// Scenario: zero catalogs configured, PM=pnpm; pkg-a uses real specifier
  /// → MissingFromCatalog{default, ^18.0.0}. severity downgrades to `Warn`.
  #[tokio::test]
  async fn missing_from_catalog_warn() {
    let winning = Specifier::new("^18.0.0");
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
        "severity": {"MissingFromCatalog": "warn"}
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
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: Some(Severity::Warn),
      },
    ]);
  }

  /// `severity: { MissingFromCatalog: "error" }` → `Error`.
  #[tokio::test]
  async fn missing_from_catalog_error() {
    let winning = Specifier::new("^18.0.0");
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
        "severity": {"MissingFromCatalog": "error"}
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
        severity: Some(Severity::None),
      },
      ExpectedInstance {
        state: InstanceState::fixable(MissingFromCatalog {
          catalog_name: "default".to_string(),
          winning_specifier: Rc::clone(&winning),
        }),
        dependency_name: "react",
        id: "react in /dependencies of pkg-a",
        actual: "^18.0.0",
        expected: Some("catalog:"),
        overridden: None,
        severity: Some(Severity::Error),
      },
    ]);
  }

  /// Catalog permits `NotUsingCatalog` and `MissingFromCatalog`. `DiffersToPin`
  /// is a Pinned-only key → `InvalidSeverityKey`.
  #[tokio::test]
  #[should_panic(expected = "InvalidSeverityKey")]
  async fn rejects_invalid_severity_key() {
    let yaml = "catalog:\n  react: ^18.0.0\n";
    let _ctx = TestBuilder::new()
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
        "severity": {"DiffersToPin": "warn"}
      }))
      .run()
      .await;
  }
}
