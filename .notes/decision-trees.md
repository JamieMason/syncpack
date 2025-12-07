# Decision Trees for Common Development Scenarios

This guide provides decision trees and flowcharts to help you make the right architectural and implementation choices when working on Syncpack.

## Table of Contents

- [Should I Use visit_packages or visit_formatting?](#should-i-use-visit_packages-or-visit_formatting)
- [What InstanceState Variant Should I Use?](#what-instancestate-variant-should-i-use)
- [What VersionGroupVariant Does My Feature Need?](#what-versiongroupvariant-does-my-feature-need)
- [Where Should I Add My Test?](#where-should-i-add-my-test)
- [Should I Create a New Command?](#should-i-create-a-new-command)
- [How Should I Handle This Error?](#how-should-i-handle-this-error)
- [Should I Use Rc or Arc?](#should-i-use-rc-or-arc)

---

## Should I Use visit_packages or visit_formatting?

```
Does your command deal with dependency versions?
├─ YES → Use visit_packages
│         Examples: lint, fix, update, list, json
│
└─ NO → Does it deal with package.json structure/formatting?
         ├─ YES → Use visit_formatting
         │         Examples: format
         │
         └─ NO → Neither (rare case)
                  Consider if you need a new visitor
```

**Key Difference:**

- `visit_packages` → Assigns `InstanceState` to dependency instances
- `visit_formatting` → Checks formatting rules (property order, sorting, etc.)

---

## What InstanceState Variant Should I Use?

### Step 1: Is the instance correct?

```
Is the instance following the rules correctly?
├─ YES → Use Valid variant
│        ├─ Is it a local package? → IsLocalAndValid
│        ├─ Is it ignored? → IsIgnored
│        ├─ Does it match group policy? → IsHighestOrLowestSemver, IsPinned, etc.
│        └─ Other valid states...
│
└─ NO → Continue to Step 2
```

### Step 2: Can we fix it automatically?

```
Can we automatically determine the correct value?
├─ YES → Use Invalid::Fixable variant
│        ├─ Should it be removed? → IsBanned
│        ├─ Should match local package? → DiffersToLocal
│        ├─ Should match group highest/lowest? → DiffersToHighestOrLowestSemver
│        ├─ Should match pinned version? → DiffersToPinnedVersion
│        ├─ Should match snap target? → DiffersToSnapTarget
│        └─ Other fixable states...
│
└─ NO → Continue to Step 3
```

### Step 3: Is it a user error or our limitation?

```
Is this a misconfiguration or ambiguous situation?
├─ Misconfiguration → Use Suspect variant
│                     ├─ Trying to ban local? → RefuseToBanLocal
│                     ├─ Trying to pin local? → RefuseToPinLocal
│                     └─ Other suspect states...
│
├─ Semver conflict → Use Invalid::Conflict variant
│                    └─ Group + range conflict → MatchConflictsWithHighestOrLowestSemver
│
└─ Can't determine → Use Invalid::Unfixable variant
                     ├─ Non-semver mismatch? → NonSemverMismatch
                     ├─ Depends on invalid local? → DependsOnInvalidLocalPackage
                     └─ Other unfixable states...
```

**Quick Reference:**

- **Valid** = Everything is correct ✓
- **Fixable** = We know what it should be, can auto-fix ✓→
- **Conflict** = Conflicting rules, need user decision ⚠️
- **Unfixable** = Ambiguous, can't determine correct value ⚠️
- **Suspect** = User misconfigured something ⚠️

---

## What VersionGroupVariant Does My Feature Need?

```
What versioning behavior do you want?

Should these dependencies be forbidden?
├─ YES → Banned
│        Use case: Deprecated packages, security issues
│        Example: Ban all usage of "request" package
│
└─ NO → Continue...

Should all instances use the same exact version?
├─ YES → Do you want to specify the version?
│        ├─ YES → Pinned
│        │        Use case: Lock to specific version
│        │        Example: All packages use React 18.2.0
│        │
│        └─ NO → Should it follow another package's version?
│                 ├─ YES → SnappedTo
│                 │        Use case: Internal packages follow root version
│                 │        Example: All @my-org/* follow root package.json
│                 │
│                 └─ NO → SameRange
│                          Use case: All ranges must satisfy each other
│                          Example: ">=1.0.0" and "<=2.0.0" are compatible
│
└─ NO → Should they use compatible versions?
         ├─ Same minor? → SameMinor
         │               Use case: Allow patch differences
         │               Example: 1.2.3, 1.2.4, 1.2.5 all allowed
         │
         ├─ Highest found? → HighestSemver
         │                   Use case: Stay up to date
         │                   Example: If you find 1.2.3 and 1.2.5, all use 1.2.5
         │
         ├─ Lowest found? → LowestSemver
         │                  Use case: Conservative, avoid breaking changes
         │                  Example: If you find 1.2.3 and 1.2.5, all use 1.2.3
         │
         └─ Skip checks? → Ignored
                           Use case: Intentionally different versions
                           Example: Testing different versions across packages
```

---

## Where Should I Add My Test?

```
What are you testing?

Testing a new command?
├─ YES → Add test in src/commands/{command}_test.rs
│        Or create if it doesn't exist
│
└─ NO → Continue...

Testing dependency version validation logic?
├─ YES → Which aspect?
│        ├─ Banned dependencies → src/visit_packages/banned_test.rs
│        ├─ Pinned versions → src/visit_packages/pinned_test.rs
│        ├─ Semver groups → src/visit_packages/semver_group_test.rs
│        ├─ Local packages → src/visit_packages/local_test.rs
│        ├─ Workspace protocol → src/visit_packages/workspace_test.rs
│        └─ New feature → Create new file in src/visit_packages/
│
└─ NO → Testing formatting logic?
         ├─ YES → src/visit_formatting/format_test.rs
         │
         └─ NO → Testing a utility function?
                  └─ Add test in same file as source
                     Example: src/specifier.rs → src/specifier_test.rs
```

**Preferred Test Types (in order):**

1. Integration tests in `src/visit_packages/*_test.rs` (best - test full pipeline)
2. Integration tests in `src/visit_formatting/*_test.rs` (good - test full pipeline)
3. Unit tests co-located with source `*_test.rs` (okay - test isolated functions)

---

## Should I Create a New Command?

```
Is this a new user-facing operation?
├─ YES → Does it fit an existing command?
│        ├─ NO → Create new command
│        │       Steps:
│        │       1. Create src/commands/my_command.rs
│        │       2. Add to Subcommand enum in src/cli.rs
│        │       3. Add match arm in src/main.rs
│        │       4. Choose visitor (visit_packages or visit_formatting)
│        │
│        └─ YES → Extend existing command
│                 Add CLI flag or modify behavior
│
└─ NO → Is this a new way to display data?
         ├─ YES → Add to src/commands/ui/
         │
         └─ NO → This is probably internal logic
                  Add to appropriate module (context, visit_packages, etc.)
```

**Command Checklist:**

- [ ] Created `src/commands/my_command.rs`
- [ ] Implemented `pub fn run(ctx: Context) -> i32`
- [ ] Added variant to `Subcommand` enum
- [ ] Added match arm in `main.rs`
- [ ] Decided on visitor (visit_packages or visit_formatting)
- [ ] Added CLI options (if needed)
- [ ] Written tests
- [ ] Updated documentation

---

## How Should I Handle This Error?

```
What kind of error is this?

User input error (bad config, invalid arguments)?
├─ YES → Return Result with descriptive error
│        Use RcfileError or custom error type
│        Example: Invalid config file format
│
└─ NO → Continue...

Recoverable operation (file read, network request)?
├─ YES → Return Result<T, E>
│        Log at appropriate level
│        Let caller decide how to handle
│
└─ NO → Continue...

Programming error (should never happen)?
├─ YES → Use panic! or unwrap()
│        Example: Enum match that should be exhaustive
│
└─ NO → State that should be flagged for user?
         └─ Use InstanceState::Suspect or Invalid
            Don't throw error, mark instance for reporting
```

**Error Logging Levels:**

- `error!()` → User-facing issues (config errors, missing files)
- `warn!()` → Recoverable issues (deprecated config, fallbacks)
- `info!()` → User-relevant information (found N packages)
- `debug!()` → Internal state (for debugging, verbose mode)

---

## Should I Use Rc or Arc?

```
Will this data be shared across threads?
├─ YES → Use Arc<T>
│        Example: RegistryClient (used in async tasks)
│
└─ NO → Use Rc<T>
         Example: Instance (single-threaded iteration)
```

**Quick Reference:**

- `Rc<T>` → Reference counting, single-threaded (faster, less overhead)
- `Arc<T>` → Atomic reference counting, thread-safe (slower, more overhead)
- `&T` → Borrow, when you don't need ownership
- `Box<T>` → Heap allocation, single owner

**In Syncpack:**

- `Rc<Instance>` → Instances are shared but never cross threads
- `Arc<dyn RegistryClient>` → Registry client used in async/await (crosses threads)
- `&Context` → Most functions just borrow Context
- Owned `Context` → Commands take ownership at the end

---

## Additional Decision Points

### Should I Add a New Config Option?

```
Is this a user-facing setting?
├─ YES → Is it global or per-group?
│        ├─ Global → Add to Rcfile struct
│        │          Add to config schema
│        │
│        └─ Per-group → Add to version group config
│                       Update VersionGroup struct
│
└─ NO → Is it a CLI flag?
         ├─ YES → Add to Cli struct in src/cli.rs
         │
         └─ NO → This should be a constant or internal logic
```

### Should I Modify the Context Struct?

```
Do all commands need this data?
├─ YES → Add to Context
│        Example: config, packages, instances
│
└─ NO → Is it specific to one command?
         ├─ YES → Pass as function parameter
         │        Or compute in the command
         │
         └─ NO → Is it temporary computation?
                  └─ Local variable in function
```

### Should I Add a New Specifier Variant?

```
Is this a fundamentally new type of version specifier?
├─ YES → Add to Specifier enum
│        Update parser logic
│        Add tests
│        Example: Catalog protocol was added this way
│
└─ NO → Does it fit in an existing variant?
         ├─ YES → Extend parsing logic for that variant
         │
         └─ NO → Use Unsupported variant
                  Let Syncpack skip it
```

---

## Summary: Most Common Decisions

### Daily Development

1. **Adding validation logic** → New InstanceState variant + logic in visit_packages
2. **Adding tests** → Use TestBuilder in `src/visit_packages/*_test.rs`
3. **Fixing bugs** → Add test first, then fix
4. **Adding features** → May need new InstanceState, rarely new command

### Architecture Changes

1. **New command** → Follow checklist above
2. **New config option** → Rcfile or Cli struct + merge logic
3. **New version group type** → New VersionGroupVariant + validation logic
4. **Performance optimisation** → Run benchmarks before and after

### When Stuck

1. Find similar existing code (grep is your friend)
2. Check test files for usage examples
3. Read .notes/index.md for patterns
4. Ask: "Does this fit the 3-phase pattern?"

---

## Pattern Recognition

If you're trying to:

- **Change what's considered valid/invalid** → Modify InstanceState logic in visit_packages
- **Change how output looks** → Modify UI modules in src/commands/ui/
- **Change what files are read** → Modify packages.rs or rcfile.rs
- **Change how versions are parsed** → Modify `specifier/*` modules
- **Add a rule about version numbers** → New InstanceState variant
- **Add a new versioning policy** → New VersionGroupVariant
- **Test a new scenario** → TestBuilder in visit_packages tests

Remember: Most features are new InstanceState variants or modifications to visit_packages logic, not new commands!
