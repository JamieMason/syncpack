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
- Run `just format` before committing

## Comments

### Forbidden

- **`//!` module docs** — collect implementation history and rot
- **Phase/F-N labels** — `// Phase v4-2`, `// F16:`, banner headers like `// ---------- Phase v4-3 RED ----------`
- **History refs** — `Migrated from`, `carry-over`, `Mirrors v3`, `replaces the v3`, `previously inside X`, `reserved for future`
- **Issue/PR refs** — `Reproduces issue #239`, `GitHub issue #206`. Plans rot; the code is the truth
- **Banner separators** — `// ---------- Free functions: yaml ops ----------`. Use module structure or let the file speak for itself
- **Test scenario labels** — don't write `// Windows-style backslashes` above a test input. Use a descriptive variable binding (`let windows_backslashes = [...]`) instead
- **Name-restating field docs** — `/// A unique identifier for this instance`, `/// The dependency name`, `/// The instance id`. The field name already says it
- **Step numbering inside functions** — `// 1. ...`, `// 2. ...`. Drop the numbers; keep the WHY if it's non-obvious

### Allowed

- **`@TODO` markers** — future plans worth tracking inline
- **`///` doc comments** for non-obvious detail: `None`/`Err` semantics, side effects, invariants, distinctions between similar fields (e.g. `is_local_dependency` vs `is_local_instance`)
- **`//` inline comments** when WHY is non-obvious: hidden constraints, ordering invariants ("SnappedTo groups must be visited last"), workarounds for surprising library behaviour, "must not poison X" cautions

### Default

No comments. Add one only when the WHY is non-obvious to a fresh reader.

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
