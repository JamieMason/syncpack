# Syncpack Architecture

Rust CLI for synchronising npm dependency versions across JavaScript monorepos.

## Mental Model

```
CLI → Read config + package.json files → Create Context
                                              ↓
                              Assign InstanceState to each dependency
                                              ↓
                              Command processes states → Exit 0 or 1
```

Every dependency occurrence (e.g., React in package-a's dependencies) is an **Instance** tagged with a state describing whether it follows the rules.

## Three-Phase Pipeline

Every command follows this pattern:

1. **Create** — Read config, find packages, collect dependencies as Instances
2. **Inspect** — `visit_packages()` or `visit_formatting()` assigns states
3. **Run** — Command iterates states, performs side effects, returns exit code

## Critical Types

### Context (`src/context.rs`)

Owns all project data. Created once, passed through pipeline, consumed by command.

```rust
pub struct Context {
    pub config: Config,              // CLI args + config file
    pub packages: Packages,          // All package.json files
    pub instances: Vec<Rc<Instance>>, // Every dependency occurrence
    pub version_groups: Vec<VersionGroup>, // Rules/policies
}
```

### Instance (`src/instance.rs`)

Single dependency occurrence with validation state.

### InstanceState (`src/instance_state.rs`)

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
ast-grep -p 'pub enum ValidInstance' src/
ast-grep -p 'pub enum FixableInstance' src/
ast-grep -p 'pub enum UnfixableInstance' src/
ast-grep -p 'pub enum SuspectInstance' src/
```

### VersionGroup (`src/version_group.rs`)

Versioning policies. To see variants:
```bash
ast-grep -p 'pub enum VersionGroupVariant' src/
```

## Finding Files

Use `find_path` or `ast-grep` to locate files. Key entry points:

| Purpose | File |
|---------|------|
| Entry & dispatch | `src/main.rs` |
| Central data | `src/context.rs` |
| State assignment | `src/visit_packages.rs` |
| Validation logic | `src/visit_packages/*.rs` |
| Commands | `src/commands/*.rs` |
| Tests | `src/visit_packages/*_test.rs` |

## Key Patterns

### Command Structure

```rust
pub fn run(ctx: Context) -> i32 {
    ctx.version_groups.iter().for_each(|group| {
        group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
            dependency.get_sorted_instances()
                .filter(|instance| instance.is_invalid())
                .for_each(|instance| { /* process */ });
        });
    });
    if has_issues { 1 } else { 0 }
}
```

### Visitor Choice

- **visit_packages** — Dependency version validation (lint, fix, update, list, json)
- **visit_formatting** — package.json structure (format)

### Ownership

- `Rc<Instance>` for single-threaded sharing
- `Arc` only for cross-thread (RegistryClient)
- Context flows through phases, commands consume it

## Location String Format

Instance IDs follow this pattern:
```
{dependency} in {location} of {package}
```

Examples:
- `react in /dependencies of pkg-a`
- `lodash in /devDependencies of pkg-b`
- `pnpm in /packageManager of pkg-c`
