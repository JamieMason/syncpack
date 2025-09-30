# Example: Writing Tests with TestBuilder

This guide shows you how to write effective tests for Syncpack using the TestBuilder pattern.

## The TestBuilder Pattern

**Always use TestBuilder** - Never manually construct Context in tests. TestBuilder provides a fluent API that handles all the complexity of setting up test scenarios.

## Basic Test Structure

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
fn test_descriptive_name() {
    // 1. Build the test scenario
    let ctx = TestBuilder::new()
        .with_packages(vec![
            json!({
                "name": "package-a",
                "version": "1.0.0",
                "dependencies": { "react": "17.0.0" }
            }),
        ])
        .with_version_group(json!({
            "dependencies": ["react"],
            "pinned": "18.0.0"
        }))
        .build_and_visit_packages();

    // 2. Assert expectations
    expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
            state: InstanceState::fixable(DiffersToPinnedVersion),
            dependency_name: "react",
            id: "react in /dependencies of package-a",
            actual: "17.0.0",
            expected: Some("18.0.0"),
            overridden: None,
        },
    ]);
}
```

## TestBuilder Methods

### Adding Packages

```rust
// Single package
.with_package(json!({
    "name": "my-package",
    "version": "1.0.0"
}))

// Multiple packages
.with_packages(vec![
    json!({"name": "pkg-a", "dependencies": {"lodash": "4.17.0"}}),
    json!({"name": "pkg-b", "dependencies": {"lodash": "4.18.0"}}),
])
```

### Adding Version Groups

```rust
// Pinned version
.with_version_group(json!({
    "dependencies": ["react"],
    "pinned": "18.0.0"
}))

// Multiple version groups
.with_version_groups(vec![
    json!({"dependencies": ["react"], "pinned": "18.0.0"}),
    json!({"dependencies": ["lodash"], "policy": "highestSemver"}),
])
```

### Adding Semver Groups

```rust
.with_semver_group(json!({
    "dependencies": ["**"],
    "range": "^"
}))
```

### Custom Configuration

```rust
.with_config(json!({
    "customTypes": {
        "packageManager": {
            "strategy": "name@version",
            "path": "packageManager"
        }
    }
}))
```

### Strict Mode

```rust
.with_strict(true)  // Treat Suspect states as errors
```

### Registry Updates

```rust
.with_registry_updates(json!({
    "react": ["17.0.0", "17.0.1", "18.0.0", "18.2.0"],
    "lodash": ["4.17.0", "4.17.21"]
}))
```

### Update Target

```rust
use crate::cli::UpdateTarget;

.with_update_target(UpdateTarget::Latest)
.with_update_target(UpdateTarget::Minor)
.with_update_target(UpdateTarget::Patch)
```

### Build Methods

```rust
// Build Context without visiting (rare)
.build()

