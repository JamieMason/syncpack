# Quick Reference: Common Code Patterns

<purpose>
This is a quick lookup guide for the most common patterns when working on Syncpack.
</purpose>

<enum_variants>

## Core Enum Variants

<instance_state_variants>

### InstanceState Quick Reference

<valid_states>

```rust
// Valid states (14 variants)
IsIgnored                          // Configured to be ignored
IsLocalAndValid                    // Local package with valid version
IsIdenticalToLocal                 // Matches local package exactly
IsMatchingLocal                    // Matches local package (loose)
IsHighestOrLowestSemver            // Correct per version group policy
IsPinned                           // Matches pinned version
IsSnappedTo                        // Matches snap target
IsSameRange                        // Range satisfies all other ranges in group
// ... more Valid variants
```

</valid_states>

<fixable_states>

```rust
// Invalid::Fixable (can auto-fix)
IsBanned                           // In banned version group
DiffersToLocal                     // Mismatches local package
DiffersToHighestOrLowestSemver     // Wrong version per group policy
DiffersToNpmRegistry               // Older than npm registry
DiffersToSnapTarget                // Mismatches snap target
DiffersToPinnedVersion             // Mismatches pinned version
// ... more Fixable variants
```

</fixable_states>

<unfixable_states>

```rust
// Invalid::Unfixable (needs human decision)
DependsOnInvalidLocalPackage       // Local package has invalid version
NonSemverMismatch                  // Can't determine correct version
SameRangeMismatch                  // Range doesn't satisfy all others, no semver group to guide fix
// ... more Unfixable variants
```

</unfixable_states>

<suspect_states>

```rust
// Suspect (misconfiguration)
RefuseToBanLocal                   // Can't ban local package
RefuseToPinLocal                   // Can't pin local package
RefuseToSnapLocal                  // Can't snap local package
// ... more Suspect variants
```

</suspect_states>

</instance_state_variants>

<version_group_variants>

### VersionGroupVariant Quick Reference

```rust
Banned        // Dependencies that shouldn't exist
HighestSemver // Use highest version across monorepo
LowestSemver  // Use lowest version across monorepo
Pinned        // Lock to specific version
SameRange     // All ranges must satisfy every other range in the group
SameMinor     // All must use same minor version
SnappedTo     // Follow version from specific package
Ignored       // Skip validation
```

</version_group_variants>

<specifier_variants>

### Specifier Quick Reference

<main_specifier_variants>

```rust
// Main enum variants
Specifier::Alias(Alias)                    // npm:package@version
Specifier::BasicSemver(BasicSemver)        // Simple semver: "1.2.3", "^1.2.3", "~1.2.3"
Specifier::ComplexSemver(ComplexSemver)    // Complex ranges: ">=1.0.0 <2.0.0"
Specifier::File(Raw)                       // file:../path/to/package
Specifier::Git(Git)                        // git://github.com/user/repo.git
Specifier::None                            // Empty/missing version
Specifier::Tag(Raw)                        // Named tags: "latest", "next", "beta"
Specifier::Unsupported(Raw)                // Unrecognized format
Specifier::Url(Raw)                        // http://example.com/package.tgz
Specifier::WorkspaceProtocol(WorkspaceProtocol) // workspace:*, workspace:^, workspace:~
```

</main_specifier_variants>

<basic_semver_variant>

```rust
// BasicSemverVariant
BasicSemverVariant::Latest    // "*"
BasicSemverVariant::Major     // "1"
BasicSemverVariant::Minor     // "1.2"
BasicSemverVariant::Patch     // "1.2.3"
```

</basic_semver_variant>

<semver_range>

```rust
// SemverRange (for BasicSemver)
SemverRange::Any      // *
SemverRange::Minor    // ^1.4.2
SemverRange::Exact    // 1.4.2
SemverRange::Gt       // >1.4.2
SemverRange::Gte      // >=1.4.2
SemverRange::Lt       // <1.4.2
SemverRange::Lte      // <=1.4.2
SemverRange::Patch    // ~1.4.2
```

</semver_range>

</specifier_variants>

</enum_variants>

<iteration_patterns>

## Common Iteration Patterns

<basic_iteration>

### Basic iteration over all instances

```rust
ctx.version_groups.iter().for_each(|group| {
    group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
        dependency.get_sorted_instances().for_each(|instance| {
            // Process each instance
        });
    });
});
```

</basic_iteration>

<filter_invalid>

### Filter and process invalid instances

```rust
ctx.version_groups.iter().for_each(|group| {
    group.get_sorted_dependencies(&sort_by).for_each(|dependency| {
        dependency.get_sorted_instances()
            .filter(|instance| instance.is_invalid())
            .for_each(|instance| {
                // Handle invalid instance
            });
    });
});
```

</filter_invalid>

<filter_fixable>

