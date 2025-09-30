# Syncpack: LLM Context Guide

<purpose>
This file provides essential context for LLM-assisted development sessions on Syncpack.
</purpose>

<project_overview>

## Project Overview

Syncpack is a Rust command-line tool that ensures consistency across multiple package.json files in monorepos. It validates, fixes, formats, and updates npm dependency versions according to user-defined rules.

**Current Status:** Version 14 (Rust rewrite, alpha) - Replacing TypeScript v13

</project_overview>

<quick_facts>

## Quick Facts

- **Language:** Rust
- **Domain:** npm monorepo dependency management
- **Architecture:** 3-phase pipeline (Create → Inspect → Run)
- **Testing:** Integration tests using TestBuilder pattern
- **Deployment:** npm registry with platform-specific binaries

</quick_facts>

<core_concept>

## Core Concept

Users define "version groups" (rules like "all packages must use React 18.0.0" or "use highest semver found") and Syncpack validates/fixes dependencies across all package.json files.

</core_concept>

<mental_model>

## Essential Mental Model

```
User runs command → Read config & package.json files → Create Context
  ↓
Assign InstanceState to each dependency (valid/invalid/suspect)
  ↓
Command processes instances based on their states → Exit with 0 or 1
```

**Key insight:** Every dependency occurrence (e.g., React in package A's dependencies) is an "Instance" that gets tagged with a state describing if it follows the rules.

</mental_model>

<critical_types>

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

</critical_types>

<version_groups>

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

</version_groups>

<file_structure>

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

</file_structure>

<common_tasks>

## Common Tasks

<task type="reading_dependencies">
Reading dependency versions → Already done in Context::create(), available as ctx.instances
</task>

<task type="validation">
Validating dependencies → Add logic in src/visit_packages/*.rs, assign InstanceState
</task>

<task type="processing">
Processing validated dependencies → Implement in src/commands/*.rs, iterate and act on states
</task>

<task type="testing">
Testing a scenario → Use TestBuilder in src/visit_packages/*_test.rs
</task>

</common_tasks>

<data_flow_example>

## Data Flow Example

User runs: `syncpack lint`

<phase name="create">

### 1. Create Phase

- Parse CLI args: subcommand=Lint
- Read .syncpackrc.json: version groups defined
- Find package.json files via globs
- Collect all dependencies as Instances
- Assign each Instance to a version group

</phase>

<phase name="inspect">

### 2. Inspect Phase

- visit_packages(ctx) iterates all Instances
- For each: determine if it follows its version group's rules
- Assign appropriate InstanceState (Valid/Invalid/Suspect)

</phase>

<phase name="run">

### 3. Run Phase

- lint::run(ctx) iterates Instances
- Filters for is_invalid()
- Prints violations
- Returns 1 if any found, 0 otherwise

</phase>

</data_flow_example>

<testing_philosophy>

## Testing Philosophy

- **Integration over unit tests** - Test full pipeline with realistic scenarios
- **Use TestBuilder** - Never manually construct Context
- **Test in `visit_packages/*_test.rs`** - Co-located with validation logic
- **Use JSON for inputs** - Mirrors real package.json structure

<test_example>

```rust
TestBuilder::new()
    .with_packages(vec![json!({"name": "pkg-a", "dependencies": {"react": "17.0.0"}})])
    .with_version_group(json!({"dependencies": ["react"], "pinned": "18.0.0"}))
    .build_and_visit_packages()
```

</test_example>

</testing_philosophy>

<important_distinctions>

## Important Distinctions

<distinction type="terminology">

**Instance vs Dependency:**

- Dependency = "react" (the package name)
- Instance = "react@17.0.0 in package-a's dependencies" (specific occurrence)

</distinction>

<distinction type="validation_type">

**visit_packages vs visit_formatting:**

- visit_packages: Validates dependency versions
- visit_formatting: Checks package.json structure (property order, sorting)

</distinction>

<distinction type="fixability">

**Fixable vs Unfixable:**

- Fixable: We know what it should be (can auto-fix)
- Unfixable: Ambiguous, need human decision

</distinction>

<distinction type="ownership">

**Context ownership:**

- Created once, passed through pipeline
- Each phase takes ownership, returns it
- Commands consume it (take ownership without returning)

</distinction>

</important_distinctions>

<rust_specifics>

## Rust-Specific Notes

- **Rc<Instance>**: Reference counted, single-threaded sharing (cheap clones)
- **Arc<T>**: Only for cross-thread data (e.g., RegistryClient in async)
- **Ownership**: Context flows through phases, commands consume it
- **Enums with data**: InstanceState::Valid(ValidInstance::IsPinned)

</rust_specifics>

<configuration_format>

## Configuration Format

Users can write config in TypeScript, JavaScript, YAML, or JSON:

<config_example>

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

</config_example>

</configuration_format>

<key_behaviors>

## Key Behaviors

- **First match wins**: Instance assigned to first matching version group
- **Order matters**: SnappedTo groups process last (need targets fixed first)
- **Strict mode**: Suspect states treated as errors
- **Dry run**: Preview changes without writing files

</key_behaviors>

<navigation>

## Where to Look

<reference_by_task>

- **Understanding architecture:** CONTRIBUTING.md
- **Development patterns:** .notes/index.md
- **Quick syntax lookup:** quick-reference.md
- **Decision guidance:** decision-trees.md
- **Code examples:** `src/visit_packages/*_test.rs`
- **Real-world test data:** fixtures/fluid-framework/

</reference_by_task>

</navigation>

<common_questions>

## Common Questions

<qa>
<q>Where do I add validation logic?</q>
<a>src/visit_packages/*.rs - Assign InstanceState based on rules</a>
</qa>

<qa>
<q>How do I test it?</q>
<a>Use TestBuilder in src/visit_packages/*_test.rs</a>
</qa>

<qa>
<q>Where are commands implemented?</q>
<a>src/commands/*.rs - Each has `pub fn run(ctx: Context) -> i32`</a>
</qa>

<qa>
<q>How do I run locally?</q>
<a>`cargo run -- lint` or `cd fixtures/fluid-framework && cargo run -- lint`</a>
</qa>

<qa>
<q>What's the difference between Valid and Fixable?</q>
<a>Valid = correct. Fixable = incorrect but we know the right value.</a>
</qa>

</common_questions>

<session_checklist>

## Session Checklist

Before starting work:

- [ ] Understand which command/phase this affects
- [ ] Check if similar code exists (grep is your friend)
- [ ] Identify which InstanceState variants are relevant
- [ ] Know whether to use visit_packages or visit_formatting
- [ ] Have test examples ready (look at existing `*_test.rs` files)

</session_checklist>

<red_flags>

## Red Flags

<warning>🚩 Assigning InstanceState in Context::create() - WRONG PHASE</warning>
<warning>🚩 Manually constructing Context - USE TESTBUILDER</warning>
<warning>🚩 File I/O in visit_packages() - ONLY TAG, DON'T MODIFY</warning>
<warning>🚩 Using Arc when Rc would work - UNNECESSARY OVERHEAD</warning>
<warning>🚩 Forgetting to update main.rs after adding command - WON'T RUN</warning>

</red_flags>

<success_criteria>

## Success Criteria

Code is good when:

<checklist>
✓ Follows the 3-phase pattern
✓ Uses TestBuilder for tests
✓ States assigned in correct phase
✓ Follows existing naming conventions
✓ Has integration tests in visit_packages/
✓ Compiles and passes `just test`
</checklist>

</success_criteria>
