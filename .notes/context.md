# Syncpack Architecture

Rust CLI for synchronising npm dependency versions across JavaScript monorepos.

The workspace is a binary crate (`src/`) plus 8 library crates (`crates/`):
`syncpack-specifier` → `syncpack-vocab` → `syncpack-config` / `syncpack-model` →
`syncpack-disk` / `syncpack-registry` / `syncpack-rules`, with `syncpack-io` as a
second, unrelated leaf. `syncpack-rules` is the pure inspection engine and names
no config/disk type — see `RuleInputCfg`/`FormatConfig` in
`crates/syncpack-rules/src/inspection.rs`.

## Mental Model

```
CLI → Read config + package.json files → Discovered
                                              ↓
                    inspect() assigns an InstanceState to each instance,
                    returned as an Inspection value (never mutates Discovered)
                                              ↓
                              Command processes Inspected → Exit 0 or 1
```

Every dependency occurrence (e.g., React in package-a's dependencies) is an **Instance** tagged with a state describing whether it follows the rules.

## Three-Phase Pipeline

Every command follows this pattern (`src/syncpack.rs`):

1. **`analyse()`** — Read config, find packages, collect dependencies as Instances → `Discovered`
2. **`inspect()`** — `visit_packages()` or `visit_formatting()` assigns states, returned as an `Inspection` value → `Inspected { data: Discovered, inspection: Inspection, updates }`
3. **`run()`** — Command iterates `Inspected`, performs side effects, returns exit code

## Critical Types

### Discovered / Inspected (`src/discovery/mod.rs`)

`Discovered` owns all project data and is immutable once built. `inspect()` reads
it and returns an `Inspection` value instead of mutating it; `Inspected` binds the
two together so a caller can't read results early or pair the wrong `Inspection`
with the wrong data.

```rust
pub struct Discovered {
    pub config: Config,                    // CLI args + config file
    pub disk: Disk,                        // All package.json / pnpm-workspace.yaml files
    pub instances: Vec<Instance>,          // Every dependency occurrence
    pub dependencies: Vec<Dependency>,     // Global dependency arena
    pub sources: Sources,
    pub version_groups: Vec<VersionGroup>, // Rules/policies
}

pub struct Inspected {
    pub data: Discovered,
    pub inspection: Inspection,
    pub updates: Option<RegistryUpdates>,
}
```

### Instance (`crates/syncpack-model/src/instance/`)

Single dependency occurrence. Immutable — no interior mutability. Its resolved
state lives in `Inspection.states`, indexed by `InstanceIdx`, not on the
`Instance` itself.

### InstanceState (`crates/syncpack-model/src/instance/instance_state.rs`)

```
InstanceState
├── Unknown              // Not yet inspected
├── Valid(...)           // Passes rules
├── Invalid
│   ├── Fixable(...)     // Can auto-fix
│   ├── Unfixable(...)   // Needs human decision
│   └── Conflict(...)    // Conflicting rules
└── Suspect(...)         // Misconfiguration
```

To see all variants:

```bash
ast-grep -p 'pub enum ValidInstance' crates/syncpack-model/
ast-grep -p 'pub enum FixableInstance' crates/syncpack-model/
ast-grep -p 'pub enum UnfixableInstance' crates/syncpack-model/
ast-grep -p 'pub enum SuspectInstance' crates/syncpack-model/
```

### VersionGroup (`crates/syncpack-rules/src/version_group.rs`)

Versioning policies. To see variants:

```bash
ast-grep -p 'pub enum VersionGroupBehavior' crates/syncpack-rules/
```

## Finding Files

Use `find_path` or `ast-grep` to locate files. Key entry points:

| Purpose          | File                                          |
| ---------------- | ---------------------------------------------- |
| Entry & dispatch | `src/main.rs`                                  |
| Pipeline glue    | `src/syncpack.rs`                              |
| Central data     | `src/discovery/mod.rs` (`Discovered`/`Inspected`) |
| State assignment | `crates/syncpack-rules/src/visit_packages.rs`  |
| Validation logic | `crates/syncpack-rules/src/version_group/*.rs` |
| Commands         | `src/commands/*.rs`                            |
| Tests            | `src/version_group/*_test.rs`, `src/instance/*_test.rs` (wired into `src/main.rs` via `#[path]`) |

## Key Patterns

### Command Structure

```rust
pub fn run(inspected: Inspected) -> i32 {
    let Inspected { data, inspection, .. } = inspected;
    data.version_groups.iter().for_each(|group| {
        commands::get_sorted_dependencies(group, &data.dependencies, &data.config.cli.sort).iter().for_each(|&dep_idx| {
            data.dependencies[dep_idx.0].instances.iter()
                .filter(|idx| inspection.states[idx.0].is_invalid())
                .for_each(|idx| { /* process */ });
        });
    });
    if has_issues { 1 } else { 0 }
}
```

### Visitor Choice

- **visit_packages** — Dependency version validation (lint, fix, update, list, json)
- **visit_formatting** — package.json structure (format)

### Ownership

- `Rc<Specifier>` for single-threaded sharing of parsed versions
- `Arc` only for cross-thread (RegistryClient)
- `Discovered` is immutable once built; `inspect()` returns a fresh `Inspection` rather than mutating it. Only `apply` (in the bin, at fix/format time) mutates `disk`.

## Location String Format

Instance IDs follow this pattern:

```
{dependency} in {location} of {package}
```

Examples:

- `react in /dependencies of pkg-a`
- `lodash in /devDependencies of pkg-b`
- `pnpm in /packageManager of pkg-c`