// Build Context and run visit_packages (most common)
.build_and_visit_packages()
```

## Common Test Patterns

### Testing Banned Dependencies

```rust
#[test]
fn removes_banned_dependency() {
    let ctx = TestBuilder::new()
        .with_packages(vec![
            json!({
                "name": "pkg-a",
                "dependencies": {"jquery": "3.6.0"}
            }),
        ])
        .with_version_group(json!({
            "dependencies": ["jquery"],
            "isBanned": true
        }))
        .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
            state: InstanceState::fixable(IsBanned),
            dependency_name: "jquery",
            id: "jquery in /dependencies of pkg-a",
            actual: "3.6.0",
            expected: Some(""),  // Empty means remove
            overridden: None,
        },
    ]);
}
```

### Testing Version Mismatches

```rust
#[test]
fn detects_version_mismatch_across_packages() {
    let ctx = TestBuilder::new()
        .with_packages(vec![
            json!({"name": "pkg-a", "dependencies": {"lodash": "4.17.0"}}),
            json!({"name": "pkg-b", "dependencies": {"lodash": "4.18.0"}}),
        ])
        .with_version_group(json!({
            "dependencies": ["lodash"],
            "policy": "sameRange"
        }))
        .build_and_visit_packages();

    // Both will be marked - one as reference, one as mismatch
    let instances: Vec<_> = ctx.instances.iter()
        .filter(|i| i.dependency.name == "lodash")
        .collect();

    assert_eq!(instances.len(), 2);
    assert!(instances.iter().any(|i| i.is_invalid()));
}
```

### Testing Local Package Versions

```rust
#[test]
fn validates_local_package_version() {
    let ctx = TestBuilder::new()
        .with_packages(vec![
            json!({
                "name": "my-lib",
                "version": "2.0.0"
            }),
            json!({
                "name": "my-app",
                "dependencies": {"my-lib": "1.0.0"}  // Wrong version
            }),
        ])
        .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
            state: InstanceState::valid(IsLocalAndValid),
            dependency_name: "my-lib",
            id: "my-lib in /version of my-lib",
            actual: "2.0.0",
            expected: Some("2.0.0"),
            overridden: None,
        },
        ExpectedInstance {
            state: InstanceState::fixable(DiffersToLocal),
            dependency_name: "my-lib",
            id: "my-lib in /dependencies of my-app",
            actual: "1.0.0",
            expected: Some("2.0.0"),  // Should match local version
            overridden: None,
        },
    ]);
}
```

### Testing Workspace Protocol

```rust
#[test]
fn handles_workspace_protocol() {
    let ctx = TestBuilder::new()
        .with_packages(vec![
            json!({
                "name": "my-lib",
                "version": "1.0.0"
            }),
            json!({
                "name": "my-app",
                "dependencies": {"my-lib": "workspace:*"}
            }),
        ])
        .build_and_visit_packages();

    let workspace_instance = ctx.instances.iter()
        .find(|i| i.dependency.name == "my-lib" && i.package.name == "my-app")
        .unwrap();

    assert!(workspace_instance.is_valid());
}
```

### Testing Custom Dependency Types

```rust
#[test]
fn validates_custom_package_manager_field() {
    let ctx = TestBuilder::new()
        .with_config(json!({
            "customTypes": {
                "packageManager": {
                    "strategy": "name@version",
                    "path": "packageManager"
                }
            }
        }))
        .with_packages(vec![
            json!({
                "name": "pkg-a",
                "packageManager": "pnpm@7.0.0"
            }),
        ])
        .with_version_group(json!({
            "dependencies": ["pnpm"],
            "dependencyTypes": ["packageManager"],
            "pinned": "8.0.0"
        }))
        .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![
        ExpectedInstance {
            state: InstanceState::fixable(DiffersToPinnedVersion),
            dependency_name: "pnpm",
            id: "pnpm in /packageManager of pkg-a",
            actual: "7.0.0",
            expected: Some("8.0.0"),
            overridden: None,
        },
    ]);
}
```

### Testing Semver Ranges

```rust
#[test]
fn enforces_caret_range() {
    let ctx = TestBuilder::new()
        .with_packages(vec![
            json!({
                "name": "pkg-a",
                "dependencies": {
                    "react": "~18.0.0",  // Tilde range
                    "lodash": "^4.17.0"  // Caret range
                }
            }),
        ])
        .with_semver_group(json!({
            "dependencies": ["**"],
            "range": "^"
        }))
        .build_and_visit_packages();

    let react = ctx.instances.iter()
        .find(|i| i.dependency.name == "react")
        .unwrap();
    // React should be marked as needing fix (wrong range)

    let lodash = ctx.instances.iter()
        .find(|i| i.dependency.name == "lodash")
        .unwrap();
    // Lodash should be valid (correct range)
}
```

### Testing Highest/Lowest Semver

```rust
#[test]
fn uses_highest_semver_found() {
    let ctx = TestBuilder::new()
        .with_packages(vec![
            json!({"name": "pkg-a", "dependencies": {"lodash": "4.17.0"}}),
            json!({"name": "pkg-b", "dependencies": {"lodash": "4.17.21"}}),
            json!({"name": "pkg-c", "dependencies": {"lodash": "4.17.10"}}),
        ])
        .with_version_group(json!({
            "dependencies": ["lodash"],
            "policy": "highestSemver"
        }))
        .build_and_visit_packages();

    // All should use 4.17.21 (highest)
    let instances: Vec<_> = ctx.instances.iter()
        .filter(|i| i.dependency.name == "lodash")
        .collect();

    let pkg_b = instances.iter()
        .find(|i| i.package.name == "pkg-b")
        .unwrap();
    assert!(pkg_b.is_valid());  // Already at highest

    let pkg_a = instances.iter()
        .find(|i| i.package.name == "pkg-a")
        .unwrap();
    assert!(pkg_a.is_fixable());  // Needs update to 4.17.21
}
```

### Testing Multiple Version Groups

```rust
#[test]
fn applies_first_matching_version_group() {
    let ctx = TestBuilder::new()
        .with_packages(vec![
            json!({
                "name": "pkg-a",
                "dependencies": {
                    "react": "17.0.0",
                    "lodash": "4.17.0"
                }
            }),
        ])
        .with_version_groups(vec![
            json!({
                "dependencies": ["react"],
                "pinned": "18.0.0"
            }),
            json!({
                "dependencies": ["**"],  // Catches everything else
                "policy": "highestSemver"
            }),
        ])
        .build_and_visit_packages();

    // React matches first group (pinned)
    // Lodash matches second group (highestSemver)
}
```

## ExpectedInstance Fields

```rust
ExpectedInstance {
    // The state this instance should have
    state: InstanceState::fixable(IsBanned),

    // The dependency name (e.g., "react")
    dependency_name: "react",

    // Location string: "{dep} in {location} of {package}"
    id: "react in /dependencies of package-a",

    // Current version
    actual: "17.0.0",

    // Expected version (Some("") means remove, None means no expectation)
    expected: Some("18.0.0"),

    // If version was overridden by config
    overridden: None,
}
```

## Location String Format

The `id` field follows this pattern:

```
{dependency} in {location} of {package}
```

Examples:

```rust
"react in /dependencies of package-a"
"lodash in /devDependencies of package-b"
"pnpm in /packageManager of package-c"
"node in /engines/node of package-d"
"customConfig in /custom/config/version of package-e"
```

## Manual Assertions (Alternative)

Instead of `expect()`, you can make manual assertions:

```rust
#[test]
fn manual_assertion_example() {
    let ctx = TestBuilder::new()
        .with_package(json!({
            "name": "pkg-a",
            "dependencies": {"react": "17.0.0"}
        }))
        .build_and_visit_packages();

    // Find specific instance
    let react = ctx.instances.iter()
        .find(|i| i.dependency.name == "react")
        .expect("Should find react instance");

    // Make assertions
    assert_eq!(react.dependency.name, "react");
    assert_eq!(react.package.name, "pkg-a");
    assert!(react.is_valid());

    // Check specifier
    match &react.specifier {
        Specifier::BasicSemver(basic) => {
            assert_eq!(basic.raw, "17.0.0");
        }
        _ => panic!("Expected BasicSemver"),
    }
}
```

## Testing Edge Cases

### Empty Dependencies

```rust
#[test]
fn handles_package_with_no_dependencies() {
    let ctx = TestBuilder::new()
        .with_package(json!({
            "name": "pkg-a",
            "version": "1.0.0"
        }))
        .build_and_visit_packages();

    // Should only have the version instance
    assert_eq!(ctx.instances.len(), 1);
}
```

### Invalid JSON

```rust
#[test]
fn handles_malformed_version() {
    let ctx = TestBuilder::new()
        .with_package(json!({
            "name": "pkg-a",
            "dependencies": {"react": "not-a-version"}
        }))
        .build_and_visit_packages();

    let react = ctx.instances.iter()
        .find(|i| i.dependency.name == "react")
        .unwrap();

    // Should be marked as unsupported
    assert!(matches!(react.specifier, Specifier::Unsupported(_)));
}
```

### Git/File/URL Dependencies

```rust
#[test]
fn handles_git_dependencies() {
    let ctx = TestBuilder::new()
        .with_package(json!({
            "name": "pkg-a",
            "dependencies": {
                "my-pkg": "git://github.com/user/repo.git"
            }
        }))
        .build_and_visit_packages();

    let git_dep = ctx.instances.iter()
        .find(|i| i.dependency.name == "my-pkg")
        .unwrap();

    assert!(matches!(git_dep.specifier, Specifier::Git(_)));
}
```

## Running Tests

```bash
# Run all tests
just test
cargo test

