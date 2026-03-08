# Version Group Implementation Rules

Rules and patterns which apply across all version group visitors in
`src/visit_packages/`. Read this before implementing or modifying any visitor.

---

## The Visitor Contract

Each visitor in `src/visit_packages/*.rs` receives a `&Dependency` and must
call one of the following mark methods on **every** instance before returning:

```rust
instance.mark_valid(ValidInstance::*, specifier);
instance.mark_fixable(FixableInstance::*, expected_specifier);
instance.mark_unfixable(UnfixableInstance::*);
instance.mark_conflict(SemverGroupAndVersionConflict::*);
instance.mark_suspect(SuspectInstance::*);
```

No instance may be left in `InstanceState::Unknown` after `visit_packages()`
completes. This is a hard invariant.

States are assigned during inspection only — never during Context creation and
never inside commands.

---

## Top-Level Branch Priority

Every visitor that cares about semver versions must evaluate branches in this
order. First match wins.

1. **Invalid local** — the dependency is a local package whose `.version` is
   missing or not exact semver. All instances in the group get a state
   immediately; no further checks are needed.
2. **Valid local** — the dependency is developed in this monorepo and has a
   valid version. All instances are compared against it.
3. **Catalog** — any instance uses the `catalog:` protocol, which wins over
   semver comparison.
4. **Registry updates** — eligible updates were found on the npm registry
   (only relevant to `HighestSemver` / `LowestSemver` groups via
   `preferred_semver.rs`).
5. **Semver comparison** — the main version-group logic.
6. **No semver** — no instance has a parseable semver version. Fall through to
   the non-semver branch.

Not every visitor implements all branches. `sameMinor`, `pinned`, `banned`,
etc. only implement the branches that are relevant to their policy.

---

## Non-Semver Instances

When none of the instances in a group have a parseable semver version (i.e.
`get_highest_or_lowest_specifier()` or equivalent returns `None`), the visitor
falls through to the non-semver branch:

- **All identical** → `ValidInstance::IsNonSemverButIdentical`
- **Any differ** → `UnfixableInstance::NonSemverMismatch` for ALL instances

There is **no per-instance split** between semver and non-semver instances in a
mixed group. If even one non-semver instance prevents a highest/lowest specifier
from being found, ALL instances — including any semver ones — fall into the
non-semver branch. This mirrors `preferred_semver.rs` exactly.

---

## Semver Group Interaction

A semver group assigns a preferred semver range to matched instances via
`instance.preferred_semver_range: Option<SemverRange>`. Visitors must respect
this preference when it exists.

### The satisfies-check pattern

When an instance has a semver group (`must_match_preferred_semver_range()` is
true), the visitor must determine whether the preferred range is compatible with
the version group's goal. The standard pattern:

```
does preferred range applied to instance satisfy the version group's target?
├── yes, and instance already matches preferred range → Valid (satisfies)
├── yes, and instance does not match preferred range → Fixable (SemverRangeMismatch)
├── no,  and instance already matches preferred range → Conflict (Match...)
└── no,  and instance does not match preferred range → Conflict (Mismatch...)
```

`specifier_with_preferred_semver_range_will_satisfy(target)` is the method that
performs the satisfies-check.

### When the version group's target has a specific range

`preferred_semver.rs` uses `must_match_preferred_semver_range_which_is_not(
range_of_target)` to gate the semver group subtree — the semver group only
"matters" when it prefers a *different* range to the one already used by the
target specifier. If the preferred range matches the target's range, fall
through to the simpler identical/differs check.

### Fix target for instances that differ

When an instance's version number differs from the target AND it has a semver
group, the fix target is: `target.with_range(preferred_range)`. This applies
the semver group's preferred range to the target version. If `with_range`
returns `None` (unsupported combination), fall back to the bare target.

See `preferred_semver.rs` L183-193 for the canonical implementation.

---

## sameMinor-Specific Semver Group Rules

`sameMinor` divides semver ranges into two categories before applying the
satisfies-check pattern:

### Safe ranges — always satisfy the sameMinor constraint

| Range | Example | Stays within MAJOR.MINOR? |
|---|---|---|
| `Exact` | `1.2.3` | ✅ |
| `Patch` / `~` | `~1.2.3` | ✅ |

For safe ranges, the satisfies-check is unnecessary — safe ranges always satisfy
sameMinor by definition. The only question is whether the instance's on-disk
range already matches what the semver group prefers.

### Unsafe ranges — always violate the sameMinor constraint

