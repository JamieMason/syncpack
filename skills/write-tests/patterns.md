# Example: Writing Tests with TestBuilder

<purpose>
This guide shows you how to write effective tests for Syncpack using the TestBuilder pattern.
</purpose>

<testbuilder_pattern>

## The TestBuilder Pattern

**Always use TestBuilder** - Never manually construct Context in tests. TestBuilder provides a fluent API that handles all the complexity of setting up test scenarios.

</testbuilder_pattern>

<basic_structure>

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

</basic_structure>

<testbuilder_methods>

## TestBuilder Methods

<adding_packages>

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

</adding_packages>

<adding_version_groups>

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

</adding_version_groups>

<adding_semver_groups>

### Adding Semver Groups

```rust
.with_semver_group(json!({
    "dependencies": ["**"],
    "range": "^"
}))
```

</adding_semver_groups>

<custom_config>

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

</custom_config>

<strict_mode>

### Strict Mode

```rust
.with_strict(true)  // Treat Suspect states as errors
```

</strict_mode>

<registry_updates>

### Registry Updates

```rust
.with_registry_updates(json!({
    "react": ["17.0.0", "17.0.1", "18.0.0", "18.2.0"],
    "lodash": ["4.17.0", "4.17.21"]
}))
```

</registry_updates>

<update_target>

### Update Target

```rust
use crate::cli::UpdateTarget;

.with_update_target(UpdateTarget::Latest)
.with_update_target(UpdateTarget::Minor)
.with_update_target(UpdateTarget::Patch)
```

</update_target>

<build_methods>

### Build Methods

```rust
// Build Context without visiting (rare)
.build()

// Build Context and run visit_packages (most common)
.build_and_visit_packages()
```

</build_methods>

</testbuilder_methods>

<common_test_patterns>

## Common Test Patterns

<banned_dependencies>

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

</banned_dependencies>

<version_mismatches>

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

</version_mismatches>

<local_packages>

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

</local_packages>

<workspace_protocol>

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

</workspace_protocol>

<custom_types>

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

</custom_types>

<semver_ranges>

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

</semver_ranges>

<highest_lowest_semver>

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

</highest_lowest_semver>

<multiple_version_groups>

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

</multiple_version_groups>

</common_test_patterns>

<expected_instance_fields>

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

</expected_instance_fields>

<location_strings>

## Location String Format

The `id` field follows this pattern:

```
{dependency} in {location} of {package}
```

<examples>

Examples:

```rust
"react in /dependencies of package-a"
"lodash in /devDependencies of package-b"
"pnpm in /packageManager of package-c"
"node in /engines/node of package-d"
"customConfig in /custom/config/version of package-e"
```

</examples>

</location_strings>

<manual_assertions>

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

</manual_assertions>

<edge_cases>

## Testing Edge Cases

<empty_dependencies>

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

</empty_dependencies>

<invalid_json>

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

</invalid_json>

<git_file_url>

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

</git_file_url>

</edge_cases>

<running_tests>

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

</running_tests>

<test_organization>

## Test Organization

- **Integration tests:** `src/visit_packages/*_test.rs` (preferred)
- **Unit tests:** Co-located as `*_test.rs`
- **Test utilities:** `src/test/builder.rs`, `src/test/expect.rs`, `src/test/mock.rs`

</test_organization>

<good_examples>

## Good Test Examples

Study these files for patterns:

- `src/visit_packages/banned_test.rs` - Comprehensive examples
- `src/visit_packages/pinned_test.rs` - Version group testing
- `src/visit_packages/local_test.rs` - Local package handling

</good_examples>

<common_mistakes>

## Common Mistakes

<wrong_patterns>

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

</wrong_patterns>

</common_mistakes>

<troubleshooting>

## Troubleshooting

<problem>
**Problem:** Test can't find instance
→ Check package name spelling and location path format
</problem>

<problem>
**Problem:** State is Unknown instead of expected
→ Make sure you called `build_and_visit_packages()`, not just `build()`
</problem>

<problem>
**Problem:** Wrong number of instances
→ Remember that package versions create instances too ("pkg-a in /version of pkg-a")
</problem>

<problem>
**Problem:** Expected state doesn't match
→ Check if another validation ran first and set a different state
</problem>

</troubleshooting>

<tips>

## Tips

1. **Name tests descriptively** - Test name should explain what scenario is being tested
2. **One concept per test** - Don't test multiple unrelated things in one test
3. **Use real JSON structures** - Mirrors real-world usage
4. **Check all instances** - Don't forget about version instances for local packages
5. **Test edge cases** - Empty deps, malformed versions, etc.
6. **Use expect() for complex assertions** - Makes tests more readable
7. **Study existing tests** - Best source of patterns

</tips>
