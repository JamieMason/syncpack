# Specifier2 Refactor Plan

## Overview

`Specifier2` is a new, optimized replacement for the existing `Specifier` that addresses key architectural limitations. The main goal is to create a **stateless, cacheable, and performant** specifier system.

## Problems with Current Specifier

### 1. **Not Cacheable**

- `Specifier::new(value, local_version)` creates different instances for the same `value` with different `local_version`
- Cannot cache effectively due to workspace protocol state dependency
- Performance penalty: 2.8x slower cache misses due to repeated parsing

### 2. **Stateful Design**

- `WorkspaceProtocol` struct stores both `local_version` and resolved `semver`
- Resolution happens at creation time and is stored in the struct
- Makes sharing instances across different contexts impossible

### 3. **Follows Single-Threaded Architecture**

- Current `Specifier` already uses `Rc<Specifier>` following the project's single-threaded design
- However, caching issues remain due to stateful workspace protocol design
- The refactor improves on the existing `Rc` pattern with better caching

## Specifier2 Design Goals

### 1. **Stateless and Cacheable**

- Store only the raw specifier string (e.g., `"workspace:^"`)
- No workspace resolution logic in `Specifier2` - handled externally by callers
- Enable aggressive caching: same input string = same cached instance

### 2. **Optimized Single-Threaded Design**

- Continue using `Rc<Specifier2>` (consistent with project patterns)
- Use `RefCell` + `thread_local!` for optimal single-threaded caching
- Follow established project rule: "Use Rc for single-threaded sharing"

### 3. **Pure Methods**

- All methods work with the stored specifier string directly
- No context-dependent behavior or external dependencies
- Workspace protocols handled by external resolution before method calls

## Architecture Comparison

### Current Specifier (Stateful)

```rust
// Creation time resolution
Specifier::new("workspace:^", Some(local_version))
→ WorkspaceProtocol {
    raw: "workspace:^",
    local_version: BasicSemver("1.2.3"),
    semver: BasicSemver("^1.2.3")  // ← Resolved at creation
  }

// Method calls use stored semver
specifier.get_node_version() // ← Uses pre-resolved semver
```

### New Specifier2 (Stateless)

```rust
// No resolution at creation - just stores the raw string
Specifier2::new("workspace:^")
→ WorkspaceProtocol("workspace:^")  // ← Raw string only

// External resolution before method calls
resolved_specifier = resolve_workspace_specifier(specifier, local_version)
resolved_specifier.get_node_version() // ← Works with resolved specifier
```

## Implementation Plan

### Phase 1: Core Architecture

#### 1.1 Update Enum Storage

```rust
#[derive(Debug, PartialEq)]
pub enum Specifier2 {
  WorkspaceProtocol(String), // ← Raw string only: "workspace:^"
  Exact(String),             // "1.2.3"
  Range(String),             // "^1.2.3"
  // ... other variants unchanged
}
```

#### 1.2 Creation Method (Already Implemented)

```rust
impl Specifier2 {
  pub fn new(value: &str) -> Rc<Self> {
    // Cache key is only the raw string value
    // This enables perfect sharing across all contexts
    SPECIFIER_CACHE.with(|cache| {
      let mut cache = cache.borrow_mut();
      match cache.get(value) {
        Some(rc) => rc.clone(),
        None => {
          let rc = Rc::new(Self::create(value));
          cache.insert(value.to_string(), rc.clone());
          rc
        }
      }
    })
  }
}
```

### Phase 2: Method Signature Analysis

#### 2.1 Method Categories Analysis

All methods work with the stored specifier string directly. Workspace protocol resolution is handled externally.

```rust
// Target method signatures (all pure, no external dependencies):
pub fn get_semver(&self) -> Result<BasicSemver, String>
pub fn get_node_version(&self) -> Result<Version, String>
pub fn get_node_range(&self) -> Result<Range, String>
pub fn get_semver_range(&self) -> Result<SemverRange, String>
pub fn with_range(&self, range: &SemverRange) -> Option<String>  // ✅ Already implemented
pub fn with_semver(&self, semver: &BasicSemver) -> Result<String, String>
pub fn get_raw(&self) -> &str
pub fn get_config_identifier(&self) -> &'static str  // ✅ Already implemented
pub fn has_semver_range_of(&self, range: &SemverRange) -> bool
pub fn has_same_version_number_as(&self, other: &Self) -> Result<bool, String>
pub fn satisfies_all(&self, ranges: &[Range]) -> Result<bool, String>
pub fn satisfies(&self, range: &Range) -> Result<bool, String>
pub fn is_workspace_protocol(&self) -> bool
pub fn has_same_release_channel_as(&self, other: &Self) -> Result<bool, String>
pub fn is_eligible_update_for(&self, target: &Self) -> Result<bool, String>
pub fn is_older_than(&self, other: &Self) -> Result<bool, String>
pub fn is_older_than_by_minor(&self, other: &Self) -> Result<bool, String>
pub fn is_older_than_by_patch(&self, other: &Self) -> Result<bool, String>
```

