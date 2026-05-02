# Test patterns

Real, trimmed examples for the common scenarios. Imports and `mod` wrapping omitted for brevity — see [SKILL.md](SKILL.md) for the full preamble.

## Banned

`src/version_group/banned_test.rs`. Removes a dep; refuses to ban a local-package version.

```rust
#[tokio::test]
async fn refuses_to_ban_local_version() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({"name": "package-a", "version": "1.0.0"}),
      json!({"name": "package-b", "dependencies": {"package-a": "1.1.0"}}),
    ])
    .with_version_group(json!({
      "dependencies": ["package-a"],
      "isBanned": true
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
    },
    ExpectedInstance {
      state: InstanceState::suspect(RefuseToBanLocal),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(IsBanned),
      dependency_name: "package-a",
      id: "package-a in /dependencies of package-b",
      actual: "1.1.0",
      expected: Some(""),
      overridden: None,
    },
  ]);
}
```

### Banned with `customTypes`

```rust
#[tokio::test]
async fn removes_instance_with_named_version_string_strategy() {
  let ctx = TestBuilder::new()
    .with_config(json!({
      "customTypes": {
        "packageManager": {
          "strategy": "name@version",
          "path": "packageManager"
        }
      }
    }))
    .with_packages(vec![json!({
      "name": "package-a",
      "version": "1.0.0",
      "packageManager": "pnpm@7.27.0"
    })])
    .with_version_group(json!({
      "dependencies": ["pnpm"],
      "dependencyTypes": ["packageManager"],
      "isBanned": true
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(IsBanned),
      dependency_name: "pnpm",
      id: "pnpm in /packageManager of package-a",
      actual: "7.27.0",
      expected: Some(""),
      overridden: None,
    },
  ]);
}
```

## Pinned

`src/version_group/pinned_test.rs`.

```rust
#[tokio::test]
async fn an_already_pinned_version_is_valid() {
  let ctx = TestBuilder::new()
    .with_package(json!({
      "name": "package-a",
      "version": "1.0.0",
      "devDependencies": {"foo": "1.2.0"}
    }))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "pinVersion": "1.2.0"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIdenticalToPin),
      dependency_name: "foo",
      id: "foo in /devDependencies of package-a",
      actual: "1.2.0",
      expected: Some("1.2.0"),
      overridden: None,
    },
  ]);
}
```

### Pin overrides a semver-group range

```rust
#[tokio::test]
async fn pin_overrides_semver_group_match() {
  let ctx = TestBuilder::new()
    .with_package(json!({
      "name": "package-a",
      "version": "1.0.0",
      "devDependencies": {"foo": "^1.0.0"}
    }))
    .with_semver_group(json!({"range": "^", "dependencies": ["foo"]}))
    .with_version_group(json!({
      "dependencies": ["foo"],
      "pinVersion": "1.0.0"
    }))
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::fixable(PinOverridesSemverRange),
      dependency_name: "foo",
      id: "foo in /devDependencies of package-a",
      actual: "^1.0.0",
      expected: Some("1.0.0"),
      overridden: Some("^1.0.0"),
    },
    ExpectedInstance {
      state: InstanceState::valid(IsLocalAndValid),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "1.0.0",
      expected: Some("1.0.0"),
      overridden: None,
    },
  ]);
}
```

## Same range

`src/version_group/same_range_test.rs`. Use a `local`-ignored group as guard so the local instances don't pollute the assertion.

```rust
#[tokio::test]
async fn ranges_satisfy_each_other() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({"name": "package-a", "dependencies": {"foo": ">=1.0.0"}}),
      json!({"name": "package-b", "dependencies": {"foo": "<=2.0.0"}}),
    ])
    .with_version_groups(vec![
      json!({"dependencyTypes": ["local"], "isIgnored": true}),
      json!({"dependencies": ["foo"], "policy": "sameRange"}),
    ])
    .run()
    .await;
  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-a",
      id: "package-a in /version of package-a",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsIgnored),
      dependency_name: "package-b",
      id: "package-b in /version of package-b",
      actual: "",
      expected: Some(""),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-a",
      actual: ">=1.0.0",
      expected: Some(">=1.0.0"),
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(SatisfiesSameRangeGroup),
      dependency_name: "foo",
      id: "foo in /dependencies of package-b",
      actual: "<=2.0.0",
      expected: Some("<=2.0.0"),
      overridden: None,
    },
  ]);
}
```

