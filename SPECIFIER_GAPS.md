# NPM Version Specifier Specification vs Syncpack Specifier2 Implementation

## Real-World Usage Analysis

**Source**: 216 package.json files in fluid-framework fixture (Microsoft's FluidFramework monorepo)

### ✅ HEAVILY USED (Must Support)

- **Caret/Tilde ranges** - Thousands of uses (`^X.Y.Z`, `~X.Y.Z`)
- **Workspace protocol** - 1,689 uses (`workspace:~`, `workspace:^`)
- **Exact versions** - 365 uses (`2.5.0`, `7.47.8`)
- **Comparison operators** - 7 uses (`>=X.Y.Z` in engines.node)
- **Prerelease versions** - 4 uses (`0.0.0-alpha.5`)

### 📦 BUILD METADATA (Special Case)

- Only in `packageManager` field (pnpm SHA hashes)
- **Not found in dependency versions**

### ❌ NOT FOUND (But Used Elsewhere)

- `~>` operator, hyphen ranges (`1.2.3 - 2.3.4`), `||` OR ranges
- Comparison operators with partial versions (`>1`, `>1.2`)
- `*` wildcards, loose mode syntax

---

## Gap Analysis & Implementation Priority

### 🚀 PRIORITY 1: MUST IMPLEMENT

#### 1.1 Workspace Protocol Resolution ⭐ CRITICAL

- **Status**: 1,689 uses in fluid-framework
- **Action**: In progress - `resolve_workspace_protocol()` method
- **Priority**: P0 - Essential for monorepo support

#### 1.2 Comparison Operators (`>=`, `>`, `<`, `<=`)

- **Status**: 7 uses (engines.node)
- **Action**: Support basic operators, delegate to node_semver
- **Examples**: `>1` → `>=2.0.0`, `<1.2` → `<1.2.0-0`
- **Priority**: P1

#### 1.3 Hyphen Ranges (`1.2.3 - 2.3.4`)

- **Action**: Parse as `ComplexSemver`, delegate to node_semver
- **Example**: `1.2.3 - 2.3` → `>=1.2.3 <2.4.0-0` (asymmetric)
- **Priority**: P1

#### 1.4 OR Ranges (`||`)

- **Action**: Parse as `ComplexSemver`
- **Example**: `1.2.7 || >=1.2.9 <2.0.0`
- **Priority**: P1

#### 1.5 Wildcards (`x`, `X`)

- **Action**: Add `x`/`X` pattern support (`*` already via `Latest`)
- **Example**: `1.x` → `>=1.0.0 <2.0.0-0`
- **Priority**: P1

#### 1.6 Prerelease Comparison

- **Status**: 4 found in fluid-framework
- **Action**: VERIFY node_semver handles correctly
- **Details**: Numeric as int, strings lexically, numeric < alphanumeric
- **Priority**: P1

#### 1.7 Invalid Version Handling

- **Action**: Return `false` instead of panicking
- **Example**: `satisfies('not-a-version', '*')` → `false`
- **Priority**: P1

---

### ✅ PRIORITY 2: VERIFY (Likely Working)

Node_semver delegation should handle these - add tests to confirm:

#### 2.1 Whitespace Normalization

- `>  1.0.0` → `>1.0.0`
- Priority: P2

#### 2.2 Build Metadata Stripping

- `^1.2.3+build` → `>=1.2.3 <2.0.0-0`
- Priority: P2

#### 2.3 Validation Constants

- MAX_LENGTH: 256, MAX_SAFE_INTEGER: 9007199254740991
- Priority: P3

---

### ❌ PRIORITY 3: SKIP (Obscure/Not Needed)

- **`~>` operator** - Ruby/Bundler legacy, classify as `Unsupported`
- **Empty comparator sets** - Invalid syntax
- **Coercion features** - For changelogs, not package.json
- **Loose mode** - Bad practice, enforce strict semver
- **Range intersection semantics** - Not relevant to Syncpack

---

## Implementation Roadmap

### Phase 1: Critical (P0-P1)

1. ✅ Workspace protocol resolution (in progress)
2. Comparison operators with partial versions
3. Hyphen ranges, OR ranges, wildcards
4. Invalid version handling
5. Verify prerelease comparison

### Phase 2: Verification (P2)

1. Whitespace normalization tests
2. Build metadata handling tests
3. Validation constants

### Phase 3: Documentation (P3)

1. Document unsupported features
2. Update error messages

---

## Key Findings

1. **Workspace protocol critical** - 1,689 uses in fluid-framework
2. **Caret/tilde ranges dominate** - Thousands of uses
3. **Exotic syntax rare** - `~>`, hyphen ranges, OR ranges absent
4. **Build metadata niche** - packageManager field only
5. **Prerelease uncommon** - Only 4 in 216 packages
6. **node_semver handles most edge cases** via delegation

**Bottom Line**: Focus on workspace protocol, basic operators, verify node_semver delegation. Skip obscure features.
