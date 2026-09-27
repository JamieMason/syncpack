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

## Step 2: Add the variant to `crates/syncpack-model/src/instance/instance_state.rs`

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

Most additions require no changes to `impl InstanceState`. Severity resolution
and the `is_valid()` / `is_invalid()` / `get_name()` / `get_status_type()`
methods all work by matching on the outer category (`Valid`, `Invalid`,
`Suspect`), not on individual variants.

The only time you need to touch `impl InstanceState` is if you are adding a
variant that needs a **custom severity** (rare) or a new top-level predicate.
For ordinary additions, no changes are needed.

---

## Step 4: Write the detection logic in the visitor

Visitor implementations live in `crates/syncpack-rules/src/version_group/`, one
file per version-group policy (`same_minor.rs`, `pinned.rs`, `banned.rs`, ...).
Each is a struct (e.g. `SameMinorGroup`) implementing `VersionGroupBehavior`,
whose `visit(&self, input: &RuleInput, registry_updates, out: &mut Inspection)`
must call `out.classify(...)` for every instance:

```rust
out.classify(idx, InstanceState::Valid(ValidInstance::SatisfiesSameMinorGroup), Some(specifier), &self.severity, strict);
out.classify(idx, InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::DiffersToPin)), Some(expected_specifier), &self.severity, strict);
out.classify(idx, InstanceState::Invalid(InvalidInstance::Unfixable(UnfixableInstance::SameMinorHasMajorMismatch)), None, &self.severity, strict);
out.classify(idx, InstanceState::Invalid(InvalidInstance::Conflict(SemverGroupAndVersionConflict::MatchConflictsWithLocal)), None, &self.severity, strict);
out.classify(idx, InstanceState::Suspect(SuspectInstance::InvalidLocalVersion), None, &self.severity, strict);
```

`Inspection` (`crates/syncpack-rules/src/inspection.rs`) is a plain value keyed
by `InstanceIdx`/`DependencyIdx`/`SourceIdx`, returned from `inspect()` rather
than mutated in place. `Instance` has no interior mutability — always go
through `out.classify(...)`, never assign state anywhere else.

The `expected` argument to `classify` is the **expected** specifier — what the
instance should be set to (`None` when there's nothing to fix towards, e.g.
`Suspect`/most `Unfixable` states). Read the actual specifier from
`instance.descriptor.specifier`.

Example — marking all instances when a major mismatch is detected (computed
once before the per-instance loop; see `crates/syncpack-rules/src/version_group/same_minor.rs`):

```rust
let all_same_major = dep
    .instances
    .first()
    .is_some_and(|idx| arena[idx.0].already_has_same_major_as_all(&dep.instances, arena));

if !all_same_major {
    for &idx in &dep.instances {
        let instance = &arena[idx.0];
        let actual_specifier = &instance.descriptor.specifier;
        out.classify(
            idx,
            InstanceState::Invalid(InvalidInstance::Unfixable(UnfixableInstance::SameMinorHasMajorMismatch)),
            Some(Rc::clone(actual_specifier)),
            &self.severity,
            strict,
        );
    }
    continue;
}
```

Key rules:

- **Every instance must be classified** before the visitor returns. No instance
  may remain in `InstanceState::Unknown` after `visit_packages()` completes.
- **Compute per-dependency values once** before the loop (e.g. highest
  specifier, major-mismatch boolean) to avoid O(n²) re-checking.
- **Never read state back out of `out` inside the same visitor pass** to decide
  what to do next — states are write-once per instance during inspection.
  (The one documented exception is `SnappedTo`, which runs as a second stage
  and reads its targets' stage-1 results back out of `out` — see
  `crates/syncpack-rules/src/version_group.rs`.)

---

## Step 5: Integrate a new visitor (only for new policies)

If you are adding a variant to an **existing** visitor, skip this step.

If you are adding a **new version-group policy**, `crates/syncpack-rules/src/version_group.rs`
requires changes in five places, all matching on the `VersionGroup` enum:

```rust
// 1. Declare the module and re-export the struct
mod your_new_visitor;
pub use { /* ... */ your_new_visitor::YourNewGroup };

// 2. Add a variant
pub enum VersionGroup {
    // ...
    YourNew(YourNewGroup),
}

// 3-5. Add a match arm in each of:
// impl VersionGroupBehavior for VersionGroup { fn selector(&self) -> ... }
// impl VersionGroupBehavior for VersionGroup { fn dependency_indices(&self) -> ... }
// impl VersionGroupBehavior for VersionGroup { fn set_dependency_indices(&mut self, ...) }
// impl VersionGroupBehavior for VersionGroup { fn visit(&self, ...) }
```

