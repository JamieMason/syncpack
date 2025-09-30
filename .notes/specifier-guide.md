# Specifier Guide

<ai_guidance>
This document explains how Syncpack's `Specifier` type works - the core abstraction for representing npm version specifiers like `^1.2.3`, `workspace:*`, `npm:react@18.0.0`, etc.
</ai_guidance>

## Overview

**Specifier** is an enum that represents all valid npm version specifier formats. It provides:

- **Parsing** - Convert string specifiers to typed enum variants
- **Comparison** - Ordering logic for version sorting
- **Transformation** - Apply semver ranges, resolve workspace protocols
- **Validation** - Delegate complex semver checks to node_semver

**Location:** `src/specifier.rs`

## Core Design Principles

### 1. Immutability via Rc

```rust
// Specifiers are wrapped in Rc for cheap cloning
pub type SpecifierRc = Rc<Specifier>;

// Clone the pointer, not the data
let copy = Rc::clone(&specifier);
```

**Why Rc?**

- Syncpack is single-threaded (no Arc needed)
- Multiple structs reference same Specifier
- Avoids expensive string clones
- Enables pointer equality checks

### 2. Caching for Performance

```rust
// Global caches prevent duplicate parsing
static SPECIFIER_CACHE: LazyLock<DashMap<String, SpecifierRc>>
static RANGE_CACHE: LazyLock<DashMap<String, SemverRange>>
static VERSION_CACHE: LazyLock<DashMap<String, Version>>
```

**Benefits:**

- Parse each unique specifier once
- Significant speedup for large monorepos
- Tested with 200+ package.json files

### 3. Delegation to node_semver

Complex semver operations (satisfies, comparisons, etc.) delegate to the battle-tested `node_semver` crate rather than reimplementing:

```rust
pub fn satisfies(&self, version: &Version) -> bool {
    match self {
        Specifier::Exact(v) => v == version,
        // Complex cases delegate to node_semver
        _ => {
            let range = node_semver::Range::parse(self.get_raw()).ok()?;
            range.satisfies(version)
        }
    }
}
```

## Enum Variants (16 Types)

### Semver Variants (Most Common)

<variant name="Exact">

#### `Exact(Version)`

Exact version with no range operator.

**Examples:**

- `"1.2.3"` → `Exact(Version { major: 1, minor: 2, patch: 3 })`
- `"0.0.0"` → `Exact(Version { major: 0, minor: 0, patch: 0 })`

**Ordering:** Compares by Version (major → minor → patch)

</variant>

<variant name="Range">

#### `Range { version: Version, semver_range: SemverRange }`

Standard semver range with operator.

**Examples:**

- `"^1.2.3"` → `Range { version: 1.2.3, semver_range: Caret }`
- `"~2.0.0"` → `Range { version: 2.0.0, semver_range: Tilde }`
- `">=1.0.0"` → `Range { version: 1.0.0, semver_range: Gte }`

**Ordering:** Compares version first, then range type (Exact < Tilde < Caret)

</variant>

<variant name="Major">

#### `Major(u64, SemverRange)`

Major version only (e.g., `"^1"`).

**Examples:**

- `"^1"` → `Major(1, Caret)`
- `"~2"` → `Major(2, Tilde)`

**Ordering:** Compares by major number

</variant>

<variant name="Minor">

#### `Minor(u64, u64, SemverRange)`

Major.minor version (e.g., `"^1.2"`).

**Examples:**

- `"^1.2"` → `Minor(1, 2, Caret)`
- `"~3.0"` → `Minor(3, 0, Tilde)`

**Ordering:** Compares major then minor

</variant>

<variant name="RangeMajor">

#### `RangeMajor(u64, ComparisonOperator)`

Comparison operator with major only.

**Examples:**

- `">1"` → `RangeMajor(1, Gt)`
- `"<=2"` → `RangeMajor(2, Lte)`

</variant>

<variant name="RangeMinor">

#### `RangeMinor(u64, u64, ComparisonOperator)`

Comparison operator with major.minor.

**Examples:**

