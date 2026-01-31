# Example: Adding a New InstanceState Variant

<scenario>
This guide walks through adding a new `InstanceState::Invalid::Fixable` variant called `DiffersFromPreferred` that marks instances that don't match a user's preferred version format (e.g., they want all ranges to use `^` but some use `~`).
</scenario>

<state_hierarchy>

## Step 1: Understand the State Hierarchy

InstanceState has this structure:

```
InstanceState
├── Unknown
├── Valid(ValidInstance)
├── Invalid(InvalidInstance)
│   ├── Fixable(FixableInstance)      ← We're adding here
│   ├── Unfixable(UnfixableInstance)
│   └── Conflict(SemverGroupAndVersionConflict)
└── Suspect(SuspectInstance)
```

**Decision:** This is fixable because we know what the user wants (their preferred format).

</state_hierarchy>

<step number="2">

## Step 2: Add Enum Variant

**File:** `src/instance_state.rs`

Find the `FixableInstance` enum and add your variant:

```rust
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FixableInstance {
  /// - ✘ Instance is in a banned version group
  IsBanned,
  /// - ✘ Instance mismatches the version of its locally-developed package
  DiffersToLocal,
  /// - ✘ Instance mismatches highest/lowest semver in its group
  DiffersToHighestOrLowestSemver,
  /// - ✘ Instance is older than highest semver published to the registry
  DiffersToNpmRegistry,
  /// - ✘ Instance mismatches the matching snapTo instance
  DiffersToSnapTarget,
  /// - ✘ Instance mismatches the pinned version
  DiffersToPinnedVersion,
  /// - ✘ Instance's range doesn't satisfy all other ranges in same range group
  DiffersToSameRange,
  /// - ✘ Instance mismatches the same minor group
  DiffersToSameMinor,
  /// - ✘ Instance doesn't use the preferred version format  ← Add this
  DiffersFromPreferred,
}
```

<pattern_notes>

**Pattern notes:**

- Use doc comments with `///` and checkboxes (✓ ✘ ?)
- Use `Differs` prefix for mismatches
- Use `Is` prefix for states
- Be descriptive and specific

</pattern_notes>

</step>

<step number="3">

## Step 3: Update Severity (If Needed)

Check if `get_severity()` needs updating. For our case, all `Fixable` variants return the same severity, so no change needed.

If you added to `Valid`, `Unfixable`, or `Suspect`, you might need to add a match arm:

```rust
impl InstanceState {
  pub fn get_severity(&self) -> u8 {
    match self {
      InstanceState::Unknown => 0,
      InstanceState::Valid(variant) => match variant {
        ValidInstance::SomeSpecialCase => 5,  // Example
        _ => 1,
      },
      InstanceState::Invalid(_) => 2,
      InstanceState::Suspect(_) => 3,
    }
  }
}
```

</step>

<step number="4">

## Step 4: Add Detection Logic

Create a new file or add to existing validation logic.

**File:** `src/visit_packages/preferred_format.rs` (new file)

<detection_logic>

```rust
use {
  crate::{
    dependency::Dependency,
    instance_state::{FixableInstance::*, InstanceState, ValidInstance::*},
    specifier::{Specifier, semver_range::SemverRange},
  },
};

/// Visit instances and check if they use the preferred version format
pub fn visit(dependency: &Dependency, preferred_range: &SemverRange) {
  dependency.instances.iter().for_each(|instance| {
    let mut state = instance.state.borrow_mut();

    // Skip if already invalid for another reason
    if state.is_invalid() {
      return;
    }

    // Check if instance uses preferred format
    match &instance.specifier {
      Specifier::BasicSemver(basic) => {
        // Check if range matches preferred
        if let Some(range) = &basic.semver_range {
          if range != preferred_range {
            // Found a mismatch - mark as fixable
            *state = InstanceState::fixable(DiffersFromPreferred);

            // Optionally set what it should be
            // (implementation depends on your data structure)
          }
        }
      }
      _ => {
        // Other specifier types are valid for now
        if matches!(*state, InstanceState::Unknown) {
          *state = InstanceState::valid(IsHighestOrLowestSemver);
        }
      }
    }
  });
}
```