For mismatching ranges the consumer instances become `InstanceState::unfixable(SameRangeMismatch)`.

## Pnpm catalogs

`src/version_group/catalog_test.rs`. `with_pnpm_catalogs` injects `pnpm-workspace.yaml`. Catalog definitions are themselves instances under `/catalog of pnpm-workspace.yaml`.

```rust
#[tokio::test]
async fn catalog_group_definition_valid() {
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
      actual: "^18.0.0",
      expected: Some("^18.0.0"),
      overridden: None,
    },
  ]);
}
```

Named catalog: yaml `catalogs:\n  react18:\n    react: ^18.0.0\n`, consumer `"react": "catalog:react18"`, def id `react in /catalogs/react18 of pnpm-workspace.yaml`.

## Bun catalogs

`src/version_group/bun_catalog_test.rs`. Two shapes:

- `with_bun_catalogs(json!({...}))` puts `/catalog`, `/catalogs/{n}` at the synthetic Bun root.
- `with_bun_workspaces_catalogs(json!({...}))` nests them under `/workspaces/`.

These tests often use sync `.build()` — they're checking discovery wiring, not the visit pipeline:

```rust
#[test]
fn bun_top_level_catalog_definition_emits_instance() {
  let ctx = TestBuilder::new()
    .with_bun_catalogs(json!({
      "name": "bun-root",
      "catalog": {"react": "^18.0.0"}
    }))
    .with_packages(vec![json!({"name": "pkg-a", "version": "0.0.0"})])
    .build();
  let react = ctx
    .instances
    .iter()
    .find(|i| i.descriptor.name == "react")
    .expect("expected a react bun catalog instance");
  assert!(react.is_catalog_instance());
  assert_eq!(react.descriptor.specifier.get_raw(), "^18.0.0");
}
```

Full validation pass (consumer + def states):

```rust
#[test]
fn bun_catalog_definition_with_consumer_valid() {
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
    .build_and_visit_packages();
  // assert states via expect(&ctx).to_have_instances(...) as elsewhere
}
```

## Semver groups

A semver group rewrites the *expected* range; mismatches are `SemverRangeMismatch` (fixable). Combine with version groups to test interaction:

```rust
#[tokio::test]
async fn semver_group_targets_one_package_only() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({"name": "package-a", "dependencies": {"foo": ">=1.0.0"}}),
      json!({"name": "package-b", "dependencies": {"foo": "^1.2.3"}}),
    ])
    .with_semver_group(json!({"packages": ["package-b"], "range": "^"}))
    .with_version_groups(vec![
      json!({"dependencyTypes": ["local"], "isIgnored": true}),
      json!({"dependencies": ["foo"], "policy": "sameRange"}),
    ])
    .run()
    .await;
  // package-b's `^1.2.3` matches the `^` semver group → no mismatch
  // assert SatisfiesSameRangeGroup on both consumer instances
}
```

## Registry updates

`with_registry_updates` mocks the npm registry and implies subcommand `update`. Optional `with_update_target` bounds it.

```rust
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
```

Bound the update target:

```rust
let ctx = TestBuilder::new()
  .with_package(json!({"name": "package-a", "dependencies": {"wat": "1.2.3"}}))
  .with_update_target(UpdateTarget::Minor)
  .with_registry_updates(json!({"wat": ["1.2.3", "1.3.4", "2.0.0"]}))
  .run()
  .await;
// wat → 1.3.4 (minor cap), not 2.0.0
```

`UpdateTarget` import: `use crate::cli::UpdateTarget;`.
