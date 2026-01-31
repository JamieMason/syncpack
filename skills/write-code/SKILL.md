---
name: write-code
description: Rust code style and conventions for Syncpack. Use when writing or modifying Rust code. Covers functional patterns, imports, naming, and quality standards.
---

# Write Code

Rust code conventions for Syncpack.

## Style

- **Functional style:** pipelines over loops
- **Avoid `?` chains:** use `.and_then()`, `.map()`, `.or_else()`
- **Descriptive names:** clarity over brevity
- **Named placeholders:** `println!("{var}")` not `println!("{}", var)`
- **British English:** "behaviour" not "behavior", "organised" not "organized"

## Imports

Single `use` statement with grouped braces:

```rust
use {
  crate::{cli::Cli, config::Config},
  log::{debug, error},
  std::{process::exit, sync::Arc},
};
```

Rules:

- **Never use `super::`** — always `crate::` for internal imports
- Group: `crate::`, external crates, `std::`
- Alphabetise within groups

## File Organisation

| Adding...   | Location                                                         |
| ----------- | ---------------------------------------------------------------- |
| New command | `src/commands/{name}.rs`                                         |
| New test    | Sibling `_test.rs` file (e.g., `src/foo.rs` → `src/foo_test.rs`) |

**NEVER** use `#[cfg(test)]` modules inside implementation files.

## Quality

- Functions <50 lines, commands 100-300 lines
- Zero warnings (except during TDD red phase)
- No comments by default (only for genuinely complex logic)
- Run `just format` before committing

## Patterns

### Iterating Instances

```rust
ctx.version_groups.iter().for_each(|group| {
    group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
        dependency.get_sorted_instances()
            .filter(|instance| instance.is_invalid())
            .for_each(|instance| { /* process */ });
    });
});
```

### Error Handling

Prefer combinators over `?`:

```rust
// Good
path.parent()
    .and_then(|p| p.to_str())
    .map(|s| s.to_string())
    .unwrap_or_default()

// Avoid
let parent = path.parent()?;
let str = parent.to_str()?;
Ok(str.to_string())
```

### State Mutation

```rust
let mut state = instance.state.borrow_mut();
if !state.is_invalid() {
    *state = InstanceState::fixable(SomeVariant);
}
```
