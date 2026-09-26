---
name: write-tests
description: Write tests for Syncpack using the TestBuilder pattern. Use when adding tests for commands, validation logic, or any new functionality. Covers TestBuilder API, assertion patterns, and common test scenarios.
---

# Write Tests

## Golden rules

- Use `TestBuilder` — never construct `Discovered` / `Inspected` manually.
- Assert with `expect(&ctx).to_have_instances(vec![...])` — never index `ctx.data.instances`. To read one instance's state or expected specifier, use `ctx.instance_views()`.
- Async-first: `#[tokio::test]` + `.run().await` is the canonical entry.
- Default to full-pipeline `TestBuilder` tests. Write a unit test only where the pipeline can't reach.

## TDD

1. Read 2-3 tests in the same `*_test.rs` and copy the pattern
2. Write failing test → `just test` → confirm RED
3. Ask before implementing
4. Implement minimal code → GREEN → `just format`

## Quick start

```rust
use {
  crate::test::{
    builder::TestBuilder,
    expect::{expect, ExpectedInstance},
  },
  serde_json::json,
  syncpack_model::instance::{FixableInstance::*, InstanceState, SuspectInstance::*, UnfixableInstance::*, ValidInstance::*},
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
      severity: None,
    },
    ExpectedInstance {
      state: InstanceState::fixable(DiffersToPin),
      dependency_name: "foo",
      id: "foo in /devDependencies of package-a",
      actual: "workspace:*",
      expected: Some("1.2.0"),
      overridden: None,
      severity: None,
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

The version-group and format suites test code in `crates/syncpack-rules` but live in the bin, so register each new file in `src/main.rs` (`#[cfg(test)] #[path = "version_group/foo_test.rs"] mod foo_test;`).

## Builder methods

Source of truth: `src/test/builder.rs`.

| Method                                                  | Purpose                                                              |
| ------------------------------------------------------- | -------------------------------------------------------------------- |
| `.with_package(json!({...}))`                           | Add one package.json                                                 |
| `.with_packages(vec![...])`                             | Add many                                                             |
| `.with_version_group(json!({...}))`                     | Add one version group                                                |
| `.with_version_groups(vec![...])`                       | Add many                                                             |
| `.with_semver_group(json!({...}))`                      | Add a semver group                                                   |
| `.with_update_group(json!({...}))`                      | Add an update group                                                  |
| `.with_config(json!({...}))`                            | Base config (e.g. `customTypes`, `dependencyGroups`), written as `.syncpackrc` |
| `.with_yaml_config("...")`                              | Rcfile as `.syncpackrc.yaml` — exercises the yaml load path          |
| `.with_strict(bool)`                                    | Strict mode (Suspect → error)                                        |
| `.with_subcommand("update")`                            | Override subcommand (default: `lint`, or `update` if registry set)   |
| `.with_pnpm_catalogs(yaml)`                             | Inject `pnpm-workspace.yaml`; implies pnpm PM                        |
| `.with_bun_catalogs(json!({...}))`                      | Synthetic Bun root with `/catalog`, `/catalogs/{n}`; implies Bun PM  |
| `.with_bun_workspaces_catalogs(json!({...}))`           | Same, nested under `/workspaces/`                                    |
| `.with_{pnpm,bun,npm,yarn,unknown}_package_manager()`   | Force PM detection                                                   |
| `.with_registry_updates(json!({"react":[...]}))`        | Mock npm registry; implies subcommand=`update`                       |
| `.with_update_target(UpdateTarget::Minor)`              | Bound update target                                                  |
| `.run().await` → `Inspected`                            | **Primary** end-to-end (full pipeline: disk → rcfile → discovery → inspection) |
| `.build()` → `Discovered`                               | Sync, no inspection — discovery-wiring tests                         |
| `.try_build()` → `Result<Discovered, DiscoveryError>`   | Config rejections: assert with `.unwrap_err()`                       |
| `.build_and_visit_packages()` → `Inspected`             | Sync + inspection — fix tests, older suites                          |
| `.build_with_registry_and_visit().await` → `Inspected`  | Sync wiring + async registry mock + inspection                       |

`.run()` and `.build_and_visit_packages()` already run the formatting pass; assert on it with `to_have_format_mismatches` (see Format tests).

## ExpectedInstance fields

```rust
ExpectedInstance {
  state: InstanceState::fixable(DiffersToPin),  // valid / fixable / unfixable / suspect
  dependency_name: "react",                     // = `internal_name` (alias-aware)
  id: "react in /dependencies of package-a",    // {dep} in {/path} of {package_or_yaml}
  actual: "17.0.0",                             // raw specifier on disk
  expected: Some("18.0.0"),                     // None = ignore; Some("") = remove
  overridden: None,                             // semver-group override target, if any
  severity: None,                               // None = do not assert; Some(Severity::…) asserts
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

## Format tests

`src/visit_formatting/format_test.rs`. Each test switches off every formatting rule except the one under test (`rcfile_with(keep, overrides)` — `keep` leaves a rule at its rcfile default), then asserts the mismatches for one package:

```rust
expect(&ctx).to_have_format_mismatches(
  "a",
  vec![ExpectedFormatMismatch {
    variant: ExportsPropertyIsNotSorted,
    property_path: "/exports",
    expected: json!({ "import": "./index.js", "default": "./default.js" }),
  }],
);
```

`expected` is compared including key order (`serde_json` with `preserve_order` ignores order in `Value ==`). An empty `vec![]` asserts no mismatches.

## Fix tests

`src/commands/fix_test.rs` builds with `.build_and_visit_packages()` then runs `fix::run(inspected, &SilentReporter, &disk)` with `syncpack_io::mock::MockDiskIo`. `dry_run = true` is the default (set by `mock::config_from_mock`), so `file.dirty` and post-fix contents stay observable. Set `ctx.data.config.cli.dry_run = false` only when asserting writes through a recording `MockDiskIo` (see `pnpm_fix_writes_yaml_to_disk`).

## Common mistakes

| Wrong                                            | Right                                                                                  |
| ------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `#[test] fn foo()` + `.run().await`              | `#[tokio::test] async fn foo()`                                                        |
| `use crate::instance::*` / `crate::instance_state::*` | `use syncpack_model::instance::*`                                                 |
| `"pinned": "1.0.0"`                              | `"pinVersion": "1.0.0"`                                                                |
| `Discovered { ... }` / `Inspected { ... }`       | `TestBuilder::new()...`                                                                |
| `ctx.data.instances[0]`                          | `expect(&ctx).to_have_instances(vec![...])`                                            |
| `ctx.inspection.states[i]` / `expected[i]`       | `ctx.instance_views()`                                                                 |
| `.build()` then check states                     | `.run().await` (or `.build_and_visit_packages()` for sync)                             |
| `assert_eq!(value_a, value_b)` for key order     | `to_have_format_mismatches` — `Value ==` ignores key order under `preserve_order`      |
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
- `src/visit_formatting/format_test.rs` — formatting rules, order-sensitive assertions
- `src/rcfile_loader_test.rs` — rcfile / pnpm-yaml config resolution through the pipeline
