# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What is Syncpack?

Syncpack is a Rust CLI tool that ensures consistent dependency versions across JavaScript monorepos. It validates, fixes, formats, and updates npm dependency versions in multiple package.json files according to user-defined rules.

**Current Status:** Version 14.0.0-alpha (Rust rewrite replacing TypeScript v13)

## Essential Reading (Read in This Order)

Before working on this codebase, read these files in order:

1. **`.notes/context.md`** - Core concepts and mental model (READ THIS FIRST)
2. **`.notes/index.md`** - Task-oriented navigation hub and file organization
3. **`.cursorrules`** - Critical invariants and rules that must NEVER be broken

## Development Commands

### Testing

```bash
just test                            # Run all tests
just coverage                        # Generate coverage report
cargo test test_name -- --nocapture  # Run specific test with output
```

### Running Locally

```bash
# View help
cargo run -- --help
cargo run -- lint --help

# Run against test fixture
cd fixtures/fluid-framework
cargo run -- lint
cargo run -- fix --dry-run

# Run specific commands
cargo run -- lint --dependency-types prod,dev
cargo run -- fix --dependencies react
cargo run -- format --check
cargo run -- update --target latest
```

### Linting & Formatting

```bash
just format   # Fix formatting with Biome
just lint     # Run all lint checks
```

### Other Tools

```bash
just benchmark  # Performance benchmarking
just flamegraph # CPU profiling
```

## Quick Architecture Overview

Every command follows a **3-phase pipeline**:

1. **Create Context** - Read CLI args → config file → package.json files → collect dependencies
2. **Inspect Context** - Assign `InstanceState` to each dependency (valid/invalid/suspect)
3. **Run Command** - Process instances based on their states, return exit code (0 or 1)

**Core principle:** Each dependency occurrence is an "Instance" that gets tagged with a state. Commands then filter and process instances based on these states.

See `.notes/context.md` for detailed explanation and `.notes/index.md` for code patterns.

## Where to Find Things

Use this as a quick reference to locate information:

### Understanding the Codebase

- **Architecture and mental model** → `.notes/context.md`
- **Task-oriented navigation** → `.notes/index.md`
- **Data structures explained** → `.notes/context.md`
- **Visual diagrams** → `.notes/diagrams.md`
- **Design patterns and rationale** → `.notes/patterns.md`

### Working on Specific Tasks

- **"How do I add a command?"** → `.notes/examples/adding-command.md`
- **"How do I add validation logic?"** → `.notes/examples/adding-instance-state.md`
- **"How do I write tests?"** → `.notes/examples/writing-tests.md`
- **"What InstanceState should I use?"** → `.notes/quick-reference.md`
- **"Which approach should I take?"** → `.notes/decision-trees.md`

### Finding Code

- **File organization and locations** → `.notes/index.md` (File Organization section)
- **Search patterns and tips** → `.notes/search-tips.md`
- **Common code patterns** → `.notes/index.md` (Common Development Patterns section)
- **Naming conventions** → `.notes/index.md` (Naming Conventions section)

### Critical Rules and Mistakes

- **Rules that must NEVER be broken** → `.cursorrules` (Key Invariants section)
- **Common AI mistakes to avoid** → `.cursorrules` (Common Mistakes section)
- **Behavioral guidelines** → `.cursorrules` (AI Behavioral Guidelines section)

### High-Level Context

- **Project structure and workflow** → `CONTRIBUTING.md`
- **Ongoing refactoring work** → `SPECIFIER_REFACTOR.md`

## Critical Rules Summary

See `.cursorrules` for full details. **Never:**

1. Assign InstanceState during Context creation (Phase 1) - only assign in visit_packages (Phase 2)
2. Manually construct Context in tests - always use TestBuilder
3. Modify files in visit_packages - inspection phase is read-only
4. Forget to register commands in all 3 places (cli.rs enum, main.rs match, commands/ module)
5. Use Arc when Rc works - use Rc for single-threaded sharing

Breaking these rules will break the system.

## Quick Testing Example

Always use TestBuilder pattern - see `.notes/examples/writing-tests.md` for full guide:

```rust
use crate::test::builder::TestBuilder;
use serde_json::json;

#[test]
fn test_name() {
    let ctx = TestBuilder::new()
        .with_packages(vec![
            json!({"name": "pkg-a", "dependencies": {"react": "17.0.0"}})
        ])
        .with_version_group(json!({
            "dependencies": ["react"],
            "pinned": "18.0.0"
        }))
        .build_and_visit_packages();

    // Use expect() helper from src/test/expect.rs
}
```

## Git Workflow

- **main** - Most recently published Rust v14 alpha (target this for PRs)
- **v14-alpha** - Development branch for next version
- **13.x.x** - Legacy TypeScript version (being replaced)

## When You're Stuck

1. Check `.notes/index.md` - Find the right document for your task
2. Check `.notes/decision-trees.md` - Use flowcharts for decision guidance
3. Check `.notes/context.md` - Refresh your mental model
4. Check `.cursorrules` - Verify you're not violating a critical rule
5. Search existing code - Look for similar implementations
6. Check test examples - Look at `src/visit_packages/*_test.rs` for patterns

## Test Fixture

`fixtures/fluid-framework/` contains Microsoft's FluidFramework monorepo for realistic manual testing:

```bash
cd fixtures/fluid-framework
cargo run -- lint
cargo run -- fix --dry-run
```