</detection_logic>

</step>

<step number="5">

## Step 5: Integrate into visit_packages

**File:** `src/visit_packages.rs`

Add your module and call it:

<integration>

```rust
mod banned;
mod ignored;
mod indent;
mod pinned;
mod preferred_format;  // ← Add this
mod preferred_semver;
mod same_minor;
mod same_range;
mod snapped_to;

pub fn visit_packages(ctx: Context) -> Context {
  ctx.version_groups.iter().sorted_by(order_snapped_to_groups_last).for_each(|group| {
    group.dependencies.values().for_each(|dependency| match dependency.variant {
      VersionGroupVariant::Banned => banned::visit(dependency),
      VersionGroupVariant::HighestSemver | VersionGroupVariant::LowestSemver => {
        preferred_semver::visit(dependency, &ctx);
        // Optionally check format after
        if let Some(preferred_range) = &ctx.config.rcfile.preferred_range {
          preferred_format::visit(dependency, preferred_range);
        }
      }
      VersionGroupVariant::Ignored => ignored::visit(dependency),
      VersionGroupVariant::Pinned => pinned::visit(dependency),
      VersionGroupVariant::SameRange => same_range::visit(dependency),
      VersionGroupVariant::SameMinor => same_minor::visit(dependency),
      VersionGroupVariant::SnappedTo => snapped_to::visit(dependency, &ctx),
    });
  });
  ctx
}
```

</integration>

</step>

<step number="6">

## Step 6: Write Tests

**File:** `src/visit_packages/preferred_format_test.rs`

<test_examples>

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
fn marks_instances_with_wrong_range_as_fixable() {
  let ctx = TestBuilder::new()
    .with_config(json!({
      "semverGroups": [
        {
          "dependencies": ["**"],
          "range": "^"
        }
      ]
    }))
    .with_packages(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "react": "~18.0.0",      // Wrong range
          "lodash": "^4.17.21"     // Correct range
        }
      }),
    ])
    .build_and_visit_packages();

  expect(&ctx).to_have_instances(vec![
    ExpectedInstance {
      state: InstanceState::fixable(DiffersFromPreferred),
      dependency_name: "react",
      id: "react in /dependencies of package-a",
      actual: "~18.0.0",
      expected: Some("^18.0.0"),  // What it should be
      overridden: None,
    },
    ExpectedInstance {
      state: InstanceState::valid(IsHighestOrLowestSemver),
      dependency_name: "lodash",
      id: "lodash in /dependencies of package-a",
      actual: "^4.17.21",
      expected: Some("^4.17.21"),
      overridden: None,
    },
  ]);
}

#[test]
fn allows_exact_versions_when_no_range_preferred() {
  let ctx = TestBuilder::new()
    .with_packages(vec![
      json!({
        "name": "package-a",
        "dependencies": {
          "react": "18.0.0"  // Exact version, no range
        }
      }),
    ])
    .build_and_visit_packages();

  // Should be valid since no preference specified
  let react = ctx.instances.iter()
    .find(|i| i.dependency.name == "react")
    .unwrap();
  assert!(react.is_valid());
}

#[test]
fn handles_multiple_packages_with_different_ranges() {
  let ctx = TestBuilder::new()
    .with_config(json!({
      "semverGroups": [
        {
          "dependencies": ["**"],
          "range": "^"
        }
      ]
    }))
    .with_packages(vec![
      json!({"name": "pkg-a", "dependencies": {"react": "~18.0.0"}}),
      json!({"name": "pkg-b", "dependencies": {"react": ">=18.0.0"}}),
      json!({"name": "pkg-c", "dependencies": {"react": "^18.0.0"}}),
    ])
    .build_and_visit_packages();

  // pkg-a and pkg-b should be marked fixable
  // pkg-c should be valid
  let instances: Vec<_> = ctx.instances.iter()
    .filter(|i| i.dependency.name == "react")
    .collect();

  assert_eq!(instances.len(), 3);

  let pkg_a_react = instances.iter().find(|i| i.package.name == "pkg-a").unwrap();
  assert!(pkg_a_react.is_fixable());

  let pkg_b_react = instances.iter().find(|i| i.package.name == "pkg-b").unwrap();
  assert!(pkg_b_react.is_fixable());

  let pkg_c_react = instances.iter().find(|i| i.package.name == "pkg-c").unwrap();
  assert!(pkg_c_react.is_valid());
}
```

</test_examples>

</step>

<step number="7">

## Step 7: Update Commands to Handle New State

Commands like `fix` and `lint` already handle all fixable states generically, so they'll automatically work:

<automatic_handling>

```rust
// In fix.rs, this already handles your new state
dependency.get_sorted_instances()
  .filter(|instance| instance.is_fixable())  // ← Includes DiffersFromPreferred
  .for_each(|instance| {
    // Fix the instance
  });