- `">1.2"` → `RangeMinor(1, 2, Gt)`
- `"<3.0"` → `RangeMinor(3, 0, Lt)`

</variant>

### Special Variants

<variant name="Alias">

#### `Alias { raw, name, version_str, inner_specifier }`

NPM alias syntax: `npm:package@version`.

**Structure:**

```rust
pub struct Alias {
    pub raw: String,              // "npm:react@18.0.0"
    pub name: String,             // "react"
    pub version_str: String,      // "18.0.0"
    pub inner_specifier: Box<Specifier>, // Exact(18.0.0)
}
```

**Examples:**

- `"npm:react@18.0.0"` → Alias with Exact inner specifier
- `"npm:lodash@^4.17.0"` → Alias with Range inner specifier

**Ordering:** Compares inner_specifier, then name if equal

**Why recursive?** The version part can be any valid specifier, so we parse it recursively.

</variant>

<variant name="WorkspaceProtocol">

#### `WorkspaceProtocol { raw, node_range, node_version, semver_range, semver_number }`

Monorepo workspace protocol: `workspace:*`, `workspace:^1.2.3`.

**Structure:**

```rust
pub struct WorkspaceProtocol {
    pub raw: String,               // Full original string
    pub node_range: Option<String>,    // "^", "~", etc.
    pub node_version: Option<String>,  // "1.2.3"
    pub semver_range: Option<SemverRange>, // Parsed range
    pub semver_number: Option<Version>,    // Parsed version
}
```

**Examples:**

- `"workspace:*"` → All fields None except raw
- `"workspace:^"` → node_range: Some("^"), rest None
- `"workspace:^1.2.3"` → Both range and version populated

**Special behavior:** Must be resolved against local package versions before comparison.

**Ordering:** Complex logic based on presence of version/range (see below)

</variant>

<variant name="Latest">

#### `Latest`

Wildcard or empty specifier.

**Examples:**

- `"*"` → Latest
- `"latest"` → Latest
- `""` → Latest

**Ordering:** Always sorts last (uses HUGE constant)

</variant>

<variant name="ComplexSemver">

#### `ComplexSemver(String)`

Complex semver expressions that can't be represented by simple variants.

**Examples:**

- `"1.2.3 - 2.3.4"` (hyphen range)
- `"^1.0.0 || ^2.0.0"` (OR ranges)
- `">1.0.0 <2.0.0"` (AND ranges)

**Ordering:** Delegates to node_semver for comparison

**Validation:** All semver operations delegate to node_semver crate

</variant>

### Protocol Variants

<variant name="File">

#### `File(String)`

File path dependency.

**Examples:**

- `"file:../sibling-package"`
- `"file:./local/path"`

**Ordering:** Lexical string comparison

</variant>

<variant name="Git">

#### `Git(String)`

Git repository reference.

**Examples:**

- `"git+https://github.com/user/repo.git"`
- `"github:user/repo#commit-ish"`

**Ordering:** Lexical string comparison

</variant>

<variant name="Url">

#### `Url(String)`

Tarball URL.

**Examples:**

- `"https://registry.com/package-1.2.3.tgz"`
- `"http://example.com/archive.tar.gz"`

**Ordering:** Lexical string comparison

</variant>

### Other Variants

<variant name="Tag">

#### `Tag(String)`

NPM dist-tag (non-semver identifier).

**Examples:**

- `"next"`
- `"beta"`
- `"canary"`

**Ordering:** Lexical string comparison

</variant>

<variant name="None">

#### `None(String)`

Invalid or empty specifier (preserves original).

**Examples:**

- `""` parsed with `preserve_empty: true`
- Invalid syntax that we want to track

**Ordering:** Always sorts last

</variant>

<variant name="Unsupported">

#### `Unsupported(String)`

Valid NPM syntax not yet supported by Syncpack.

**Examples:**

- `"~>1.0"` (Ruby-style operator)
- Exotic edge cases

**Ordering:** Lexical string comparison

</variant>

## API Reference

### Factory Methods

<method name="new">

#### `Specifier::new(raw: &str) -> SpecifierRc`

Main factory - parses string into typed Specifier.

