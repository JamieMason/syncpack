---
name: fix-bug
description: Debug and fix bugs in Syncpack using scientific debugging methodology. Use when a test is failing, unexpected behaviour occurs, or investigating issues. Covers hypothesis-driven debugging and TDD-based fixes.
---

# Fix Bug

Guide for debugging and fixing bugs in Syncpack.

## Scientific Debugging Workflow

1. **Observe** — Understand the symptom precisely
2. **Hypothesise** — Form a theory about the root cause
3. **Experiment** — Test the hypothesis with targeted changes
4. **Validate** — Confirm the fix with tests
5. **Verify** — Run full test suite

## Step 1: Observe

Gather information before changing code:

```bash
# Run the failing test with output
cargo test test_name -- --nocapture

# Run with backtrace
RUST_BACKTRACE=1 cargo test test_name

# Test against fixture
cd fixtures/fluid-framework
cargo run -- lint
```

Questions to answer:

- What is the exact error message?
- What input triggers it?
- What is the expected vs actual behaviour?

## Step 2: Hypothesise

Common root causes by symptom:

| Symptom              | Likely Cause                                            |
| -------------------- | ------------------------------------------------------- |
| State is `Unknown`   | `visit_packages` not called or instance skipped         |
| Wrong state assigned | Validation logic order, earlier check overriding        |
| Instance not found   | Package/dependency name mismatch, location format       |
| Command not running  | Missing registration in cli.rs, main.rs, or commands.rs |
| Panic/unwrap failure | Unexpected None or Err value                            |

## Step 3: Experiment

Add targeted debug output:

```rust
use log::debug;

debug!("Instance: {:#?}", instance);
debug!("State before: {:?}", instance.state.borrow());
```

Search for related code:

```bash
# Find where state is assigned
ast-grep -p 'InstanceState::fixable' src/visit_packages/

# Find similar patterns
ast-grep -p 'PATTERN' src/
```

## Step 4: Validate

Write a failing test FIRST (TDD):

```rust
#[test]
fn reproduces_the_bug() {
    let ctx = TestBuilder::new()
        .with_packages(vec![/* minimal reproduction */])
        .build_and_visit_packages();

    // Assert expected behaviour (will fail initially)
}
```

Then fix the code until the test passes.

## Step 5: Verify

```bash
just test        # All tests pass
cargo clippy     # No warnings
```

## Where to Look

### By component

| Issue with...    | Check...                                 |
| ---------------- | ---------------------------------------- |
| CLI parsing      | `src/cli.rs`                             |
| Config loading   | `src/config.rs`, `src/rcfile.rs`         |
| Package reading  | `src/packages.rs`, `src/package_json.rs` |
| State assignment | `src/visit_packages/*.rs`                |
| Command output   | `src/commands/*.rs`, `src/commands/ui/`  |
| Version parsing  | `src/specifier/*.rs`                     |

### By phase

| Phase   | What happens                             | Files                                      |
| ------- | ---------------------------------------- | ------------------------------------------ |
| Create  | Read config, packages, collect instances | `context.rs`, `packages.rs`                |
| Inspect | Assign InstanceState                     | `visit_packages.rs`, `visit_packages/*.rs` |
| Run     | Process instances, output/write          | `commands/*.rs`                            |

## Common Fixes

### Instance state not being assigned

Check:

1. Is the instance's version group variant handled in `visit_packages.rs`?
2. Is there an early return skipping this instance?
3. Is another validation assigning a state first?

### Test assertion failing

Check:

1. Is `build_and_visit_packages()` called (not just `build()`)?
2. Is the location string format correct? (`"{dep} in {location} of {package}"`)
3. Are there multiple instances of the same dependency?

### Command not found

Verify registration in all three places:

1. `src/cli.rs` — `Subcommand` enum
2. `src/main.rs` — match arm
3. `src/commands.rs` — module declaration

## Debugging Tips

- **Isolate** — Create minimal reproduction
- **Binary search** — Comment out code to narrow down
- **Print early** — Add debug output before suspected area
- **Check order** — Validation order matters (first match wins)
- **Read tests** — Existing tests show expected behaviour

## Anti-Patterns

❌ Changing code without understanding the cause
❌ Removing code just to make tests pass
❌ Fixing symptoms instead of root cause
❌ Skipping the test-first step
