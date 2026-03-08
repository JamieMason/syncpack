# Adding a New InstanceState Variant

<scenario>
This guide walks through adding new `InstanceState` variants. The example used
throughout is `SameMinorHasMajorMismatch`, a real variant added during the
sameMinor improvement work. It is an `Unfixable` state assigned when instances
in a `sameMinor` version group have different MAJOR versions.
</scenario>

---

## Step 1: Choose the right category

```
InstanceState
├── Unknown              — never assign this; it is the initial pre-inspection state
├── Valid(ValidInstance) — instance follows all rules correctly
├── Invalid(InvalidInstance)
│   ├── Fixable(FixableInstance)                  — syncpack knows the correct value and can auto-fix
│   ├── Unfixable(UnfixableInstance)              — ambiguous; needs human decision
│   └── Conflict(SemverGroupAndVersionConflict)   — two active rules are simultaneously unsatisfiable
└── Suspect(SuspectInstance)                      — user misconfiguration
```

Ask:

- Do we know the correct value? → `Fixable`
- Is the situation ambiguous, multiple valid options? → `Unfixable`
- Are two rules active and irreconcilable? → `Conflict`
- Has the user misconfigured something? → `Suspect`
- Is everything fine? → `Valid`

For `SameMinorHasMajorMismatch`: crossing a major version boundary is unsafe
and we cannot pick a winner without asking the user → `Unfixable`.

---

## Step 2: Add the variant to `src/instance_state.rs`

Find the appropriate enum and add the variant with a multi-line doc comment.
Every condition relevant to the state must be listed. Use these symbols:

- `✓` — condition is true / requirement is met
- `✘` — condition is false / requirement is violated
- `?` — ambiguous / unknown / why we cannot auto-fix
- `!` — notable side effect or policy decision

```rust
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UnfixableInstance {
  // ... existing variants ...

  /// - ✘ Instance is in a sameMinor version group
  /// - ✘ One or more other instances have a different MAJOR version
  /// - ? Crossing a major version boundary is unsafe
  /// - ? We cannot know which MAJOR the user wants and have to ask them
  SameMinorHasMajorMismatch,
}
```

The doc comment is the source of truth for when this state is assigned. Be
precise — every bullet should be independently verifiable from the visitor code.

---

## Step 3: Check whether any `impl` blocks need updating

Most additions require no changes to `impl InstanceState`. The severity, the
`is_fixable()` / `is_unfixable()` helpers, and the `get_name()` / `get_status_type()`
methods all work by matching on the outer category (`Valid`, `Invalid`,
`Suspect`), not on individual variants.

The only time you need to touch `impl InstanceState` is if you are adding a
variant that needs a **custom severity** (rare) or a new top-level predicate
(e.g. `is_banned()`). For ordinary additions, no changes are needed.

---

## Step 4: Write the detection logic in the visitor

Visitor files live in `src/visit_packages/`. Each visitor receives a
`&Dependency` and must call one of the mark methods on every instance:

```rust
instance.mark_valid(ValidInstance::SatisfiesSameMinorGroup, specifier);
instance.mark_fixable(FixableInstance::DiffersToPin, expected_specifier);
instance.mark_unfixable(UnfixableInstance::SameMinorHasMajorMismatch);
instance.mark_conflict(SemverGroupAndVersionConflict::MatchConflictsWithLocal);
instance.mark_suspect(SuspectInstance::InvalidLocalVersion);
```

Never assign to `instance.state` directly — always use these methods.

The specifier argument to `mark_valid` and `mark_fixable` is the **expected**
specifier — what the instance should be set to. Read the actual specifier from
`instance.descriptor.specifier`.

Example — marking all instances when a major mismatch is detected (computed
once before the per-instance loop):

```rust
let has_major_mismatch = dependency
    .instances
    .iter()
    .any(|i| !i.already_has_same_major_as_all(&dependency.instances));

dependency.instances.iter().for_each(|instance| {
    let actual_specifier = &instance.descriptor.specifier;

    if has_major_mismatch {
        instance.mark_unfixable(UnfixableInstance::SameMinorHasMajorMismatch);
        return;
    }

    // ... rest of visitor logic ...
});
```

Key rules:

- **Every instance must be marked** before the visitor returns. No instance may
  remain in `InstanceState::Unknown` after `visit_packages()` completes.
- **Compute per-dependency values once** before the loop (e.g. highest
  specifier, major-mismatch boolean) to avoid O(n²) re-checking.
- **Do not read `instance.state` inside the visitor** to decide what to do —
  states are write-once during inspection.

---

## Step 5: Integrate into `visit_packages.rs` (only for new visitors)

If you are adding a variant to an **existing** visitor, skip this step.

If you are adding a **new visitor module**, register it in
`src/visit_packages.rs`:

