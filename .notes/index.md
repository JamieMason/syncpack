# Syncpack Development Hub

<!-- AI ASSISTANTS: This is the central navigation hub for Syncpack development.
     Always read this file first to understand the codebase structure and find
     the right documentation for your task. -->

**Syncpack** is a Rust CLI tool that synchronizes npm dependency versions across monorepos. It ensures consistency and prevents version conflicts by analyzing package.json files and enforcing version alignment rules.

## Quick Start for AI Assistants

**ALWAYS read these first:**

1. `.notes/context.md` - Essential mental model and core concepts
2. This file - Navigation and task guidance
3. `.notes/quick-reference.md` - Syntax lookup when needed

**Architecture:** Every command follows Create Context → Inspect Context → Run Command

## Task-Oriented Navigation

### "I need to add or modify validation logic"

- **Location:** `src/visit_packages/*.rs`
- **Step-by-step guide:** `.notes/examples/adding-instance-state.md`
- **Tests:** `src/visit_packages/*_test.rs`
- **State definitions:** `src/instance_state.rs`
- **Integration point:** `src/visit_packages.rs`

**Example files to study:**

- `src/visit_packages/banned.rs` - Simple validation
- `src/visit_packages/pinned.rs` - Version comparison
- `src/visit_packages/preferred_semver.rs` - Complex semver logic

### "I need to add or modify a command"

- **Location:** `src/commands/*.rs`
- **Step-by-step guide:** `.notes/examples/adding-command.md`
- **Three places to update:**
  - `src/cli.rs` (add enum variant)
  - `src/main.rs` (add dispatch match arm)
  - `src/commands.rs` (register module)

**Example files to study:**

- `src/commands/lint.rs` - Simple reporting command
- `src/commands/fix.rs` - File modification command
- `src/commands/update.rs` - Registry interaction command

### "I need to write tests"

- **Preferred location:** `src/visit_packages/*_test.rs`
- **Alternative:** Co-located as `*_test.rs`
- **Comprehensive guide:** `.notes/examples/writing-tests.md`
- **Test utilities:** `src/test/builder.rs`, `src/test/expect.rs`

**Always use TestBuilder pattern:**

```rust
let ctx = TestBuilder::new()
    .with_packages(vec![json!({"name": "pkg", "dependencies": {"react": "17.0.0"}})])
    .with_version_group(json!({"dependencies": ["react"], "pinned": "18.0.0"}))
    .build_and_visit_packages();
```

### "I need syntax reference or quick lookup"

- **Quick lookup:** `.notes/quick-reference.md`
- **All InstanceState variants:** 14 Valid, 7 Fixable, 3 Unfixable, 5 Suspect
- **State checking methods:** `is_valid()`, `is_invalid()`, `is_fixable()`, etc.
- **Common code patterns:** Iteration, filtering, TestBuilder usage

### "I need to make a design decision"

- **Decision flowcharts:** `.notes/decision-trees.md`
- **Key decisions:**
  - Should I use `visit_packages` or `visit_formatting`?
  - What InstanceState variant should I use?
  - Where should I add my test?
  - Should I create a new command or extend existing?

### "I need to understand the architecture"

- **High-level overview:** `CONTRIBUTING.md`
- **Visual diagrams:** `.notes/diagrams.md`
- **Core concepts:** `.notes/context.md`
- **Design patterns:** `.notes/patterns.md`

## Core Data Structures

### Context (`src/context.rs`)

The central struct that owns all project data:

- `config: Config` - Combined CLI + config file settings
- `packages: Packages` - All package.json files
- `instances: Vec<Rc<Instance>>` - Every dependency occurrence
- `version_groups: Vec<VersionGroup>` - Versioning policies

**Ownership rule:** Created once, passed through pipeline, commands consume it.

### Instance (`src/instance.rs`)

