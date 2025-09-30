# Syncpack: LLM Context Guide

This file provides essential context for LLM-assisted development sessions on Syncpack.

## Project Overview

Syncpack is a Rust command-line tool that ensures consistency across multiple package.json files in monorepos. It validates, fixes, formats, and updates npm dependency versions according to user-defined rules.

**Current Status:** Version 14 (Rust rewrite, alpha) - Replacing TypeScript v13

## Quick Facts

- **Language:** Rust
- **Domain:** npm monorepo dependency management
- **Architecture:** 3-phase pipeline (Create → Inspect → Run)
- **Testing:** Integration tests using TestBuilder pattern
- **Deployment:** npm registry with platform-specific binaries

## Core Concept

Users define "version groups" (rules like "all packages must use React 18.0.0" or "use highest semver found") and Syncpack validates/fixes dependencies across all package.json files.

## Essential Mental Model

```
User runs command → Read config & package.json files → Create Context
  ↓
Assign InstanceState to each dependency (valid/invalid/suspect)
  ↓
Command processes instances based on their states → Exit with 0 or 1
```

**Key insight:** Every dependency occurrence (e.g., React in package A's dependencies) is an "Instance" that gets tagged with a state describing if it follows the rules.

## The Three Critical Types

### 1. Context (owns everything)

```rust
pub struct Context {
    pub config: Config,              // CLI args + config file
    pub packages: Packages,          // All package.json files
    pub instances: Vec<Rc<Instance>>, // Every dependency occurrence
    pub version_groups: Vec<VersionGroup>, // Rules/policies
    // ...
}
```

### 2. Instance (a single dependency occurrence)

```rust
pub struct Instance {
    pub dependency_name: String,     // e.g., "react"
    pub specifier: Specifier,        // e.g., "18.0.0", "^18.0.0"
    pub state: InstanceState,        // Assigned during inspection
    // ...
}
```

### 3. InstanceState (validation result)

```rust
pub enum InstanceState {
    Valid(ValidInstance),           // ✓ Follows rules
    Invalid(InvalidInstance),       // ✗ Breaks rules (fixable/unfixable/conflict)
    Suspect(SuspectInstance),       // ⚠ Misconfiguration
    Unknown,                        // Not yet inspected
}
```

## Version Groups (The Rules)

Users configure these policies:

- **Banned** - Dependency shouldn't exist (e.g., deprecated packages)
- **Pinned** - Lock to exact version (e.g., all use "18.0.0")
- **HighestSemver** - Use highest version found in monorepo
- **LowestSemver** - Use lowest version found in monorepo
- **SameRange** - All version ranges must satisfy every other range in the group (e.g., ">=1.0.0" and "<=2.0.0" are compatible)
- **SameMinor** - Allow patch differences
- **SnappedTo** - Follow version from specific package
- **Ignored** - Skip validation

## File Structure Quick Map

```
src/
├── main.rs                 - Entry point, command dispatch
├── cli.rs                  - CLI argument parsing
├── config.rs               - Merged CLI + file config
├── context.rs              - Main data structure, owns everything
├── instance.rs             - Single dependency occurrence
├── instance_state.rs       - Validation state enums (14 Valid, 7 Fixable, 3 Unfixable, etc.)
├── version_group.rs        - Rule/policy definitions
├── packages.rs             - package.json file reading
├── visit_packages.rs       - Assigns InstanceState to dependencies
├── visit_formatting.rs     - Checks package.json formatting
│
├── commands/
│   ├── lint.rs             - Find issues
│   ├── fix.rs              - Auto-fix issues
│   ├── format.rs           - Format package.json files
│   ├── update.rs           - Update from npm registry
│   ├── list.rs             - List dependencies
│   └── json.rs             - JSON output
│
├── test/
│   ├── builder.rs          - TestBuilder (ALWAYS USE THIS)
│   ├── expect.rs           - Test assertions
│   └── mock.rs             - Mock utilities
│
└── visit_packages/         - Validation logic by rule type
    ├── banned_test.rs      - Tests for banned dependencies
    ├── pinned_test.rs      - Tests for pinned versions
    └── ...
```

## Common Tasks

### Reading dependency versions

→ Already done in Context::create(), available as ctx.instances

### Validating dependencies

→ Add logic in src/visit_packages/\*.rs, assign InstanceState

### Processing validated dependencies

→ Implement in src/commands/\*.rs, iterate and act on states

### Testing a scenario

→ Use TestBuilder in src/visit_packages/\*\_test.rs

## Data Flow Example

User runs: `syncpack lint`

1. **Create Phase:**
   - Parse CLI args: subcommand=Lint
   - Read .syncpackrc.json: version groups defined
   - Find package.json files via globs
   - Collect all dependencies as Instances
   - Assign each Instance to a version group

2. **Inspect Phase:**
   - visit_packages(ctx) iterates all Instances
   - For each: determine if it follows its version group's rules
   - Assign appropriate InstanceState (Valid/Invalid/Suspect)

3. **Run Phase:**
   - lint::run(ctx) iterates Instances
   - Filters for is_invalid()
   - Prints violations
   - Returns 1 if any found, 0 otherwise

## Testing Philosophy

- **Integration over unit tests** - Test full pipeline with realistic scenarios
- **Use TestBuilder** - Never manually construct Context
- **Test in visit_packages/\*\_test.rs** - Co-located with validation logic
- **Use JSON for inputs** - Mirrors real package.json structure

Example:

```rust
TestBuilder::new()
    .with_packages(vec![json!({"name": "pkg-a", "dependencies": {"react": "17.0.0"}})])
    .with_version_group(json!({"dependencies": ["react"], "pinned": "18.0.0"}))
    .build_and_visit_packages()
```

## Important Distinctions

**Instance vs Dependency:**

- Dependency = "react" (the package name)
- Instance = "react@17.0.0 in package-a's dependencies" (specific occurrence)

**visit_packages vs visit_formatting:**

- visit_packages: Validates dependency versions
- visit_formatting: Checks package.json structure (property order, sorting)

**Fixable vs Unfixable:**

- Fixable: We know what it should be (can auto-fix)
- Unfixable: Ambiguous, need human decision

**Context ownership:**

- Created once, passed through pipeline
- Each phase takes ownership, returns it
- Commands consume it (take ownership without returning)

## Rust-Specific Notes

- **Rc<Instance>**: Reference counted, single-threaded sharing (cheap clones)
- **Arc<T>**: Only for cross-thread data (e.g., RegistryClient in async)
- **Ownership**: Context flows through phases, commands consume it
- **Enums with data**: InstanceState::Valid(ValidInstance::IsPinned)

## Configuration Format

Users can write config in TypeScript, JavaScript, YAML, or JSON:

```json
{
  "versionGroups": [
    {
      "dependencies": ["react", "react-dom"],
      "packages": ["@my-org/*"],
      "pinned": "18.0.0"
    }
  ],
  "semverGroups": [
    {
      "dependencies": ["**"],
      "range": "^"
    }
  ]
}
```

## Key Behaviors

- **First match wins**: Instance assigned to first matching version group
- **Order matters**: SnappedTo groups process last (need targets fixed first)
- **Strict mode**: Suspect states treated as errors
- **Dry run**: Preview changes without writing files

## Where to Look

**Understanding architecture:** CONTRIBUTING.md
**Development patterns:** .notes/index.md
**Quick syntax lookup:** quick-reference.md
**Decision guidance:** decision-trees.md
**Code examples:** src/visit_packages/\*\_test.rs
**Real-world test data:** fixtures/fluid-framework/

## Common Questions

**Q: Where do I add validation logic?**
A: src/visit_packages/\*.rs - Assign InstanceState based on rules

**Q: How do I test it?**
A: Use TestBuilder in src/visit_packages/\*\_test.rs

**Q: Where are commands implemented?**
A: src/commands/\*.rs - Each has `pub fn run(ctx: Context) -> i32`

**Q: How do I run locally?**
A: `cargo run -- lint` or `cd fixtures/fluid-framework && cargo run -- lint`

**Q: What's the difference between Valid and Fixable?**
A: Valid = correct. Fixable = incorrect but we know the right value.

## Session Checklist

Before starting work:

- [ ] Understand which command/phase this affects
- [ ] Check if similar code exists (grep is your friend)
- [ ] Identify which InstanceState variants are relevant
- [ ] Know whether to use visit_packages or visit_formatting
- [ ] Have test examples ready (look at existing \*\_test.rs files)

## Red Flags

🚩 Assigning InstanceState in Context::create() - WRONG PHASE
🚩 Manually constructing Context - USE TESTBUILDER
🚩 File I/O in visit_packages() - ONLY TAG, DON'T MODIFY
🚩 Using Arc when Rc would work - UNNECESSARY OVERHEAD
🚩 Forgetting to update main.rs after adding command - WON'T RUN

## Success Criteria

Code is good when:
✓ Follows the 3-phase pattern
✓ Uses TestBuilder for tests
✓ States assigned in correct phase
✓ Follows existing naming conventions
✓ Has integration tests in visit_packages/
✓ Compiles and passes `just test`