```rust
mod banned;
mod ignored;
mod indent;
mod pinned;
mod preferred_semver;
mod same_minor;   // ← example of an existing entry
mod same_range;
mod snapped_to;
mod your_new_visitor;  // ← add here

pub fn visit_packages(ctx: Context) -> Context {
    ctx.version_groups.iter().sorted_by(order_snapped_to_groups_last).for_each(|group| {
        group.dependencies.values().for_each(|dependency| match dependency.variant {
            VersionGroupVariant::Banned => banned::visit(dependency),
            VersionGroupVariant::SameMinor => same_minor::visit(dependency),
            // ... add your new variant arm here if a new VersionGroupVariant was added ...
            VersionGroupVariant::SnappedTo => snapped_to::visit(dependency, &ctx),
        });
    });
    ctx
}
```

---

## Step 6: Write tests

Test files live alongside their visitor: `src/visit_packages/same_minor_test.rs`
is declared inside `same_minor.rs` with:

```rust
#[cfg(test)]
#[path = "same_minor_test.rs"]
mod same_minor_test;
```

Tests use `TestBuilder` and `expect(&ctx).to_have_instances(...)`. Every
`ExpectedInstance` must specify:

- `state` — the exact `InstanceState` expected
- `dependency_name` — the dependency name (not the instance id)
- `id` — `"{dep} in {location} of {package}"` format
- `actual` — the on-disk specifier string
- `expected` — `Some("...")` for the expected/fix-target specifier, or `None`
- `overridden` — `Some("...")` when a semver group preference was overridden

```rust
use {
    crate::{
        instance_state::{FixableInstance::*, InstanceState, UnfixableInstance::*, ValidInstance::*},
        test::{
            builder::TestBuilder,
            expect::{expect, ExpectedInstance},
        },
    },
    serde_json::json,
};

#[test]
fn major_mismatch_marks_all_instances_unfixable() {
    let ctx = TestBuilder::new()
        .with_packages(vec![json!({
            "name": "my-project",
            "version": "1.0.0",
            "dependencies": {
                "foo": "1.2.0"
            },
            "devDependencies": {
                "foo": "2.1.0"
            }
        })])
        .with_version_groups(vec![json!({
            "dependencies": ["foo"],
            "policy": "sameMinor"
        })])
        .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
            state: InstanceState::valid(IsLocalAndValid),
            dependency_name: "my-project",
            id: "my-project in /version of my-project",
            actual: "1.0.0",
            expected: Some("1.0.0"),
            overridden: None,
        },
        ExpectedInstance {
            state: InstanceState::unfixable(SameMinorHasMajorMismatch),
            dependency_name: "foo",
            id: "foo in /dependencies of my-project",
            actual: "1.2.0",
            expected: None,
            overridden: None,
        },
        ExpectedInstance {
            state: InstanceState::unfixable(SameMinorHasMajorMismatch),
            dependency_name: "foo",
            id: "foo in /devDependencies of my-project",
            actual: "2.1.0",
            expected: None,
            overridden: None,
        },
    ]);
}
```

Cover every leaf of the decision tree. For a visitor with semver group
interaction, the minimum test matrix is:

| scenario                             | test name pattern                                          |
| ------------------------------------ | ---------------------------------------------------------- |
| no semver group, correct version     | `satisfies_..._and_has_no_semver_group`                    |
| no semver group, wrong version       | `has_..._mismatch_and_no_semver_group`                     |
| safe semver group, already matches   | `satisfies_..._and_matches_compatible_..._semver_group`    |
| safe semver group, does not match    | `satisfies_..._but_mismatches_compatible_..._semver_group` |
| unsafe semver group, already matches | `..._but_range_matches_incompatible_..._semver_group`      |
| unsafe semver group, does not match  | `..._but_range_mismatches_incompatible_..._semver_group`   |

---

## Step 7: Verify commands need no changes

Commands (`src/commands/*.rs`) operate on states generically:

```rust
// lint.rs, fix.rs etc. already handle your new variant automatically
dependency.get_sorted_instances()
    .filter(|instance| instance.is_invalid())  // or is_fixable(), is_unfixable() etc.
    .for_each(|instance| { ... });
```

New variants within existing categories (`Fixable`, `Unfixable`, etc.) are
picked up automatically. You only need to touch command files if your new state
requires **bespoke fix logic** — for example, `IsBanned` requires removing the
dependency entry rather than updating a version string.

---

## Step 8: Run the tests

```bash
# Run tests for a specific visitor
cargo test same_minor

# Run all tests
just test
```

---

## Checklist

- [ ] Variant added to the correct enum in `src/instance_state.rs`
- [ ] Doc comment uses `✓` `✘` `?` `!` bullets and describes every condition
- [ ] Visitor uses `instance.mark_*()` methods — no direct state assignment
- [ ] Per-dependency values computed once before the per-instance loop
- [ ] Every instance is marked on every code path through the visitor
- [ ] Tests cover every leaf of the decision tree
- [ ] Tests use `expect(&ctx).to_have_instances(...)` with `ExpectedInstance`
- [ ] No command changes needed (or bespoke fix logic added if required)

---

## Related files

- `src/instance_state.rs` — all state variant definitions and annotations
- `src/visit_packages.rs` — visitor dispatch
- `src/visit_packages/*.rs` — visitor implementations
- `src/visit_packages/*_test.rs` — test examples to copy patterns from
- `src/commands/fix.rs` — example of how states drive fix behaviour
- `.notes/reference/version-group-implementation-rules.md` — cross-cutting rules for all visitors