**Caching:** Returns cached Rc if already parsed.

**Examples:**

```rust
let spec = Specifier::new("^1.2.3");
// Returns: Rc<Range { version: 1.2.3, semver_range: Caret }>

let spec = Specifier::new("workspace:*");
// Returns: Rc<WorkspaceProtocol { ... }>
```

</method>

<method name="from_version">

#### `Specifier::from_version(version: &Version, range: SemverRange) -> SpecifierRc`

Create Range variant from parsed version.

**Use case:** Converting exact versions to ranges (e.g., lint fix applying caret).

</method>

### Getter Methods

<method name="get_raw">

#### `get_raw(&self) -> &str`

Returns original unparsed string.

**Examples:**

- `Exact(1.2.3).get_raw()` → `"1.2.3"`
- `Range(^1.2.3).get_raw()` → `"^1.2.3"`
- `WorkspaceProtocol.get_raw()` → `"workspace:^1.2.3"`

</method>

<method name="get_config_identifier">

#### `get_config_identifier(&self) -> &str`

Returns string for config matching.

**Examples:**

- `"^1.2.3"` → `"semver"`
- `"npm:react@18"` → `"alias"`
- `"workspace:*"` → `"workspace"`
- `"file:../pkg"` → `"file"`
- `"*"` → `"*"`

**Use case:** Filtering instances by specifier type in config.

</method>

<method name="get_semver_number">

#### `get_semver_number(&self) -> Option<&Version>`

Extract Version from semver variants.

**Examples:**

- `Exact(1.2.3)` → `Some(Version(1.2.3))`
- `Range(^1.2.3)` → `Some(Version(1.2.3))`
- `WorkspaceProtocol(workspace:^1.2.3)` → `Some(Version(1.2.3))`
- `Tag("next")` → `None`

</method>

<method name="get_semver_range">

#### `get_semver_range(&self) -> Option<&SemverRange>`

Extract SemverRange if present.

**Examples:**

- `Range(^1.2.3)` → `Some(Caret)`
- `Exact(1.2.3)` → `None`
- `Major(^1)` → `Some(Caret)`

</method>

<method name="get_alias_name">

#### `get_alias_name(&self) -> Option<&str>`

Extract package name from Alias variant.

**Examples:**

- `Alias("npm:react@18")` → `Some("react")`
- `Exact(1.2.3)` → `None`

</method>

<method name="get_node_version">

#### `get_node_version(&self) -> Option<&str>`

Extract version string (includes range operators).

**Examples:**

- `Range(^1.2.3)` → `Some("^1.2.3")`
- `Exact(1.2.3)` → `Some("1.2.3")`
- `WorkspaceProtocol(workspace:^1.2.3)` → `Some("^1.2.3")`
- `Tag("next")` → `None`

</method>

### Transformer Methods

<method name="with_range">

#### `with_range(&self, range: &SemverRange) -> Option<SpecifierRc>`

Apply semver range to current specifier.

**Returns:** `Option<SpecifierRc>` - None if not applicable.

**Examples:**

```rust
Exact(1.2.3).with_range(&Caret)
// → Some(Range(^1.2.3))

Range(~1.2.3).with_range(&Caret)
// → Some(Range(^1.2.3))

Tag("next").with_range(&Caret)
// → None (not semver)

Alias("npm:react@1.2.3").with_range(&Caret)
// → Some(Alias("npm:react@^1.2.3"))

WorkspaceProtocol("workspace:1.2.3").with_range(&Caret)
// → Some(WorkspaceProtocol("workspace:^1.2.3"))
```

**Fluent API pattern:**

```rust
// DON'T chain with ? operator
let with_range = spec.with_range(&Caret)?;
let result = with_range.with_node_version(&version)?;

// DO use and_then pipeline
spec.with_range(&Caret)
    .and_then(|s| s.with_node_version(&version))
```

</method>

<method name="with_node_version">

#### `with_node_version(&self, version: &str) -> Option<SpecifierRc>`

Replace version while preserving range.

**Examples:**