# Run specific test
cargo test test_name

# Run tests in a file (matches pattern)
cargo test banned_test

# Run with output
cargo test test_name -- --nocapture

# Run and show ignored
cargo test -- --ignored

# Watch mode
just watch
```

## Test Organization

- **Integration tests:** `src/visit_packages/*_test.rs` (preferred)
- **Unit tests:** Co-located as `*_test.rs`
- **Test utilities:** `src/test/builder.rs`, `src/test/expect.rs`, `src/test/mock.rs`

## Good Test Examples

Study these files for patterns:

- `src/visit_packages/banned_test.rs` - Comprehensive examples
- `src/visit_packages/pinned_test.rs` - Version group testing
- `src/visit_packages/local_test.rs` - Local package handling

## Common Mistakes

❌ **Don't** manually construct Context

```rust
let ctx = Context { /* ... */ };  // Wrong!
```

✅ **Do** use TestBuilder

```rust
let ctx = TestBuilder::new().build();
```

❌ **Don't** forget to call build_and_visit_packages()

```rust
let ctx = TestBuilder::new()
    .with_package(...)
    .build();  // States won't be assigned!
```

✅ **Do** call build_and_visit_packages() when testing validation

```rust
let ctx = TestBuilder::new()
    .with_package(...)
    .build_and_visit_packages();  // States assigned
```

❌ **Don't** hardcode instance indices

```rust
let instance = ctx.instances[0];  // Fragile!
```

✅ **Do** find instances by criteria

```rust
let instance = ctx.instances.iter()
    .find(|i| i.dependency.name == "react")
    .unwrap();
```

## Troubleshooting

**Problem:** Test can't find instance
→ Check package name spelling and location path format

**Problem:** State is Unknown instead of expected
→ Make sure you called `build_and_visit_packages()`, not just `build()`

**Problem:** Wrong number of instances
→ Remember that package versions create instances too ("pkg-a in /version of pkg-a")

**Problem:** Expected state doesn't match
→ Check if another validation ran first and set a different state

## Tips

1. **Name tests descriptively** - Test name should explain what scenario is being tested
2. **One concept per test** - Don't test multiple unrelated things in one test
3. **Use real JSON structures** - Mirrors real-world usage
4. **Check all instances** - Don't forget about version instances for local packages
5. **Test edge cases** - Empty deps, malformed versions, etc.
6. **Use expect() for complex assertions** - Makes tests more readable
7. **Study existing tests** - Best source of patterns
