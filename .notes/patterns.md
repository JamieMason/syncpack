# Design Patterns in Syncpack

This document explains the design patterns used in Syncpack and the rationale behind them.

## Table of Contents

- [Three-Phase Pipeline](#three-phase-pipeline)
- [Visitor Pattern](#visitor-pattern)
- [Builder Pattern for Tests](#builder-pattern-for-tests)
- [State Machine Pattern](#state-machine-pattern)
- [Ownership and Borrowing](#ownership-and-borrowing)
- [Lazy Evaluation](#lazy-evaluation)
- [First Match Wins](#first-match-wins)
- [Tagged Unions (Enums with Data)](#tagged-unions-enums-with-data)

---

## Three-Phase Pipeline

### Pattern

Every command follows the same three-phase pattern:

```
1. Create Context (Read-Only)
   ↓
2. Inspect Context (Tag with States)
   ↓
3. Run Command (Process and Side Effects)
```

### Rationale

**Separation of Concerns:**

- **Phase 1** handles data collection and is purely functional
- **Phase 2** handles validation logic and tags instances
- **Phase 3** handles side effects (printing, writing files)

**Benefits:**

- Clear data flow that's easy to reason about
- Each phase can be tested independently
- Validation logic is centralized in one place
- Commands are simple and focused on their specific behavior

**Why Not Combine Phases?**

- Combining creation and inspection would make Context construction complex
- Combining inspection and execution would mix validation with side effects
- Separation allows reusing inspection logic across commands

### Example

```rust
// Phase 1: Create
let config = Config::from_cli(cli);
let packages = Packages::from_config(&config);
let ctx = Context::create(config, packages, registry_client);

// Phase 2: Inspect
let ctx = visit_packages(ctx);

// Phase 3: Run
let exit_code = lint::run(ctx);
```

---

## Visitor Pattern

### Pattern

The `visit_packages()` and `visit_formatting()` functions traverse the Context structure and apply validation logic to each element.

```rust
pub fn visit_packages(ctx: Context) -> Context {
  ctx.version_groups.iter().for_each(|group| {
    group.dependencies.values().for_each(|dependency| {
      match dependency.variant {
        VersionGroupVariant::Banned => banned::visit(dependency),
        VersionGroupVariant::Pinned => pinned::visit(dependency),
        // ...
      }
    });
  });
  ctx
}
```

### Rationale

**Extensibility:**

- Easy to add new validation rules (just add a new visitor module)
- Each rule is isolated in its own module
- No need to modify existing code when adding new rules

**Single Responsibility:**

- Each visitor module handles one type of validation
- Clear mapping between version group types and validation logic

**Benefits:**

- Validation logic is organized by feature, not scattered
- Easy to test each validation rule in isolation
- New contributors can add rules without understanding the entire codebase

**Why Not Class-Based Visitors?**

- Rust's ownership model makes function-based visitors simpler
- No need for visitor traits or complex hierarchies
- Each visitor is just a function that operates on the data it needs

---

## Builder Pattern for Tests

### Pattern

Tests use `TestBuilder` to construct test scenarios:

```rust
let ctx = TestBuilder::new()
    .with_packages(vec![json!({...})])
    .with_version_group(json!({...}))
    .build_and_visit_packages();
```

### Rationale

**Ergonomics:**

- Fluent API is readable and self-documenting
- Reduces boilerplate in every test
- Makes test intent clear

**Encapsulation:**

- Hides complexity of Context construction
- Prevents tests from depending on internal implementation details
- If Context structure changes, only TestBuilder needs updating

**Defaults:**

- Builder provides sensible defaults for all fields
- Tests only specify what matters for that specific test
- Reduces noise and focuses on what's being tested

**Benefits:**

- Tests are shorter and easier to write
- Tests are more maintainable (less coupling to internals)
- Consistent test structure across the codebase

**Why Not Manual Construction?**

- Context has many interdependent fields
- Manual construction is verbose and error-prone
- Tests would be brittle and hard to maintain
- Builder pattern is standard in Rust testing

### Anti-Pattern

```rust
// DON'T: Manual construction
let config = Config { /* ... */ };
let packages = Packages { /* ... */ };
let ctx = Context { config, packages, /* ... */ };
```

```rust
// DO: Builder pattern
let ctx = TestBuilder::new()
    .with_packages(vec![...])
    .build_and_visit_packages();
```

---

## State Machine Pattern

### Pattern

Each Instance goes through a state machine during inspection:

```
Unknown → [Inspection] → Valid | Invalid | Suspect
```

InstanceState is a tagged enum representing all possible states:

```rust
pub enum InstanceState {
    Unknown,                      // Initial state
    Valid(ValidInstance),         // Passes validation
    Invalid(InvalidInstance),     // Fails validation
    Suspect(SuspectInstance),     // Misconfiguration
}
```

### Rationale

**Explicit States:**

- Every possible outcome of validation is represented
- No implicit or hidden states
- Compiler ensures all states are handled

**Type Safety:**

- Can't accidentally process an instance in the wrong state
- Pattern matching forces handling of all variants
- States carry additional data (which specific validation failed)

**Self-Documenting:**

- State names explain what's wrong (DiffersToLocal, IsBanned, etc.)
- Reading code tells you what validations exist
- Easy to understand what will be fixed or reported

**Benefits:**

- Commands can filter by state (is_fixable, is_invalid, etc.)
- New validations just add new state variants
- Clear distinction between fixable and unfixable issues

**Why Nested Enums?**

```rust
Invalid(InvalidInstance)
  ├── Fixable(FixableInstance)     // We know the fix
  ├── Unfixable(UnfixableInstance) // Ambiguous
  └── Conflict(...)                // Conflicting rules
```

This hierarchy makes it easy to:

- Find all fixable issues: `instance.is_fixable()`
- Find all invalid issues: `instance.is_invalid()`
- Distinguish between "can auto-fix" and "needs human decision"

---

## Ownership and Borrowing

### Pattern

Context flows through the pipeline using Rust's ownership:

```rust
// Context is created
let ctx = Context::create(...);

// Ownership transferred to visitor
let ctx = visit_packages(ctx);

// Ownership transferred to command
let exit_code = lint::run(ctx);
// ctx is consumed, can't be used again
```

### Rationale

**Move Semantics:**

- Each phase takes ownership and returns it
- Prevents accidental mutation from other code
- Compiler enforces single owner at a time

**No Cloning:**

- Context can be large (many packages, dependencies)
- Moving is free (just pointer transfer)
- Avoids expensive deep copies

**Interior Mutability Where Needed:**

- Instance states use `RefCell` for mutation during inspection
- Allows tagging without requiring `&mut Context`
- Safe because only one phase mutates at a time

**Reference Counting:**

- `Rc<Instance>` allows cheap sharing
- Multiple version groups can reference same instances
- No need for complex lifetime management

**Benefits:**

- No lifetime parameters in function signatures
- Clear ownership makes data flow obvious
- Prevents data races at compile time
- Zero runtime overhead

### Anti-Pattern

```rust
// DON'T: Pass by reference everywhere
fn visit_packages(ctx: &mut Context) { /* ... */ }

// DO: Pass by value (move semantics)
fn visit_packages(ctx: Context) -> Context { /* ... */ }
```

---

## Lazy Evaluation

### Pattern

Commands print headers only when they have content to show:

```rust
pub fn run(ctx: Context) -> i32 {
  let mut has_printed_group = false;

  group.get_sorted_dependencies(...).for_each(|dependency| {
    let mut has_printed_dependency = false;

    dependency.get_sorted_instances()
      .filter(|instance| instance.is_invalid())
      .for_each(|instance| {
        if !has_printed_group {
          print_group_header();
          has_printed_group = true;
        }
        if !has_printed_dependency {
          print_dependency_header();
          has_printed_dependency = true;
        }
        print_instance();
      });
  });
}
```

### Rationale

**User Experience:**

- No empty sections in output
- Only show what's relevant
- Cleaner, more focused output

**Performance:**

- Don't format strings that won't be printed
- Don't waste time on unnecessary work

**Benefits:**

- Output is more readable
- Scales well (monorepos with many packages)
- Easy to understand what went wrong

**Why Not Print Everything?**

- Would clutter output with empty groups
- Users would have to scroll past irrelevant sections
- Harder to find actual issues

---

## First Match Wins

### Pattern

When assigning instances to version groups, the first matching group wins:

```rust
// User config
versionGroups: [
  { "dependencies": ["react"], "pinned": "18.0.0" },
  { "dependencies": ["**"], "policy": "highestSemver" },
]
```

React matches the first group (pinned), not the second (catch-all).

### Rationale

**Predictability:**

- Clear, deterministic behavior
- Order matters, which is explicit in config
- No ambiguity about which rule applies

**User Control:**

- Users can be specific first, general later
- Common pattern in configuration systems
- Matches user mental model (if/else chains)

**Benefits:**

- No complex rule merging logic needed
- Easy to reason about which rule will apply
- Users can override defaults by putting specific rules first

**Why Not "Most Specific Wins"?**

- Hard to define "most specific" (is `react*` more specific than `@org/*`?)
- Would require complex scoring system
- Order is simpler and more predictable

---

## Tagged Unions (Enums with Data)

### Pattern

Enums carry data specific to each variant:

```rust
pub enum Specifier {
    BasicSemver(BasicSemver),        // Carries version data
    Git(Git),                        // Carries repo URL
    WorkspaceProtocol(WorkspaceProtocol), // Carries workspace info
    None,                            // No data
}
```

### Rationale

**Type Safety:**

- Can't access git URL on a semver variant
- Compiler prevents invalid operations
- Pattern matching forces handling all cases

**Memory Efficiency:**

- Enum is size of largest variant + tag
- No separate heap allocations needed
- More cache-friendly than trait objects

**Exhaustive Matching:**

- Adding new variant causes compile errors in match statements
- Forces update of all code that handles specifiers
- Prevents forgetting to handle new cases

**Benefits:**

- Self-documenting (variant names explain what they are)
- Impossible to be in invalid state
- Easy to add new specifier types

### Example

```rust
match &instance.specifier {
    Specifier::BasicSemver(basic) => {
        // Can access basic.version, basic.semver_range
    }
    Specifier::Git(git) => {
        // Can access git.url
    }
    // Compiler error if we forget a variant
}
```

**Why Not Trait Objects?**

```rust
// Alternative: Trait objects
pub trait Specifier { /* ... */ }
pub struct BasicSemver { /* ... */ }
impl Specifier for BasicSemver { /* ... */ }

// Problems:
// - Need Box<dyn Specifier> (heap allocation)
// - Can't exhaustively match on type
// - Can't access variant-specific fields easily
```

---

## Summary

These patterns work together to create a system that is:

1. **Predictable** - Three-phase pipeline, first-match-wins
2. **Type-Safe** - Tagged unions, state machines, ownership
3. **Maintainable** - Visitor pattern, builder pattern, separation of concerns
4. **Performant** - Move semantics, lazy evaluation, no unnecessary cloning
5. **Testable** - Builder pattern, pure functions, isolated visitors

The patterns reinforce each other:

- Ownership enables the three-phase pipeline
- State machines make commands simple
- Visitor pattern keeps validation logic organized
- Tagged unions ensure type safety throughout

When adding features, follow these patterns to maintain consistency and quality.