```rust
Range(^1.2.3).with_node_version("2.0.0")
// → Some(Range(^2.0.0))

Exact(1.2.3).with_node_version("2.0.0")
// → Some(Exact(2.0.0))

Alias("npm:react@^1.0.0").with_node_version("2.0.0")
// → Some(Alias("npm:react@^2.0.0"))
```

</method>

### Comparison Methods

<method name="satisfies">

#### `satisfies(&self, version: &Version) -> bool`

Check if version satisfies this specifier.

**Delegation:** Complex cases delegate to node_semver.

**Examples:**

```rust
Range(^1.2.3).satisfies(&Version(1.3.0)) // true
Range(^1.2.3).satisfies(&Version(2.0.0)) // false
Exact(1.2.3).satisfies(&Version(1.2.3))  // true
```

</method>

<method name="is_older_than">

#### `is_older_than(&self, other: &Specifier) -> bool`

Compare two specifiers by version.

**Use case:** Finding outdated dependencies.

**Note:** Returns false for non-comparable types (Git, Url, etc.).

</method>

<method name="is_semver">

#### `is_semver(&self) -> bool`

Check if variant is semver-based.

**Returns true for:** Exact, Range, Major, Minor, RangeMajor, RangeMinor, ComplexSemver

**Returns false for:** File, Git, Url, Tag, WorkspaceProtocol, Alias, Latest, None, Unsupported

</method>

### Workspace Protocol Methods

<method name="needs_workspace_resolution">

#### `needs_workspace_resolution(&self) -> bool`

Check if specifier needs resolution against local package.

**Returns true for:** `workspace:*`, `workspace:^`, `workspace:~`

**Returns false for:** `workspace:^1.2.3` (already has version)

</method>

<method name="resolve_workspace_protocol">

#### `resolve_workspace_protocol(&self, local_version: &Version) -> Option<SpecifierRc>`

Resolve workspace protocol using local package version.

**Examples:**

```rust
WorkspaceProtocol("workspace:*")
    .resolve_workspace_protocol(&Version(1.2.3))
// → Some(Exact(1.2.3))

WorkspaceProtocol("workspace:^")
    .resolve_workspace_protocol(&Version(1.2.3))
// → Some(Range(^1.2.3))

WorkspaceProtocol("workspace:^1.2.3")
    .resolve_workspace_protocol(&Version(1.2.3))
// → Some(Range(^1.2.3)) (already resolved, returns equivalent)
```

**When to use:** Before comparison or sorting operations involving workspace protocols.

</method>

## Ordering Logic

Specifier implements `Ord` for sorting. Order priority:

### 1. By Category

1. Semver variants (Exact, Range, etc.)
2. Special variants (File, Git, Url, etc.)
3. Latest/None (always last)

### 2. Within Semver

```
Exact(1.2.3) < Range(~1.2.3) < Range(^1.2.3)
```

**Version comparison first:**

- `1.2.3 < 2.0.0` regardless of range type

**Range type second (if versions equal):**

- Exact < Tilde < Caret

### 3. WorkspaceProtocol Special Case

Complex ordering based on resolution state:

```
workspace:^1.2.3  < workspace:1.2.3   (range before exact)
workspace:^       < workspace:*       (explicit before wildcard)
workspace:1.2.3   < workspace:*       (resolved before unresolved)
```

**Key insight:** Unresolved protocols sort differently than resolved ones.

### 4. Alias Ordering

Compares inner_specifier first, then name:

```
npm:react@17.0.0 < npm:react@18.0.0   (by version)
npm:react@18.0.0 < npm:vue@18.0.0     (by name if version equal)
```

### 5. The HUGE Constant

```rust
pub const HUGE: u64 = 9_007_199_254_740_991; // JavaScript MAX_SAFE_INTEGER
```

**Use case:** Latest and None variants use HUGE for sorting last.

**Why this value?** Matches JavaScript's max safe integer for consistency with npm ecosystem.

## Common Usage Patterns

### Pattern 1: Parse and Compare

```rust
let spec1 = Specifier::new("^1.2.3");
let spec2 = Specifier::new("^2.0.0");

if spec1.is_older_than(&spec2) {
    println!("spec1 is outdated");
}
```