| Range | Example | Can resolve outside MAJOR.MINOR? |
|---|---|---|
| `Minor` / `^` | `^1.2.3` | ✅ (can resolve `1.3.x`) |
| `Any` / `*` | `*` | ✅ |
| `Gt` / `>` | `>1.2.3` | ✅ |
| `Gte` / `>=` | `>=1.2.3` | ✅ |
| `Lt` / `<` | `<1.2.3` | ✅ |
| `Lte` / `<=` | `<=1.2.3` | ✅ |

For unsafe ranges, **sameMinor policy wins unconditionally** — no satisfies-
check is needed. The fix is always: replace the unsafe range with `~`, keeping
the same version number.

### sameMinor semver group decision table

| has semver group? | preferred range | on-disk range | status |
|---|---|---|---|
| no | — | safe or none | `SatisfiesSameMinorGroup` |
| no | — | unsafe | `SameMinorOverridesSemverRange` |
| yes | safe | matches preferred | `SatisfiesSameMinorGroup` |
| yes | safe | does not match | `SemverRangeMismatch` |
| yes | unsafe | matches preferred | `SameMinorOverridesSemverRange` |
| yes | unsafe | does not match | `SameMinorOverridesSemverRangeMismatch` |

Note: `SameMinorOverridesSemverRange` applies whether or not the instance has a
semver group. "Matches its semver group" in the status annotation means "the
on-disk range matches what is expected — either the semver group's preference if
one exists, or there is no semver group and the on-disk range is all there is."

---

## Fix Target Construction

### When all instances share the same version number

The fix target is the current specifier with the preferred range applied:
`specifier.with_range(preferred_range)`.

### When version numbers differ (e.g. DiffersToHighest...)

The fix target is the winning specifier with the preferred range applied:
`winner.with_range(preferred_range).unwrap_or(winner)`. This is computed
**per-instance**, not once globally, because different instances may belong to
different semver groups with different preferred ranges.

### Greedy-range tiebreaker for highest/lowest selection

When selecting the highest (or lowest) specifier, each instance's preferred
semver range is applied before comparison. This means a semver group that widens
a range (e.g. exact → caret) can promote that instance to be the "highest" via
range greediness. See `Dependency::get_highest_or_lowest_specifier`.

---

## Pre-loop vs Per-instance Work

Some computations must be performed **once per dependency** before the
per-instance loop, not once per instance:

- **Highest/lowest specifier** — call `get_highest_or_lowest_specifier()` once,
  then reference the result for every instance.
- **Major-mismatch gate** (`sameMinor`) — call `already_has_same_major_as_all`
  once, then use the boolean result inside the loop. Avoids O(n²) re-checking.
- **Minor-mismatch gate** (`sameMinor`) — similarly, determine once whether all
  instances share the same MAJOR.MINOR before entering the loop.

---

## Status Semantics

### Match vs Mismatch in conflict/override names

The `Match` / `Mismatch` prefix on status names refers to whether the
**instance's on-disk range already matches its semver group's preferred range**:

- `Match...` — on-disk range = preferred range (the semver group is currently
  satisfied, but it conflicts with the version group)
- `Mismatch...` — on-disk range ≠ preferred range (the semver group is not
  currently satisfied, AND it would conflict with the version group if fixed)

### Valid "satisfies" vs "identical" distinction

- `IsHighestOrLowestSemver` / `IsIdenticalToLocal` etc. — instance is byte-for-
  byte identical to the target specifier
- `SatisfiesHighestOrLowestSemver` / `SatisfiesLocal` etc. — instance has the
  same version number as the target but uses a different (compatible) range via
  a semver group; considered a loose match that should be highlighted

### Unfixable vs Conflict

- `Invalid::Unfixable` — syncpack cannot determine the correct value without
  human input (e.g. multiple non-semver versions, major version crossing)
- `Invalid::Conflict` — two active rules are simultaneously unsatisfiable;
  syncpack knows what each rule wants but cannot satisfy both

---

## Consistency Checklist for New Visitors

Before marking a visitor implementation complete, verify:

- [ ] Every instance is marked before the visitor returns
- [ ] Non-semver branch mirrors `preferred_semver.rs`: no per-instance semver/
      non-semver split; all fall into identical-or-mismatch together
- [ ] Semver group interaction uses the satisfies-check pattern (or the
      sameMinor override pattern) — not an ad-hoc check
- [ ] Fix targets for differing instances apply the instance's preferred range
      to the winning specifier, not the winning specifier's own range
- [ ] Expensive per-dependency computations (highest specifier, major-mismatch
      boolean, etc.) are computed once before the per-instance loop
- [ ] New `InstanceState` variants have doc-comment annotations following the
      existing bullet-point format in `instance_state.rs`
- [ ] Tests cover every leaf of the decision tree, including semver group
      interaction (safe range match, safe range mismatch, unsafe range match,
      unsafe range mismatch)