#### 2.2 Method Categories

All methods are pure and work with the stored specifier string. Workspace protocol resolution happens externally before calling these methods.

**Category A: Simple Operations**

```rust
pub fn get_raw(&self) -> &str                           // Return stored string
pub fn is_workspace_protocol(&self) -> bool             // Check string prefix
pub fn get_config_identifier(&self) -> &'static str     // ✅ Already implemented
pub fn has_semver_range_of(&self, range: &SemverRange) -> bool // String comparison
pub fn with_range(&self, range: &SemverRange) -> Option<String>  // ✅ Already implemented
```

**Category B: Version Resolution Methods**
These parse version information from the stored string:

```rust
pub fn get_semver(&self) -> Result<BasicSemver, String>
pub fn get_node_version(&self) -> Result<Version, String>
pub fn get_node_range(&self) -> Result<Range, String>
pub fn get_semver_range(&self) -> Result<SemverRange, String>
pub fn with_semver(&self, semver: &BasicSemver) -> Result<String, String>
```

**Category C: Comparison Methods**
These delegate to Category B methods:

```rust
pub fn has_same_version_number_as(&self, other: &Self) -> Result<bool, String>
pub fn satisfies(&self, range: &Range) -> Result<bool, String>
pub fn satisfies_all(&self, ranges: &[Range]) -> Result<bool, String>
pub fn has_same_release_channel_as(&self, other: &Self) -> Result<bool, String>
pub fn is_eligible_update_for(&self, target: &Self) -> Result<bool, String>
pub fn is_older_than(&self, other: &Self) -> Result<bool, String>
pub fn is_older_than_by_minor(&self, other: &Self) -> Result<bool, String>
pub fn is_older_than_by_patch(&self, other: &Self) -> Result<bool, String>
```

**Note**: `BasicSemver` could potentially be replaced by `Specifier2` entirely, with lazy caching of `node_range` and `node_version` properties.

#### 2.3 Return Type Decisions

**Owned vs Borrowed Returns:**

- `get_raw()`: Return `&str` (borrow from stored string)
- `get_semver()`: Return `Option<BasicSemver>` (owned, since we construct it)
- `get_node_version()`: Return `Option<Version>` (owned, since we construct it)

**Error Handling:**

- Methods return `Result<T, String>` for operations that can fail
- Error messages include context about what failed and why
- Workspace protocols return errors when methods can't extract version info from raw string

### Phase 3: Method Implementation

#### 3.1 Implementation Priority Order

**Phase 3A: Simple Methods**

```rust
pub fn get_raw(&self) -> &str
pub fn is_workspace_protocol(&self) -> bool
pub fn has_semver_range_of(&self, range: &SemverRange) -> bool
```

**Phase 3B: Version Resolution Methods**
These parse version information from the stored string:

```rust
pub fn get_semver(&self) -> Result<BasicSemver, String>
pub fn get_node_version(&self) -> Result<Version, String>
pub fn get_node_range(&self) -> Result<Range, String>
pub fn get_semver_range(&self) -> Result<SemverRange, String>
pub fn with_semver(&self, semver: &BasicSemver) -> Result<String, String>
```

**Phase 3C: Comparison Methods**
These delegate to Phase 3B methods:

```rust
pub fn has_same_version_number_as(&self, other: &Self) -> Result<bool, String>
pub fn satisfies(&self, range: &Range) -> Result<bool, String>
pub fn satisfies_all(&self, ranges: &[Range]) -> Result<bool, String>
pub fn has_same_release_channel_as(&self, other: &Self) -> Result<bool, String>
pub fn is_eligible_update_for(&self, target: &Self) -> Result<bool, String>
pub fn is_older_than(&self, other: &Self) -> Result<bool, String>
pub fn is_older_than_by_minor(&self, other: &Self) -> Result<bool, String>
pub fn is_older_than_by_patch(&self, other: &Self) -> Result<bool, String>
```