```

</automatic_handling>

If you need special handling:

<custom_handling>

```rust
match &instance.state {
  InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::DiffersFromPreferred)) => {
    // Special handling for this state
  }
  _ => {
    // Default handling
  }
}
```

</custom_handling>

</step>

<step number="8">

## Step 8: Test Locally

```bash
# Run your specific tests
cargo test preferred_format

# Run all tests
just test

# Test against fixture
cd fixtures/fluid-framework
cargo run -- lint
```

</step>

<step number="9">

## Step 9: Update Documentation

Add your state to the quick reference:

**File:** `QUICK_REFERENCE.md`

```markdown
// Invalid::Fixable (can auto-fix)
IsBanned // In banned version group
DiffersToLocal // Mismatches local package
DiffersToHighestOrLowestSemver // Wrong version per group policy
DiffersToNpmRegistry // Older than npm registry
DiffersToSnapTarget // Mismatches snap target
DiffersToPinnedVersion // Mismatches pinned version
DiffersToSameRange // Range doesn't satisfy all other ranges in group
DiffersToSameMinor // Minor version differs in group
DiffersFromPreferred // Doesn't use preferred format ← Add this
```

</step>

<checklist>

## Checklist

- [x] Added variant to appropriate enum in `src/instance_state.rs`
- [x] Updated `get_severity()` if needed
- [x] Created detection logic in `src/visit_packages/*.rs`
- [x] Integrated into `visit_packages()` function
- [x] Written comprehensive tests
- [x] Verified commands handle it correctly
- [x] Tested locally
- [x] Updated documentation

</checklist>

<variations>

## Common Variations

<variation type="valid_state">

### Adding a Valid State

```rust
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ValidInstance {
  IsIgnored,
  IsLocalAndValid,
  IsMyNewValidState,  // ← Add here
  // ...
}
```

Then assign it when instance passes validation:

```rust
*state = InstanceState::valid(IsMyNewValidState);
```

</variation>

<variation type="unfixable_state">

### Adding an Unfixable State

When you can't determine the correct value:

```rust
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UnfixableInstance {
  DependsOnInvalidLocalPackage,
  NonSemverMismatch,
  MyAmbiguousCase,  // ← Add here when unclear what's right
  // ...
}
```

</variation>

<variation type="suspect_state">

### Adding a Suspect State

When user has misconfigured something:

```rust
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SuspectInstance {
  RefuseToBanLocal,
  RefuseToPinLocal,
  RefuseToDoSomethingLocal,  // ← Add here for misconfigurations
  // ...
}
```

</variation>

</variations>

<troubleshooting>

## Troubleshooting

<problem>
**Problem:** State not being assigned
→ Check that your detection logic is actually running (add debug prints)
→ Verify the conditions in your match arms are correct
</problem>

<problem>
**Problem:** Tests failing
→ Make sure `build_and_visit_packages()` is called (assigns states)
→ Check that expected state matches what you're assigning
</problem>

<problem>
**Problem:** State assigned but not showing in output
→ Commands filter by `is_invalid()` - make sure your variant is included
</problem>

<problem>
**Problem:** Multiple states being assigned
→ Check order of validation - earlier checks might override yours
→ Add conditions to skip if already invalid
</problem>

</troubleshooting>

<related_files>

## Related Files

- `src/instance_state.rs` - All state definitions
- `src/visit_packages.rs` - Where states are assigned
- `src/visit_packages/*_test.rs` - Test examples
- `src/commands/fix.rs` - How states are processed

</related_files>
