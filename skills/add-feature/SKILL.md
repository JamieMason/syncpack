---
name: add-feature
description: Add new features to Syncpack including commands and validation logic. Use when implementing new CLI commands, adding InstanceState variants, or extending version group behaviour.
---

# Add Feature

Guide for adding new functionality to Syncpack.

## Quick Decision

What are you adding?

- **New CLI command** (lint, fix, format, etc.) → See [adding-command.md](adding-command.md)
- **New validation logic** (InstanceState variant) → See [adding-instance-state.md](adding-instance-state.md)
- **Both** → Start with instance state, then add command

## Prerequisites

Before adding any feature:

1. Understand the 3-phase pattern: Create Context → Inspect Context → Run Command
2. Know which phase your feature affects
3. Check if similar code exists (use `ast-grep -p 'PATTERN' src/`)

## Feature Types

### Commands

Commands are user-facing operations (`syncpack lint`, `syncpack fix`, etc.).

**When to create a new command:**

- New user-facing operation that doesn't fit existing commands
- Different output format or behaviour needed

**Registration points (all three required):**

1. `src/cli.rs` - Add to `Subcommand` enum
2. `src/main.rs` - Add match arm for dispatch
3. `src/commands/*.rs` - Implement `pub fn run(ctx: Context) -> i32`

→ Full guide: [adding-command.md](adding-command.md)

### Instance States

InstanceState variants describe validation results (valid, fixable, unfixable, suspect).

**When to add a new state:**

- New validation rule needed
- New type of error/warning to report
- New auto-fix capability

**Location:** `src/instance_state.rs` + `src/visit_packages/*.rs`

→ Full guide: [adding-instance-state.md](adding-instance-state.md)

## Common Workflow

1. Write failing test using TestBuilder
2. Implement minimal code to pass
3. Run `cargo clippy` and fix warnings
4. Refactor if needed
5. Update documentation

## Checklist

Before submitting:

- [ ] Tests pass (`just test`)
- [ ] Zero clippy warnings
- [ ] Follows existing patterns
- [ ] Registered in all required places (for commands)
- [ ] Uses TDD approach