#### 3.2 Error Handling for Workspace Protocols

```rust
impl Specifier2 {
  // Workspace protocols return errors when version info can't be extracted
  pub fn get_semver(&self) -> Result<BasicSemver, String> {
    match self {
      Self::WorkspaceProtocol(raw) => {
        Err(format!("Cannot extract semver from unresolved workspace protocol: '{}'", raw))
      }
      Self::Exact(version) => BasicSemver::new(version)
        .ok_or_else(|| format!("Invalid semver: '{}'", version)),
      // ... other variants
    }
  }
}
```

#### 3.3 Final Method Signatures (Pure and Stateless)

**Simple methods:**

```rust
pub fn get_raw(&self) -> &str                           // ✅ Already implemented
pub fn is_workspace_protocol(&self) -> bool             // Check string prefix
pub fn get_config_identifier(&self) -> &'static str     // ✅ Already implemented
pub fn get_alias_name(&self) -> Option<&str>            // ✅ Already implemented
pub fn get_semver_number(&self) -> Option<&str>         // ✅ Already implemented
pub fn with_range(&self, range: &SemverRange) -> Option<String>  // ✅ Already implemented
pub fn has_semver_range_of(&self, range: &SemverRange) -> bool   // String comparison only
```

**Version resolution methods:**
These parse version information from the stored string:

```rust
pub fn get_semver(&self) -> Result<BasicSemver, String>
pub fn get_node_version(&self) -> Result<Version, String>
pub fn get_node_range(&self) -> Result<Range, String>
pub fn get_semver_range(&self) -> Result<SemverRange, String>
pub fn with_semver(&self, semver: &BasicSemver) -> Result<String, String>
```

**Comparison methods:**
These delegate to version resolution methods above:

```rust
pub fn has_same_version_number_as(&self, other: &Self) -> Result<bool, String>
pub fn satisfies(&self, range: &Range) -> Result<bool, String>
pub fn satisfies_all(&self, ranges: &[Range]) -> Result<bool, String>
pub fn has_same_release_channel_as(&self, other: &Self) -> Result<bool, String>
pub fn is_eligible_update_for(&self, target: &Self) -> Result<bool, String>
pub fn is_older_than(&self, other: &Self) -> Result<bool, String>
pub fn is_older_than_by_minor(&self, other: &Self) -> Result<bool, String>
pub fn is_older_than_by_patch(&self, other: &Self) -> Result<bool, String>
```

### Phase 4: Advanced Features

#### 4.1 Traits Implementation

```rust
impl Ord for Specifier2 {
  fn cmp(&self, other: &Self) -> Ordering {
    // Implementation depends on get_semver() and version comparison logic
  }
}

impl PartialOrd for Specifier2 {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
```

#### 4.2 Advanced Workspace Resolution

Move logic from current `Specifier::from_workspace_protocol` to helper methods:

```rust
impl Specifier2 {
  fn resolve_workspace_protocol(&self, local_version: &BasicSemver) -> BasicSemver {
    match self {
      Self::WorkspaceProtocol(raw) => {
        let without_protocol = &raw[10..]; // Skip "workspace:"
        let sanitised = sanitise_value(without_protocol);
        let sanitised_str = sanitised.as_deref().unwrap_or(without_protocol);

        if parser::is_simple_semver(sanitised_str) {
          BasicSemver::new(sanitised_str).unwrap()
        } else if sanitised_str == "~" || sanitised_str == "^" {
          let combined = format!("{}{}", sanitised_str, local_version.raw);
          BasicSemver::new(&combined).unwrap()
        } else {
          // Handle other cases...
        }
      }
      _ => panic!("resolve_workspace_protocol called on non-workspace specifier")
    }
  }
}
```

#### 4.3 Pure Method Implementation

```rust
pub fn get_semver(&self) -> Result<BasicSemver, String> {
  match self {
    Self::WorkspaceProtocol(raw) => {
      Err(format!("Cannot extract semver from unresolved workspace protocol: '{}'", raw))
    }
    Self::Exact(version) => BasicSemver::new(version)
      .ok_or_else(|| format!("Invalid semver: '{}'", version)),
    Self::Range(raw) => BasicSemver::new(raw)
      .ok_or_else(|| format!("Invalid range semver: '{}'", raw)),
    // ... other variants
    _ => Err("No semver available for this specifier type".to_string()),
  }
}
```