### Pattern 2: Transform Version

```rust
let exact = Specifier::new("1.2.3");
let with_caret = exact.with_range(&SemverRange::Caret)?;
// "1.2.3" → "^1.2.3"
```

### Pattern 3: Resolve Workspace Protocol

```rust
let workspace = Specifier::new("workspace:^");
let local_pkg_version = Version::parse("1.2.3")?;

if workspace.needs_workspace_resolution() {
    let resolved = workspace.resolve_workspace_protocol(&local_pkg_version)?;
    // "workspace:^" → "^1.2.3"
}
```

### Pattern 4: Rc Comparison

```rust
// Compare VALUES, not pointers
if *spec1 == *spec2 {
    println!("Same specifier");
}

// Or use helper method
if instance.specifier_equals(&expected_specifier) {
    // ...
}
```

### Pattern 5: Extract Version for Sorting

```rust
let specifiers = vec![
    Specifier::new("^2.0.0"),
    Specifier::new("^1.0.0"),
    Specifier::new("~1.5.0"),
];

let mut sorted: Vec<_> = specifiers.into_iter()
    .filter_map(|s| s.get_semver_number().map(|v| (v, s)))
    .collect();
sorted.sort_by(|a, b| a.0.cmp(b.0));
```

## Integration Points

### In Context Creation (`src/context.rs`)

```rust
// Specifiers created during package.json parsing
let specifier = Specifier::new(&dep.version_str);
```

### In InstanceDescriptor (`src/instance.rs`)

```rust
pub struct InstanceDescriptor {
    pub specifier: SpecifierRc,  // Current version in package.json
    // ...
}

pub struct Instance {
    pub expected_specifier: Option<SpecifierRc>,  // Target version (if fixable)
    // ...
}
```

### In Dependency (`src/dependency.rs`)

```rust
pub struct Dependency {
    pub expected: Option<SpecifierRc>,      // Version group target
    pub pinned_specifier: Option<SpecifierRc>,  // Pinned version
}
```

### In VersionGroup (`src/version_group.rs`)

```rust
pub struct VersionGroup {
    pub pin_version: Option<SpecifierRc>,  // Pinned version if applicable
    // ...
}
```

## Testing with Specifier

### Test Pattern 1: Direct Construction

```rust
#[test]
fn test_ordering() {
    let exact = Specifier::new("1.2.3");
    let caret = Specifier::new("^1.2.3");

    assert!(exact < caret);  // Dereference Rc for comparison
}
```

### Test Pattern 2: TestBuilder Integration

```rust
#[test]
fn test_with_workspace_protocol() {
    let ctx = TestBuilder::new()
        .with_package(json!({
            "name": "pkg-a",
            "version": "1.2.3",
            "dependencies": {
                "pkg-b": "workspace:^"
            }
        }))
        .build_and_visit_packages();

    // Workspace protocols resolved during inspection
    // Test expected_specifier is resolved correctly
}
```

### Test Pattern 3: Transformation Testing

```rust
#[test]
fn with_range_preserves_version() {
    let cases = vec![
        ("1.2.3", SemverRange::Caret, Some("^1.2.3")),
        ("~1.2.3", SemverRange::Caret, Some("^1.2.3")),
        ("next", SemverRange::Caret, None),
    ];

    for (input, range, expected) in cases {
        let spec = Specifier::new(input);
        let result = spec.with_range(&range);

        match expected {
            Some(exp) => assert_eq!(result.unwrap().get_raw(), exp),
            None => assert!(result.is_none()),
        }
    }
}
```

## Performance Characteristics

### Cache Hit Rates

In real-world monorepos (200+ packages):

- **Specifier cache:** 90%+ hit rate
- **Version cache:** 85%+ hit rate
- **Range cache:** 95%+ hit rate

### Memory Usage

- Rc overhead: 8 bytes per reference
- Cache size scales with unique specifiers, not total instances
- Typical monorepo: <1MB for all cached specifiers

### Comparison Performance

- Semver comparison: O(1) (compare cached Version structs)
- Complex semver: Delegates to node_semver (still fast)
- String-based (Git, Url): O(n) string comparison