### Process fixable instances only

```rust
dependency.get_sorted_instances()
    .filter(|instance| instance.is_fixable())
    .for_each(|instance| {
        // Auto-fix this instance
    });
```

</filter_fixable>

<strict_mode>

### Handle strict mode (invalid + suspect)

```rust
dependency.get_sorted_instances()
    .filter(|instance| {
        instance.is_invalid() || (instance.is_suspect() && ctx.config.rcfile.strict)
    })
    .for_each(|instance| {
        // Handle issues
    });
```

</strict_mode>

</iteration_patterns>

<test_patterns>

## Test Patterns

<basic_test>

### Basic test with TestBuilder

```rust
#[test]
fn test_description() {
    let ctx = TestBuilder::new()
        .with_packages(vec![
            json!({
                "name": "pkg-a",
                "version": "1.0.0",
                "dependencies": { "react": "17.0.0" }
            }),
        ])
        .with_version_group(json!({
            "dependencies": ["react"],
            "pinned": "18.0.0"
        }))
        .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
            state: InstanceState::fixable(DiffersToPinnedVersion),
            dependency_name: "react",
            id: "react in /dependencies of pkg-a",
            actual: "17.0.0",
            expected: Some("18.0.0"),
            overridden: None,
        },
    ]);
}
```

</basic_test>

<multiple_packages>

### Test with multiple packages

```rust
#[test]
fn test_multiple_packages() {
    let ctx = TestBuilder::new()
        .with_packages(vec![
            json!({"name": "pkg-a", "dependencies": {"lodash": "4.17.0"}}),
            json!({"name": "pkg-b", "dependencies": {"lodash": "4.18.0"}}),
        ])
        .with_version_group(json!({
            "dependencies": ["lodash"],
            "policy": "highestSemver"
        }))
        .build_and_visit_packages();

    // Test that pkg-a's lodash is marked as needing update
}
```

</multiple_packages>

<custom_config>

### Test with custom config

```rust
#[test]
fn test_with_custom_types() {
    let ctx = TestBuilder::new()
        .with_config(json!({
            "customTypes": {
                "packageManager": {
                    "strategy": "name@version",
                    "path": "packageManager"
                }
            }
        }))
        .with_package(json!({
            "name": "pkg-a",
            "packageManager": "pnpm@7.0.0"
        }))
        .with_version_group(json!({
            "dependencies": ["pnpm"],
            "dependencyTypes": ["packageManager"],
            "pinned": "8.0.0"
        }))
        .build_and_visit_packages();
}
```

</custom_config>

<registry_updates>

### Test with registry updates

```rust
#[test]
fn test_with_npm_registry() {
    let ctx = TestBuilder::new()
        .with_package(json!({
            "name": "pkg-a",
            "dependencies": {"react": "17.0.0"}
        }))
        .with_registry_updates(json!({
            "react": ["17.0.0", "17.0.1", "18.0.0", "18.1.0"]
        }))
        .with_update_target(UpdateTarget::Latest)
        .build_and_visit_packages();
}
```

</registry_updates>

</test_patterns>

<command_patterns>

## Command Implementation Pattern

<standard_command>

### Standard command structure

```rust
use crate::{commands::ui, context::Context};

pub fn run(ctx: Context) -> i32 {
    let mut has_issues = false;

    ctx.version_groups.iter().for_each(|group| {
        let mut has_printed_group = false;

        group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
            let mut has_printed_dependency = false;

            dependency.get_sorted_instances()
                .filter(|instance| instance.is_invalid())
                .for_each(|instance| {
                    // Lazy print headers
                    if !has_printed_group {
                        ui::group::print_header(&ctx, group);
                        has_printed_group = true;
                    }
                    if !has_printed_dependency {
                        ui::dependency::print(&ctx, dependency, &group.variant);
                        has_printed_dependency = true;
                    }

                    // Process instance
                    ui::instance::print(&ctx, instance, &group.variant);
                    has_issues = true;
                });
        });
    });

    if has_issues {
        1  // Exit with error
    } else {
        ui::util::print_no_issues_found();
        0  // Exit successfully
    }
}
```

</standard_command>

<file_modification>

### Command that modifies files

```rust
pub fn run(mut ctx: Context) -> i32 {
    let mut changed_count = 0;

    ctx.version_groups.iter().for_each(|group| {
        group.get_sorted_dependencies(&ctx.config.cli.sort).for_each(|dependency| {
            dependency.get_sorted_instances()
                .filter(|instance| instance.is_fixable())
                .for_each(|instance| {
                    // Get the expected value
                    if let Some(expected) = instance.get_expected() {
                        // Update package.json
                        instance.set_value(&mut ctx, expected);
                        changed_count += 1;
                    }
                });
        });
    });

    // Write changes to disk
    ctx.packages.write_all();

    if changed_count > 0 {
        println!("Fixed {} instances", changed_count);
        0
    } else {
        println!("No changes needed");
        0
    }
}
```