### Phase 5: Performance Optimizations

#### 5.1 Single-Threaded Cache (Already Implemented)

```rust
thread_local! {
  static SPECIFIER_CACHE: RefCell<HashMap<String, Rc<Specifier2>>> = RefCell::new(HashMap::new());
}
```

## Performance Benefits

### Real-World Impact

Based on fluid-framework project analysis:

- **152x** `typescript ~5.4.5`
- **153x** `prettier ~3.0.3`
- **115x** `@fluid-tools/build-cli ^0.49.0`

### Expected Performance Gains

- **31x faster** cache hits (measured)
- **Improved single-threaded performance** with `thread_local!` caching
- **Lazy workspace resolution** - only when needed
- **Perfect caching** - same string = same instance regardless of context

## Migration Strategy

### Phase 1: Parallel Development

- Complete `Specifier2` implementation in isolation
- Add comprehensive tests matching `specifier_test.rs`
- Validate performance with real-world data

### Phase 2: Integration Planning

- Design integration strategy for replacing `Specifier` with `Specifier2`
- Plan migration of workspace-dependent operations to use new method signatures
- Consider whether `BasicSemver` should be replaced entirely by `Specifier2`

### Phase 3: Replacement

- Replace `Specifier` with `Specifier2` throughout codebase
- Remove old `Specifier` and related types
- Clean up workspace protocol handling

### Error Handling Strategy

### Workspace Protocol Handling

```rust
// Workspace protocols return descriptive errors:
workspace_spec.get_semver()
// ↓
// Err("Cannot extract semver from unresolved workspace protocol: 'workspace:^'")

// Resolution happens externally before calling methods:
resolved_spec = resolve_workspace_protocol(workspace_spec, local_version);
resolved_spec.get_semver()  // ✅ Works with resolved specifier
```

### Validation

- All methods are pure and work with stored strings
- Workspace protocols return clear error messages when version info is needed
- External callers handle workspace resolution before calling methods

## Testing Strategy

### Unit Tests

- Port all tests from `specifier_test.rs` to `specifier2_test.rs`
- Add tests for new method signatures with `local_version` parameters
- Test error handling for workspace protocols without `local_version`
- Test workspace resolution accuracy with various patterns
- Performance tests comparing cache hit/miss ratios
- Memory usage validation for cached instances

## Current Implementation Status

### Key Architectural Decisions Made

1. **Single argument constructor**: `new(value: &str)` - no `local_version` parameter
2. **Perfect caching**: Cache key is only the raw string, enabling maximum reuse
3. **Raw string storage**: All variants store the original string unchanged
4. **Call-time resolution**: Workspace resolution happens in methods that need it
5. **Pure methods**: No external dependencies or context-dependent behavior
6. **Follows project patterns**: Uses `Rc<Specifier2>` and single-threaded design

## Current Status

### ✅ Completed

- Basic enum structure and parsing logic in `create()`
- Cache infrastructure with `Rc<Specifier2>` and `thread_local!`
- Core methods: `get_alias_name`, `get_semver_number`, `with_range`, `get_config_identifier`
- Single-threaded optimization with `RefCell` + `thread_local!`
- Workspace protocol storage (raw string only)
- Comprehensive parsing tests in `specifier2_test.rs`

### 🚧 Next Steps

- Add simple methods (Phase 3A): `get_raw()`, `is_workspace_protocol()`, `has_semver_range_of()`
- Add version resolution methods (Phase 3B) that parse from stored strings
- Add comparison methods (Phase 3C) that delegate to resolution methods
- Implement error handling for workspace protocols (return descriptive errors)
- Consider replacing `BasicSemver` with `Specifier2` for complete consolidation

### ⏳ Pending

- Port remaining tests from `specifier_test.rs` with pure method signatures
- Performance validation with simplified architecture
- Evaluate `BasicSemver` replacement strategy
- Plan migration from `Specifier` to `Specifier2`

## Benefits Summary

1. **Performance**: 31x cache speedup, zero thread overhead
2. **Simplicity**: Stateless design, easier to reason about
3. **Efficiency**: Lazy evaluation, work only when needed
4. **Scalability**: Perfect caching enables massive monorepo support
5. **Maintainability**: Cleaner separation of concerns

This refactor transforms `Specifier` from a stateful, context-dependent type into a pure, cacheable, high-performance component suitable for large-scale dependency management.