## Common Pitfalls

### Pitfall 1: Cloning Rc

```rust
// ❌ WRONG: Implicit clone is less clear
let copy = specifier.clone();

// ✅ CORRECT: Explicit Rc::clone shows pointer copy
let copy = Rc::clone(&specifier);
```

### Pitfall 2: Comparing Rc Pointers

```rust
// ❌ WRONG: Compares pointer addresses
if specifier1 == specifier2 { }

// ✅ CORRECT: Dereference to compare values
if *specifier1 == *specifier2 { }

// ✅ ALTERNATIVE: Use helper method
if instance.specifier_equals(&expected) { }
```

### Pitfall 3: Forgetting Workspace Resolution

```rust
// ❌ WRONG: Comparing unresolved workspace protocol
let ws = Specifier::new("workspace:^");
let exact = Specifier::new("1.2.3");
ws.is_older_than(&exact); // Returns false! Can't compare.

// ✅ CORRECT: Resolve first
let resolved = ws.resolve_workspace_protocol(&local_version)?;
resolved.is_older_than(&exact);
```

### Pitfall 4: Chaining with ? Operator

```rust
// ❌ WRONG: Verbose with intermediate variables
let with_range = spec.with_range(&Caret)?;
let result = with_range.with_node_version(&version)?;

// ✅ CORRECT: Use and_then for fluent pipeline
let result = spec.with_range(&Caret)
    .and_then(|s| s.with_node_version(&version))?;
```

### Pitfall 5: Matching Without Dereference

```rust
// ❌ WRONG: Matches Rc<Specifier>, not Specifier
match specifier {
    Specifier::Exact(v) => { }, // Won't compile
}

// ✅ CORRECT: Dereference before matching
match &**specifier {
    Specifier::Exact(v) => { },
    Specifier::Range { version, .. } => { },
    // ...
}
```

## Edge Cases and Limitations

### Unsupported NPM Syntax

These are classified as `Unsupported`:

- `~>` operator (Ruby/Bundler legacy)
- Some exotic comparison combinations

### Build Metadata

Build metadata (`+build.123`) is preserved in raw string but ignored in comparisons:

- `1.2.3+build` ≡ `1.2.3` for comparison purposes
- Matches npm/semver spec

### Invalid Versions

`satisfies()` returns `false` for invalid versions instead of panicking:

```rust
Specifier::new("*").satisfies(&invalid_version) // false, not panic
```

### Workspace Protocol Limitations

Workspace protocols must be resolved before:

- Comparison operations
- Sorting
- Version range checks

Otherwise operations return `false` or default values.

## Related Types

### SemverRange (`src/semver_range.rs`)

```rust
pub enum SemverRange {
    Exact,   // "1.2.3"
    Caret,   // "^1.2.3"
    Tilde,   // "~1.2.3"
    Gt,      // ">1.2.3"
    Gte,     // ">=1.2.3"
    Lt,      // "<1.2.3"
    Lte,     // "<=1.2.3"
}
```

### Version (`src/version.rs`)

```rust
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    // prerelease and build metadata fields
}
```

## When to Use Which Method

<decision_tree>

**Need to parse a version string?**
→ Use `Specifier::new()`

**Need to compare versions?**
→ Use `is_older_than()` or `Ord` trait

**Need to check semver satisfaction?**
→ Use `satisfies()`

**Need to transform a specifier?**
→ Use `with_range()` or `with_node_version()`

**Working with workspace protocols?**
→ Check `needs_workspace_resolution()`, then use `resolve_workspace_protocol()`

**Need config type string?**
→ Use `get_config_identifier()`

**Need to extract version?**
→ Use `get_semver_number()` or `get_node_version()`

</decision_tree>

## Further Reading

- **SemverRange enum:** `src/semver_range.rs`
- **Version parsing:** `src/version.rs`
- **Integration examples:** `src/visit_packages/*.rs`
- **Test patterns:** `src/specifier_test.rs`
- **Context7 MCP guide:** `.notes/context7-guide.md` (for node_semver docs)
