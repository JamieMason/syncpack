# Link Validation Improvement Plan

## Summary

This document outlines a plan to improve the incomplete link (`link:../path`) support introduced in git commit `4b403825` (PR #293). The current implementation assumes all links are valid. We need proper validation and dedicated instance states for links.

## Current Implementation Analysis

### What Was Added (Commit 4b403825)

1. **Specifier Support**: Added `Specifier::Link(Raw)` variant for parsing `link:` syntax
2. **Basic Detection**: Added `is_link()` method to identify link specifiers
3. **Validation Logic**: In `visit_packages/preferred_semver.rs`, links are automatically marked as `ValidInstance::SatisfiesLocal`
4. **Tests**: Basic test coverage for parsing and one integration test

### Issues with Current Implementation

1. **No Path Validation**: No verification that the link path points to the correct local package
2. **No Target Validation**: No checking that the linked package matches the expected dependency name
3. **Missing Error Cases**: No handling of broken, invalid, or mismatched links

## Implementation Plan

Following a documentation-first, test-driven development approach:

## Implementation Steps

Following a documentation-first, test-driven development approach:

### Step 1: Documentation

Create status documentation for the new instance state and update existing documentation.

### Step 2: Tests

Write comprehensive tests for both unit and integration scenarios before implementing the functionality.

### Step 3: Implementation

Implement the Link specifier, instance states, and validation logic to make the tests pass.

### Step 1: Documentation

Create status documentation for the new instance state and update existing documentation.

#### Create Status Documentation

Create `site/src/content/docs/status/invalid-link.mdx`:

```mdx
---
title: InvalidLink
status: fixable
---

## When this happens

- ✓ Instance uses link syntax (`link:../path`)
- ✘ Link is invalid due to one of:
  - No local package exists with the expected dependency name
  - Link path cannot be resolved to an absolute path
  - Link path does not point to the same directory as the local package
```

Create `site/src/content/docs/status/link-to-non-local-package.mdx`:

```mdx
---
title: LinkToNonLocalPackage
status: unfixable
---

## When this happens

- ✓ Instance uses link syntax (`link:../path`)
- ✘ Link points to a package that has no local instance in the workspace
- ? Cannot be automatically fixed since there's no local version to link to
```

#### Verify Existing Documentation

Confirm that `site/src/content/docs/reference/specifier-types.mdx` properly documents the "link" specifier type (✅ already complete).

### Step 2: Unit Tests and Basic Implementation

#### Write Unit Tests for Link Specifier

Create tests in `src/specifier/link.rs` (as part of the module):

```rust
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn link_parsing() {
    let cases: Vec<&str> = vec![
      "link:../foo",
      "link:../package-a",
      "link:path/to/foo",
      "link:./relative/path",
      "link:/absolute/path",
      "link:../../../deeply/nested/path",
    ];

    for value in cases {
      match Specifier::new(value, None) {
        Specifier::Link(actual) => {
          assert_eq!(actual.raw, value);
          // Verify path extraction
          let expected_path = value.strip_prefix("link:").unwrap();
          assert_eq!(actual.path, expected_path);
        }
        _ => panic!("Expected Link for value: {}", value),
      }
    }
  }

  #[test]
  fn link_construction() {
    let raw = "link:../package-a";
    let link = Link::new(raw.to_string());

    assert_eq!(link.raw, "link:../package-a");
    assert_eq!(link.path, "../package-a");
  }

  #[test]
  fn link_with_range_no_op() {
    let link = Link::new("link:../package-a".to_string());
    let result = link.clone().with_range(&SemverRange::Minor);

    // Links don't support range transformations
    assert_eq!(result, link);
  }

  #[test]
  fn link_with_semver_no_op() {
    let link = Link::new("link:../package-a".to_string());
    let semver = BasicSemver::new("1.2.3").unwrap();
    let result = link.clone().with_semver(&semver);

    // Links don't support semver transformations
    assert_eq!(result, link);
  }
}
```

#### Create Helper Function for Link Path Extraction

No new specifier struct needed - keep `Specifier::Link(Raw)` and add a helper function in `src/specifier.rs`:

```rust
impl Specifier {
  // ... existing methods ...

  /// Extract the path portion from a link specifier
  pub fn get_link_path(&self) -> Option<&str> {
    if let Specifier::Link(raw) = self {
      raw.raw.strip_prefix("link:")
    } else {
      None
    }
  }
}
```

#### Unit Tests for Link Path Extraction

Add test to `src/specifier_test.rs` to verify path extraction:

```rust
#[test]
fn link_path_extraction() {
  let cases = vec![
    ("link:../foo", Some("../foo")),
    ("link:path/to/foo", Some("path/to/foo")),
    ("link:./relative/path", Some("./relative/path")),
    ("link:/absolute/path", Some("/absolute/path")),
    ("not-a-link", None),
  ];

  for (input, expected) in cases {
    let spec = Specifier::new(input, None);
    assert_eq!(spec.get_link_path(), expected);
  }
}
```

### Step 3: Integration Tests and Full Implementation

#### Update Test Mock for Path Resolution

Update `src/test/mock.rs` to set consistent file paths in `package_json_from_value`:

```rust
use std::env;

// Extract package name from the JSON value and set consistent path
let package_name = contents.get("name")
  .and_then(|name| name.as_str())
  .unwrap_or("unnamed");

// Use cross-platform path construction
let base = env::current_dir().unwrap().join("fake-repo-root");
let package_dir = base.join(package_name);
file_path: package_dir.join("package.json")
```

#### Write Integration Tests

Add new `mod links` to `visit_packages/preferred_semver_test.rs`:

```rust
mod links {
  use super::*;
  use crate::instance_state::{FixableInstance, UnfixableInstance, ValidInstance};

  #[test]
  fn valid_link_to_local_package() {
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({
          "name": "package-a",
          "version": "1.0.0"
        }),
        json!({
          "name": "package-b",
          "version": "2.0.0",
          "dependencies": {
            "package-a": "link:../package-a"
          }
        }),
      ])
      .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(ValidInstance::IsLocalAndValid),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "2.0.0",
        expected: Some("2.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(ValidInstance::IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(ValidInstance::SatisfiesLocal),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "link:../package-a",
        expected: Some("link:../package-a"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn invalid_link_to_local_package() {
    // Test link that points to wrong directory but target package exists locally
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({
          "name": "package-a",
          "version": "1.0.0"
        }),
        json!({
          "name": "package-b",
          "version": "2.0.0",
          "dependencies": {
            "package-a": "link:../wrong-path"
          }
        }),
      ])
      .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(ValidInstance::IsLocalAndValid),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "2.0.0",
        expected: Some("2.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(ValidInstance::IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::fixable(FixableInstance::InvalidLink),
        dependency_name: "package-a",
        id: "package-a in /dependencies of package-b",
        actual: "link:../wrong-path",
        expected: Some("link:../package-a"),
        overridden: None,
      },
    ]);
  }

  #[test]
  fn link_to_non_local_package() {
    // Test link that points to a package that has no local instance
    let ctx = TestBuilder::new()
      .with_packages(vec![
        json!({
          "name": "package-a",
          "version": "1.0.0"
        }),
        json!({
          "name": "package-b",
          "version": "2.0.0",
          "dependencies": {
            "external-package": "link:../external-package"
          }
        }),
      ])
      .build_and_visit_packages();

    expect(&ctx).to_have_instances(vec![
      ExpectedInstance {
        state: InstanceState::valid(ValidInstance::IsLocalAndValid),
        dependency_name: "package-b",
        id: "package-b in /version of package-b",
        actual: "2.0.0",
        expected: Some("2.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::valid(ValidInstance::IsLocalAndValid),
        dependency_name: "package-a",
        id: "package-a in /version of package-a",
        actual: "1.0.0",
        expected: Some("1.0.0"),
        overridden: None,
      },
      ExpectedInstance {
        state: InstanceState::unfixable(UnfixableInstance::LinkToNonLocalPackage),
        dependency_name: "external-package",
        id: "external-package in /dependencies of package-b",
        actual: "link:../external-package",
        expected: None,
        overridden: None,
      },
    ]);
  }
}
```

#### Add New Instance State

Update `src/instance_state.rs`:

```rust
pub enum FixableInstance {
  // ... existing variants ...

  /// - ✘ Instance uses link syntax but link is invalid
  /// - Could be: path not found, no package.json, wrong package name, or not a local package
  InvalidLink,
}

// Add to UnfixableInstance enum:
pub enum UnfixableInstance {
  // ... existing variants ...

  /// - ✘ Instance uses link syntax but points to a package that has no local instance
  /// - ? We cannot determine the correct path since there's no local version
  LinkToNonLocalPackage,
}
```

#### Add Validation Method to Instance

Add `has_valid_link()` method to `src/instance.rs` with necessary imports:

```rust
use crate::{context::Context, specifier::Specifier};
use std::path::{Path, PathBuf};

impl Instance {
  // ... existing methods ...

  /// Validates that a link specifier points to the correct local package in the workspace
  pub fn has_valid_link(&self, ctx: &Context, expected_name: &str) -> bool {
    if let Some(link_path) = self.descriptor.specifier.get_link_path() {
      // Find the local package with the expected dependency name
      let local_package = ctx.packages.all.iter()
        .find(|package| package.borrow().name == expected_name);

      let local_package = match local_package {
        Some(package) => package,
        None => return false, // No local package with this name exists
      };

      // Get the directory containing the consuming package.json
      let package_dir = match self.descriptor.package.borrow().file_path.parent() {
        Some(dir) => dir,
        None => return false, // No parent directory for consuming package
      };

      // Resolve the link path relative to the consuming package directory (no disk I/O)
      let resolved_path = package_dir.join(link_path);

      // Get the directory of the local package (file_path points to package.json, we need its parent)
      let local_package_dir = match local_package.borrow().file_path.parent() {
        Some(dir) => dir,
        None => return false, // No parent directory for local package
      };

      // Compare absolute paths (no need for cwd normalization if file_path is already absolute)
      let normalized_resolved = if resolved_path.is_absolute() {
        resolved_path
      } else {
        package_dir.join(link_path)
      };

      // Compare normalized paths
      normalized_resolved == local_package_dir
    } else {
      false // Not a link specifier
    }
  }

  /// Calculate the correct link path to a local package for error correction
  pub fn get_expected_link_path(&self, ctx: &Context, expected_name: &str) -> Option<String> {
    // Find the local package
    let local_package = ctx.packages.all.iter()
      .find(|package| package.borrow().name == expected_name)?;

    // Get directories (both consuming and target packages)
    let package_dir = self.descriptor.package.borrow().file_path.parent()?;
    let local_package_dir = local_package.borrow().file_path.parent()?;

    // Calculate relative path from consuming package to local package
    let relative_path = calculate_relative_path(package_dir, local_package_dir)?;
    Some(format!("link:{}", relative_path))
  }
}

/// Calculate relative path from source to target directory
fn calculate_relative_path(from: &Path, to: &Path) -> Option<String> {
  // Simple relative path calculation for common workspace structures
  // e.g., from /fake-repo-root/package-b to /fake-repo-root/package-a = "../package-a"
  let from_components: Vec<_> = from.components().collect();
  let to_components: Vec<_> = to.components().collect();

  // Find common prefix
  let mut common_len = 0;
  for (a, b) in from_components.iter().zip(to_components.iter()) {
    if a == b {
      common_len += 1;
    } else {
      break;
    }
  }

  // Build relative path with forward slashes (link: syntax uses forward slashes)
  let mut result = String::new();

  // Go up from source to common ancestor
  let up_levels = from_components.len() - common_len;
  for _ in 0..up_levels {
    if !result.is_empty() {
      result.push('/');
    }
    result.push_str("..");
  }

  // Go down to target
  for component in &to_components[common_len..] {
    if !result.is_empty() {
      result.push('/');
    }
    result.push_str(&component.as_os_str().to_string_lossy());
  }

  if result.is_empty() { Some(".".to_string()) } else { Some(result) }
}
```

#### Update Validation Logic

Update `src/visit_packages/preferred_semver.rs`:

```rust
// Replace current link handling
if instance.descriptor.specifier.is_link() {
  debug!("{L5}it is using the link specifier");

  if instance.has_valid_link(ctx, &dependency.internal_name) {
    debug!("{L6}it is valid");
    instance.mark_valid(ValidInstance::SatisfiesLocal, &instance.descriptor.specifier);
  } else {
    // Check if there is a local package for this dependency
    let has_local_package = ctx.packages.all.iter()
      .any(|package| package.borrow().name == dependency.internal_name);

    if has_local_package {
      debug!("{L6}it is invalid");
      debug!("{L7}mark as error");
      // Calculate the expected correct link path
      if let Some(expected_link) = instance.get_expected_link_path(ctx, &dependency.internal_name) {
        let expected_specifier = Specifier::new(&expected_link, None);
        instance.mark_fixable(FixableInstance::InvalidLink, &expected_specifier);
      } else {
        // Fallback to local version if path calculation fails
        let local_specifier = dependency.get_local_specifier().unwrap();
        instance.mark_fixable(FixableInstance::InvalidLink, &local_specifier);
      }
    } else {
      debug!("{L6}it depends on a package with no local instance");
      debug!("{L7}mark as error");
      instance.mark_unfixable(UnfixableInstance::LinkToNonLocalPackage);
    }
  }
  return;
}
```

### Additional Considerations

1. **Symlink Resolution**: Handle cases where links point to symlinked directories
2. **Absolute vs Relative Paths**: Support both `link:../package` and `link:/absolute/path`
3. **Cross-Platform Path Handling**: Ensure Windows/Unix path compatibility
4. **Circular Dependencies**: Detect and handle circular link references
5. **Nested Workspaces**: Handle links across nested workspace boundaries

## Success Criteria

1. **Documentation**: Clear documentation for the new InvalidLink status
2. **Tests Pass**: All tests pass, demonstrating correct validation behavior
3. **Proper Validation**: Links are validated against their target packages
4. **Clear Error Messages**: Invalid links provide actionable error messages
5. **Backwards Compatibility**: Existing valid link configurations continue to work

## Breaking Changes

This improvement will introduce breaking changes in behavior:

1. **Previously Valid Links**: Some links that were previously marked as valid may now be flagged as invalid if they don't meet validation criteria
2. **State Consistency**: Valid links will continue to use `ValidInstance::SatisfiesLocal` (no change from current behavior)
3. **Error Detection**: New error cases will be detected that were previously ignored

## Note on CLI Support

The CLI already properly supports the "link" specifier type in `--specifier-types` filtering (this was included in the original PR #293). No additional CLI changes are needed.

This plan addresses the incomplete link implementation by following a structured documentation-first, test-driven approach that ensures proper validation, dedicated instance states, and comprehensive error handling while maintaining the existing API structure.