Also add a case to `VersionGroup::variant_label()` (used in logging/UI), and to
`build_version_groups` (`src/discovery/groups.rs`) so the rcfile's `policy`
string can select it. There is no single dispatch table — the compiler's
exhaustiveness checking on each `match self { ... }` is what catches a missed
arm, so add the enum variant first and let `cargo check` find every site.

---

## Step 6: Write tests

Test files live in `src/version_group/` in the **binary** crate (not alongside
the visitor, which is in `syncpack-rules` — that crate has no unit tests of its
own; these are its only test coverage). Each is wired into `src/main.rs`:

```rust
#[cfg(test)]
#[path = "version_group/same_minor_test.rs"]
mod same_minor_test;
```

Tests use `TestBuilder` and `expect(&inspected).to_have_instances(...)`. Every
`ExpectedInstance` must specify:

- `state` — the exact `InstanceState` expected
- `dependency_name` — the dependency name (not the instance id)
- `id` — `"{dep} in {location} of {package}"` format
- `actual` — the on-disk specifier string
- `expected` — `Some("...")` for the expected/fix-target specifier, or `None`
- `overridden` — `Some("...")` when a semver group preference was overridden
- `severity` — `Some(severity)` to assert resolved severity, or `None` to skip that assertion

```rust
use {
    crate::test::{
        builder::TestBuilder,
        expect::{ExpectedInstance, expect},
    },
    serde_json::json,
    syncpack_model::instance::{FixableInstance::*, InstanceState, UnfixableInstance::*, ValidInstance::*},
};

#[tokio::test]
async fn major_mismatch_marks_all_instances_unfixable() {
    let inspected = TestBuilder::new()
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
        .with_version_group(json!({
            "dependencies": ["foo"],
            "policy": "sameMinor"
        }))
        .run()
        .await;

    expect(&inspected).to_have_instances(vec![
        ExpectedInstance {
            state: InstanceState::valid(IsLocalAndValid),
            dependency_name: "my-project",
            id: "my-project in /version of my-project",
            actual: "1.0.0",
            expected: Some("1.0.0"),
            overridden: None,
            severity: None,
        },
        ExpectedInstance {
            state: InstanceState::unfixable(SameMinorHasMajorMismatch),
            dependency_name: "foo",
            id: "foo in /dependencies of my-project",
            actual: "1.2.0",
            expected: None,
            overridden: None,
            severity: None,
        },
        ExpectedInstance {
            state: InstanceState::unfixable(SameMinorHasMajorMismatch),
            dependency_name: "foo",
            id: "foo in /devDependencies of my-project",
            actual: "2.1.0",
            expected: None,
            overridden: None,
            severity: None,
        },
    ]);
}
```

`.run().await` builds and runs the full pipeline including registry fetches.
`TestBuilder` also has a sync `build_and_visit_packages()` for cases that don't
need the registry.

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

Commands (`src/commands/*.rs`) operate on states generically, reading them out
of `Inspection` rather than off the instance:

```rust
// lint.rs, fix.rs etc. already handle your new variant automatically
dependency.instances.iter()
    .filter(|idx| inspection.state(*idx).is_invalid())
    .for_each(|idx| { ... });
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

- [ ] Variant added to the correct enum in `crates/syncpack-model/src/instance/instance_state.rs`
- [ ] Doc comment uses `✓` `✘` `?` `!` bullets and describes every condition
- [ ] Visitor uses `out.classify(...)` — no direct state assignment
- [ ] Per-dependency values computed once before the per-instance loop
- [ ] Every instance is classified on every code path through the visitor
- [ ] Tests cover every leaf of the decision tree
- [ ] Tests use `expect(&inspected).to_have_instances(...)` with `ExpectedInstance`
- [ ] No command changes needed (or bespoke fix logic added if required)

---

## Related files

- `crates/syncpack-model/src/instance/instance_state.rs` — all state variant definitions and annotations
- `crates/syncpack-rules/src/visit_packages.rs` — two-stage visitor dispatch
- `crates/syncpack-rules/src/version_group.rs` — the `VersionGroup` enum and `VersionGroupBehavior` dispatch
- `crates/syncpack-rules/src/version_group/*.rs` — visitor implementations
- `src/version_group/*_test.rs` — test examples to copy patterns from (wired into `src/main.rs` via `#[path]`)
- `src/commands/fix.rs` — example of how states drive fix behaviour
- `.notes/reference/version-group-implementation-rules.md` — cross-cutting rules for all visitors
