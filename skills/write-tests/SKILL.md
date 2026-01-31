---
name: write-tests
description: Write tests for Syncpack using the TestBuilder pattern. Use when adding tests for commands, validation logic, or any new functionality. Covers TestBuilder API, assertion patterns, and common test scenarios.
---

# Write Tests

Guide for writing tests in Syncpack using the TestBuilder pattern.

## TDD Workflow (Mandatory)

1. **Study existing tests** — Read 2-3 tests in same file, identify the pattern, match it exactly
2. **Write failing test** — Never invent APIs; read source to see what exists
3. **Verify it fails** — Run `just test` to confirm
4. **Ask user to confirm** — Get approval before implementing
5. **Implement minimal code** — Only what's needed to pass
6. **Clean up** — Run `just format`, fix warnings, refactor if needed

## Golden Rule

**Always use TestBuilder** — Never manually construct Context in tests.

## Quick Start

```rust
use {
  crate::{
    instance_state::{FixableInstance::*, InstanceState, ValidInstance::*},
    test::{
      builder::TestBuilder,
      expect::{expect, ExpectedInstance},
    },
  },
  serde_json::json,
};

#[test]
fn test_descriptive_name() {
    let ctx = TestBuilder::new()
        .with_packages(vec![
            json!({"name": "pkg-a", "dependencies": {"react": "17.0.0"}}),
        ])
        .with_version_group(json!({
            "dependencies": ["react"],
            "pinned": "18.0.0"
        }))
        .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
            state: InstanceState::fixable(DiffersToPinnedVersion),
            dependency_name: "react",
            id: "react in /dependencies of pkg-a",
            actual: "17.0.0",
            expected: Some("18.0.0"),
            overridden: None,
        },
    ]);
}
```

## TestBuilder Methods

### Packages

```rust
.with_package(json!({...}))           // Single package
.with_packages(vec![json!({...})])    // Multiple packages
```

### Version Groups

```rust
.with_version_group(json!({...}))     // Single group
.with_version_groups(vec![...])       // Multiple groups
```

### Configuration

```rust
.with_config(json!({...}))            // Custom config
.with_semver_group(json!({...}))      // Semver rules
.with_strict(true)                    // Strict mode
```

### Registry (for update command)

```rust
.with_registry_updates(json!({"react": ["17.0.0", "18.0.0"]}))
.with_update_target(UpdateTarget::Latest)
```

### Build

```rust
.build()                              // Without visiting (rare)
.build_and_visit_packages()           // With visiting (most common)
```

## Location String Format

```
{dependency} in {location} of {package}
```

Examples:

- `"react in /dependencies of pkg-a"`
- `"lodash in /devDependencies of pkg-b"`
- `"pnpm in /packageManager of pkg-c"`

## Running Tests

```bash
just test                              # All tests
cargo test test_name -- --nocapture   # Specific test with output
cargo test banned_test                 # Tests matching pattern
```

## Test Organisation

- **Integration tests:** `src/visit_packages/*_test.rs` (preferred)
- **Unit tests:** Co-located as `*_test.rs` (e.g., `src/foo.rs` → `src/foo_test.rs`)
- **Test utilities:** `src/test/builder.rs`, `src/test/expect.rs`

## Common Patterns

→ Full patterns and examples: [patterns.md](patterns.md)

## Good Test Examples

Study these files:

- `src/visit_packages/banned_test.rs` — Comprehensive examples
- `src/visit_packages/pinned_test.rs` — Version group testing
- `src/visit_packages/local_test.rs` — Local package handling

## Common Mistakes

| Wrong                        | Right                                       |
| ---------------------------- | ------------------------------------------- |
| `Context { ... }`            | `TestBuilder::new()...`                     |
| `.build()` then check states | `.build_and_visit_packages()`               |
| `ctx.instances[0]`           | `.find(\|i\| i.dependency.name == "react")` |