Represents a single dependency occurrence (e.g., `react@18.0.0` in package A's dependencies):

- Has `state: InstanceState` assigned during inspection
- Contains references to dependency, package, and version group

### InstanceState (`src/instance_state.rs`)

Tagged enum describing validation status:

- `Valid` - 14 variants (IsLocalAndValid, IsPinned, etc.)
- `Invalid::Fixable` - Can auto-fix (IsBanned, DiffersToLocal, etc.)
- `Invalid::Unfixable` - Needs human decision
- `Invalid::Conflict` - Semver group conflicts with version group
- `Suspect` - Misconfiguration detected

### VersionGroup (`src/version_group.rs`)

Defines versioning policies:

- `Banned` - Should not exist
- `Pinned` - Use exact version
- `HighestSemver` / `LowestSemver` - Use highest/lowest found
- `SameRange` - Compatible version ranges
- `SnappedTo` - Follow another package's version

## File Organization Quick Reference

### Entry Points

- **Main:** `src/main.rs`
- **CLI parsing:** `src/cli.rs`
- **Config:** `src/config.rs`, `src/rcfile.rs`

### Core Logic

- **Context:** `src/context.rs`
- **Instance:** `src/instance.rs`
- **InstanceState:** `src/instance_state.rs`
- **VersionGroup:** `src/version_group.rs`
- **Packages:** `src/packages.rs`, `src/package_json.rs`

### Validation Logic

- **Entry point:** `src/visit_packages.rs`
- **Banned deps:** `src/visit_packages/banned.rs`
- **Pinned versions:** `src/visit_packages/pinned.rs`
- **Semver logic:** `src/visit_packages/preferred_semver.rs`
- **Formatting:** `src/visit_formatting.rs`

### Commands

- **Lint:** `src/commands/lint.rs`
- **Fix:** `src/commands/fix.rs`
- **Format:** `src/commands/format.rs`
- **Update:** `src/commands/update.rs`
- **List:** `src/commands/list.rs`
- **JSON:** `src/commands/json.rs`

### Testing Infrastructure

- **TestBuilder:** `src/test/builder.rs`
- **Assertions:** `src/test/expect.rs`
- **Mocks:** `src/test/mock.rs`

## Common Development Patterns

### Standard Command Structure

```rust
pub fn run(ctx: Context) -> i32 {
    let mut has_issues = false;

    ctx.get_version_groups().for_each(|group| {
        group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
            dependency.get_sorted_instances()
                .filter(|instance| instance.is_invalid())
                .for_each(|instance| {
                    // Handle instance
                    has_issues = true;
                });
        });
    });

    if has_issues { 1 } else { 0 }
}
```

### Standard Test Structure

```rust
#[test]
fn test_descriptive_name() {
    let ctx = TestBuilder::new()
        .with_packages(vec![...])
        .with_version_group(json!({...}))
        .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
            state: InstanceState::fixable(DiffersToPinnedVersion),
            dependency_name: "react",
            // ...
        },
    ]);
}
```

### Common Iteration Pattern

Most commands iterate: version groups → dependencies → instances

```rust
ctx.get_version_groups()
    .for_each(|group| {
        group.get_sorted_dependencies(&sort_by)
            .for_each(|dependency| {
                dependency.get_sorted_instances()
                    .filter(|instance| instance.is_invalid())
                    .for_each(|instance| { /* process */ });
            });
    });
```

## Step-by-Step Task Guides

### Adding a New InstanceState Variant

1. Read `.notes/examples/adding-instance-state.md`
2. Add variant to enum in `src/instance_state.rs`
3. Implement `get_severity()` (0-100, higher = more severe)
4. Add detection logic in appropriate `src/visit_packages/*.rs` file
5. Integrate in `src/visit_packages.rs`
6. Write tests in `src/visit_packages/*_test.rs`

### Adding a New Command

1. Read `.notes/examples/adding-command.md`
2. Create `src/commands/my_command.rs` with `pub fn run(ctx: Context) -> i32`
3. Add variant to `Subcommand` enum in `src/cli.rs`
4. Add match arm in `src/main.rs`
5. Update `src/commands.rs` module registration
6. Choose `visit_packages()` or `visit_formatting()`
7. Write tests

### Adding a New Version Group Type

1. Add variant to `VersionGroupVariant` in `src/version_group.rs`
2. Implement validation logic in `src/visit_packages.rs`
3. Update config schema in `src/rcfile.rs`
4. Write integration tests using TestBuilder

## Development Workflow

### Running Tests

```bash
just test                              # All tests
cargo test test_name -- --nocapture   # Specific test with output
just coverage                          # Coverage report
```

### Local Testing

```bash
# Run against test fixture
cd fixtures/fluid-framework
cargo run -- lint
cargo run -- fix --dry-run

# Basic commands
cargo run -- --help
cargo run -- lint --help
```

### Development Tools

```bash
just format     # Format code
just lint       # Run all checks
just benchmark  # Performance testing
```

## Data Flow Architecture

### 1. Create Context (Read-Only)

Order matters: CLI Args → Config File → package.json Files → Collect Dependencies

### 2. Inspect Context

Choose visitor based on command:

- **visit_packages** (`src/visit_packages.rs`) - For version validation (lint, fix, update, list, json)
- **visit_formatting** (`src/visit_formatting.rs`) - For package.json structure (format command)

### 3. Run Command

Each command in `src/commands/*.rs` takes ownership of Context, performs side effects, returns exit code.

## Naming Conventions

- **Files:** snake_case.rs (instance_state.rs)
- **Test files:** \*\_test.rs (co-located with source)
- **Types:** PascalCase (Context, InstanceState)
- **Enum variants:** PascalCase (IsBanned, IsLocalAndValid)
- **Functions:** snake_case (visit_packages, is_invalid)
- **Predicates:** is*\* or has*\* (is_fixable, has_issues)
- **Booleans:** is*\*, has*_, should\__

## Key Performance Patterns

- **Rc vs Arc:** Use `Rc<Instance>` for single-threaded sharing, `Arc` only for cross-thread data
- **Cloning:** Context fields use `Rc` to avoid expensive clones
- **Registry fetching:** Only instantiate `RegistryClient` for `update` command

## Error Handling Patterns

- Use `Result<T, E>` for operations that can fail
- Custom error types in dedicated files (e.g., `src/rcfile/error.rs`)
- Use `thiserror` crate for deriving error implementations
- Log with `log` crate: `debug!()`, `info!()`, `error!()`

## Documentation Hierarchy

```
.cursorrules (AI behavioral rules)
    ↓
.notes/context.md (essential concepts)
    ↓
.notes/index.md (this file - navigation hub)
    ↓
.notes/examples/*.md (step-by-step guides)
    ↓
.notes/quick-reference.md (syntax lookup)
    ↓
.notes/decision-trees.md (decision guidance)
    ↓
.notes/patterns.md (design rationale)
    ↓
Source code (actual implementation)
```

## Finding Code with grep/ast-grep

See `.notes/search-tips.md` for comprehensive search guide.

**Quick examples:**

```bash
# Find state assignments
ast-grep --pattern 'InstanceState::fixable' --lang rust src/

# Find command signatures
ast-grep --pattern 'pub fn run' --lang rust src/commands/

# Find tests
ast-grep --pattern '#[test]' --lang rust src/

# Find Context creation
ast-grep --pattern 'Context::create' --lang rust src/
```

## When You're Stuck

1. **Check this index** - Find the right document for your task
2. **Check `.notes/decision-trees.md`** - Use flowcharts for guidance
3. **Check `.notes/context.md`** - Understand the mental model and data flow
4. **Check `.notes/quick-reference.md`** - Look up InstanceState variants and syntax
5. **Search existing code** - Use grep or look at similar implementations
6. **Look at test examples** - Check `src/visit_packages/*_test.rs` for real patterns
7. **Trace the data flow** - Context creation → inspection → command execution

## Debugging Tips

1. **Add debug logging:** `log::debug!("{:#?}", value)`
2. **Run single test:** `cargo test test_name -- --nocapture`
3. **Check generated Context:** Print `ctx` in tests before assertions
4. **Use coverage:** `just coverage` to find untested code paths
5. **Test against fixture:** Use `fixtures/fluid-framework` for realistic scenarios

## Git Workflow

- **`main`** - Most recently published Rust v14 alpha
- **`v14-alpha`** - Development branch for next version
- **`13.x.x`** - Legacy TypeScript version (being replaced)

Target `v14-alpha` for new features and bug fixes.

## Additional Resources

- **Live documentation:** https://jamiemason.github.io/syncpack/
- **Test fixture:** `./fixtures/fluid-framework` - Real monorepo for testing
- **Architecture overview:** `CONTRIBUTING.md`
- **Project rules:** `.cursorrules`
