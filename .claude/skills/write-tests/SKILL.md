---
name: write-tests
description: Write tests for Syncpack using the TestBuilder pattern. Use when adding tests for commands, validation logic, or any new functionality. Covers TestBuilder API, assertion patterns, and common test scenarios.
---

# Write Tests

## Golden rules

- Use `TestBuilder` — never construct `Context` manually.
- Assert with `expect(&ctx).to_have_instances(vec![...])` — never index `ctx.instances`.
- Async-first: `#[tokio::test]` + `.run().await` is the canonical entry.

## TDD

1. Read 2-3 tests in the same `*_test.rs` and copy the pattern
2. Write failing test → `just test` → confirm RED
3. Ask before implementing
4. Implement minimal code → GREEN → `just format`

## Quick start

```rust
use {
  crate::{
    instance::{FixableInstance::*, InstanceState, SuspectInstance::*, UnfixableInstance::*, ValidInstance::*},
    test::{
      builder::TestBuilder,
      expect::{expect, ExpectedInstance},
    },
  },
  serde_json::json,
};

#[tokio::test]
async fn pinned_version_replaces_anything_different() {
  let ctx = TestBuilder::new()
    .with_package(json!({
      "name": "package-a",
      "version": "1.0.0",
      "devDependencies": {"foo": "workspace:*"}
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
      state: InstanceState::fixable(DiffersToPin),
      dependency_name: "foo",
      id: "foo in /devDependencies of package-a",
      actual: "workspace:*",
      expected: Some("1.2.0"),
      overridden: None,
    },
  ]);
}
```

## Sub-module organisation

Group related scenarios under nested modules — see `pinned_test.rs`, `catalog_defs_test.rs`:

```rust
mod local {
  use super::*;
  #[tokio::test] async fn refuses_to_pin_local_version() { ... }
}

mod normal {
  use super::*;
  #[tokio::test] async fn an_already_pinned_version_is_valid() { ... }
}

mod registry_updates {
  use super::*;
  #[tokio::test] async fn def_marked_outdated_when_registry_has_newer_version() { ... }
}
```

## File location

| Test type                                    | Location                                    |
| -------------------------------------------- | ------------------------------------------- |
| Version-group behaviour (pin, ban, ranges …) | `src/version_group/<group>_test.rs`         |
| Catalog discovery wiring                     | `src/version_group/{catalog,bun_catalog}_test.rs` |
| Fix mutations                                | `src/commands/fix_test.rs`                  |
| Format pass                                  | `src/visit_formatting/format_test.rs`       |
| Other unit tests                             | Co-located: `src/foo.rs` ↔ `src/foo_test.rs` |

## Builder methods

Source of truth: `src/test/builder.rs`.

| Method                                                  | Purpose                                                              |
| ------------------------------------------------------- | -------------------------------------------------------------------- |
| `.with_package(json!({...}))`                           | Add one package.json                                                 |
| `.with_packages(vec![...])`                             | Add many                                                             |
| `.with_version_group(json!({...}))`                     | Add one version group                                                |
| `.with_version_groups(vec![...])`                       | Add many                                                             |
| `.with_semver_group(json!({...}))`                      | Add a semver group                                                   |
| `.with_config(json!({...}))`                            | Base config (e.g. `customTypes`, `dependencyGroups`)                 |
| `.with_strict(bool)`                                    | Strict mode (Suspect → error)                                        |
| `.with_subcommand("update")`                            | Override subcommand (default: `lint`, or `update` if registry set)   |
| `.with_pnpm_catalogs(yaml)`                             | Inject `pnpm-workspace.yaml`; implies pnpm PM                        |
| `.with_bun_catalogs(json!({...}))`                      | Synthetic Bun root with `/catalog`, `/catalogs/{n}`; implies Bun PM  |
| `.with_bun_workspaces_catalogs(json!({...}))`           | Same, nested under `/workspaces/`                                    |
| `.with_{pnpm,bun,npm,yarn,unknown}_package_manager()`   | Force PM detection                                                   |
| `.with_registry_updates(json!({"react":[...]}))`        | Mock npm registry; implies subcommand=`update`                       |
| `.with_update_target(UpdateTarget::Minor)`              | Bound update target                                                  |
| `.run().await` → `Context`                              | **Primary** end-to-end (full pipeline through disk + discovery)      |
| `.build()`                                              | Sync, no visit — context-wiring tests                                |
| `.build_and_visit_packages()`                           | Sync + visit_packages — fix tests, older suites                      |
| `.build_and_visit_formatting()`                         | Sync + visit_formatting                                              |
| `.build_with_registry_and_visit().await`                | Sync wiring + async registry mock + visit                            |

## ExpectedInstance fields

```rust
ExpectedInstance {
  state: InstanceState::fixable(DiffersToPin),  // valid / fixable / unfixable / suspect
  dependency_name: "react",                     // = `internal_name` (alias-aware)
  id: "react in /dependencies of package-a",    // {dep} in {/path} of {package_or_yaml}
  actual: "17.0.0",                             // raw specifier on disk
  expected: Some("18.0.0"),                     // None = ignore; Some("") = remove
  overridden: None,                             // semver-group override target, if any
}
```

`id` location examples:

- `/dependencies`, `/devDependencies`, `/peerDependencies`
- `/version of package-a` (local version)
- `/packageManager`, `/engines/node`
- `/catalog of pnpm-workspace.yaml`, `/catalogs/<name> of pnpm-workspace.yaml`
- `/customVersion`, `/custom/config/version` (via `customTypes`)

## Patterns

→ [patterns.md](patterns.md): banned, pinned, sameRange, pnpm catalogs, bun catalogs, semver ranges, registry updates.

## Fix tests

`src/commands/fix_test.rs` builds with `.build_and_visit_packages()` then runs `fix::run(ctx, &SilentReporter, &disk)`. `dry_run = true` is the default (set by `mock::config_from_mock`), so `is_dirty()` and post-fix contents stay observable. Set `ctx.config.cli.dry_run = false` only when asserting writes through a recording `MockDiskIo` (see `pnpm_fix_writes_yaml_to_disk`).

## Common mistakes

| Wrong                                            | Right                                                                                  |
| ------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `#[test] fn foo()` + `.run().await`              | `#[tokio::test] async fn foo()`                                                        |
| `use crate::instance_state::*`                   | `use crate::instance::*`                                                               |
| `"pinned": "1.0.0"`                              | `"pinVersion": "1.0.0"`                                                                |
| `Context { ... }`                                | `TestBuilder::new()...`                                                                |
| `ctx.instances[0]`                               | `expect(&ctx).to_have_instances(vec![...])`                                            |
| `.build()` then check states                     | `.run().await` (or `.build_and_visit_packages()` for sync)                             |
| Missing `SuspectInstance::*` import              | Import all 4: `FixableInstance::*, ValidInstance::*, SuspectInstance::*, UnfixableInstance::*` |

## Running

```bash
just test                            # all
cargo test pinned_test               # pattern match
cargo test test_name -- --nocapture  # with stdout
```

## Reference tests

- `src/version_group/banned_test.rs` — banned + custom types
- `src/version_group/pinned_test.rs` — sub-modules, semver-group interaction
- `src/version_group/same_range_test.rs` — range satisfaction
- `src/version_group/catalog_test.rs` — pnpm catalogs
- `src/version_group/bun_catalog_test.rs` — bun catalogs (sync `.build()`)
- `src/version_group/preferred_semver_test.rs` — registry updates, update targets
