---
name: search-code
description: Search for code patterns in Syncpack. Use when finding symbols, implementations, or understanding how code is used. Covers ast-grep for Rust and grep/rg for other cases.
---

# Search Code

Guide for searching the Syncpack codebase effectively.

## Golden Rule

**Use `ast-grep` for Rust files** — it filters out comments, strings, and docs.

## Quick Reference

| Looking for...                    | Use         |
| --------------------------------- | ----------- |
| Rust symbols, patterns, structs   | `ast-grep`  |
| Multiple file types               | `grep`/`rg` |
| Content in comments intentionally | `grep`/`rg` |
| Non-Rust files (JSON, MD, TOML)   | `grep`/`rg` |

## ast-grep Examples

```bash
# Find struct definitions
ast-grep -p 'pub struct $NAME' src/

# Find function calls
ast-grep -p 'Specifier::new' src/

# Find enum variants
ast-grep -p 'pub enum $NAME' src/

# Find impl blocks
ast-grep -p 'impl $TYPE' src/

# Find specific method definitions
ast-grep -p 'pub fn run($$$) -> i32' src/

# Find match expressions
ast-grep -p 'match $EXPR { $$$ }' src/
```

## Pattern Syntax

| Pattern | Matches                                      |
| ------- | -------------------------------------------- |
| `$NAME` | Single identifier                            |
| `$$$`   | Zero or more items (arguments, fields, etc.) |
| `$_`    | Any single expression                        |

## When to Use grep/rg

```bash
# Search across file types
rg "TODO" --type rust --type md

# Search in comments (ast-grep ignores these)
rg "FIXME" src/

# Search non-Rust files
rg "version" Cargo.toml

# Case-insensitive search
rg -i "error" src/
```

## Common Searches

### Find where a type is used

```bash
ast-grep -p 'Context' src/
```

### Find all InstanceState assignments

```bash
ast-grep -p 'InstanceState::valid' src/
ast-grep -p 'InstanceState::fixable' src/
ast-grep -p 'InstanceState::unfixable' src/
```

### Find test files

```bash
find src -name '*_test.rs'
```

### Find command implementations

```bash
ast-grep -p 'pub fn run($$$) -> i32' src/commands/
```

## Tips

- Start broad, then narrow with more specific patterns
- Use `ast-grep` output to find the file, then `read_file` to understand context
- Chain searches: find the type, then find where it's constructed