</file_modification>

</command_patterns>

<json_patterns>

## JSON Patterns for Tests

<version_group_configs>

### Version group configurations

```rust
// Pinned version
json!({
    "dependencies": ["react"],
    "pinned": "18.0.0"
})

// Highest semver
json!({
    "dependencies": ["lodash"],
    "policy": "highestSemver"
})

// Banned
json!({
    "dependencies": ["jquery"],
    "isBanned": true
})

// Snap to another package
json!({
    "dependencies": ["@my-org/*"],
    "snapTo": ["root-package"]
})

// Same range
json!({
    "dependencies": ["typescript"],
    "policy": "sameRange"
})

// With dependency types
json!({
    "dependencies": ["react"],
    "dependencyTypes": ["prod", "dev"],
    "pinned": "18.0.0"
})

// With package filter
json!({
    "dependencies": ["eslint"],
    "packages": ["@my-org/pkg-*"],
    "pinned": "8.0.0"
})
```

</version_group_configs>

<package_json_structures>

### Package.json structures

```rust
// Basic package
json!({
    "name": "my-package",
    "version": "1.0.0"
})

// With dependencies
json!({
    "name": "my-package",
    "version": "1.0.0",
    "dependencies": {
        "react": "18.0.0",
        "lodash": "4.17.21"
    },
    "devDependencies": {
        "typescript": "5.0.0"
    }
})

// With workspace protocol
json!({
    "name": "my-package",
    "dependencies": {
        "@my-org/shared": "workspace:*"
    }
})

// With custom types
json!({
    "name": "my-package",
    "packageManager": "pnpm@8.0.0",
    "engines": {
        "node": ">=18.0.0"
    }
})
```

</package_json_structures>

</json_patterns>

<state_checking>

## State Checking Methods

```rust
// Check instance state
instance.is_valid()      // Valid variant
instance.is_invalid()    // Invalid variant (any)
instance.is_fixable()    // Invalid::Fixable
instance.is_unfixable()  // Invalid::Unfixable
instance.is_suspect()    // Suspect variant

// Get state details
instance.state.get_name()      // String representation
instance.state.get_severity()  // 0-100, higher = more severe

// Check specific states
match &instance.state {
    InstanceState::Valid(ValidInstance::IsLocalAndValid) => { /* ... */ }
    InstanceState::Invalid(InvalidInstance::Fixable(FixableInstance::IsBanned)) => { /* ... */ }
    _ => {}
}
```

</state_checking>

<expected_instance>

## ExpectedInstance Pattern

```rust
ExpectedInstance {
    state: InstanceState::fixable(DiffersToPinnedVersion),
    dependency_name: "react",
    id: "react in /dependencies of package-a",  // Format: "{dep} in {location} of {package}"
    actual: "17.0.0",
    expected: Some("18.0.0"),
    overridden: None,  // Or Some("reason") if version was overridden
}
```

</expected_instance>

<running_tests>

## Running Tests

```bash
# Run all tests
just test
cargo test

# Run specific test
cargo test test_name

# Run tests in a file
cargo test --test banned_test

# Run with output
cargo test test_name -- --nocapture

# Run and show ignored tests
cargo test -- --ignored

# Run with coverage
just coverage
```

</running_tests>

<debugging>

## Debug Printing

```rust
use log::debug;

// Debug a value
debug!("Context: {:#?}", ctx);

// Debug with formatting
debug!("Found {} instances", count);

// Conditional debug
if ctx.config.cli.verbose {
    debug!("Detailed info: {:#?}", instance);
}
```

</debugging>

<file_paths>

## Common File Paths in Tests

<location_strings>

```rust
// Location strings follow pattern: /{property-path} of {package-name}
"/version of package-a"
"/dependencies of package-b"
"/devDependencies of package-c"
"/peerDependencies of package-d"
"/packageManager of package-e"
"/engines/node of package-f"
"/custom/nested/path of package-g"
```

</location_strings>

</file_paths>

<cargo_commands>

## Cargo Commands

```bash
# Build
cargo build
cargo build --release

# Run
cargo run -- --help
cargo run -- lint
cargo run -- fix --dry-run

# Test
cargo test
cargo test --lib          # Only unit tests
cargo test --test name    # Specific integration test

# Check (faster than build)
cargo check

# Format
cargo fmt

# Lint
cargo clippy
```

</cargo_commands>

<just_commands>

## Just Commands

```bash
just                # List all commands
just test          # Run tests
just lint          # Run linting
just format        # Format code
just coverage      # Generate coverage report
just benchmark     # Run benchmarks
just build         # Build release
just clean         # Clean build artifacts
```

</just_commands>
