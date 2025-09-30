# Visual Diagrams for Syncpack

This document provides visual representations of Syncpack's architecture and data flow using Mermaid diagrams.

## Data Flow Through the Pipeline

```mermaid
graph TB
    CLI[CLI Arguments] --> ParseCLI[Parse CLI]
    ParseCLI --> Config[Locate Config File]
    Config --> ReadConfig[Read Config<br/>TypeScript/JS/YAML/JSON]
    ReadConfig --> FindPackages[Find package.json Files<br/>Using Globs]
    FindPackages --> ReadPackages[Read All package.json Files]
    ReadPackages --> CollectDeps[Collect Dependencies<br/>Create Instances]
    CollectDeps --> AssignGroups[Assign to Version Groups]
    AssignGroups --> Context[Context Created<br/>State: Unknown]

    Context --> ChooseVisitor{Which Visitor?}
    ChooseVisitor -->|Dependency Versions| VisitPackages[visit_packages]
    ChooseVisitor -->|File Formatting| VisitFormatting[visit_formatting]

    VisitPackages --> TagStates[Tag Each Instance<br/>with InstanceState]
    VisitFormatting --> TagFormat[Tag Format Issues]

    TagStates --> ContextTagged[Context Tagged<br/>States Assigned]
    TagFormat --> ContextTagged

    ContextTagged --> Command{Which Command?}
    Command -->|lint| Lint[Report Issues]
    Command -->|fix| Fix[Auto-Fix Issues]
    Command -->|format| Format[Format Files]
    Command -->|update| Update[Update from Registry]
    Command -->|list| List[List Dependencies]
    Command -->|json| JSON[JSON Output]

    Lint --> Exit[Exit Code 0 or 1]
    Fix --> Exit
    Format --> Exit
    Update --> Exit
    List --> Exit
    JSON --> Exit

    style Context fill:#e1f5ff
    style ContextTagged fill:#fff4e1
    style Exit fill:#e8f5e9
```

## InstanceState State Machine

```mermaid
stateDiagram-v2
    [*] --> Unknown: Instance Created

    Unknown --> Valid: Passes All Rules
    Unknown --> Invalid: Breaks Rules
    Unknown --> Suspect: Misconfiguration

    state Invalid {
        [*] --> Fixable: Can Auto-Fix
        [*] --> Unfixable: Ambiguous
        [*] --> Conflict: Conflicting Rules
    }

    Valid --> [*]: Command Processes
    Invalid --> [*]: Command Processes
    Suspect --> [*]: Command Processes

    note right of Valid
        14 variants:
        - IsLocalAndValid
        - IsPinned
        - IsHighestOrLowestSemver
        - etc.
    end note

    note right of Fixable
        8 variants:
        - IsBanned
        - DiffersToLocal
        - DiffersToPinnedVersion
        - etc.
    end note

    note right of Unfixable
        3 variants:
        - NonSemverMismatch
        - DependsOnInvalidLocalPackage
        - etc.
    end note
```

## Context Ownership Flow

```mermaid
graph LR
    Create[Context::create] -->|Owns Context| Visit
    Visit[visit_packages] -->|Returns Context| Command
    Command[lint::run] -->|Consumes Context| Exit[Exit]

    style Create fill:#e3f2fd
    style Visit fill:#fff3e0
    style Command fill:#f3e5f5
    style Exit fill:#e8f5e9

    note1[Phase 1: Create]
    note2[Phase 2: Inspect]
    note3[Phase 3: Run]

    Create -.-> note1
    Visit -.-> note2
    Command -.-> note3
```

## Version Group Decision Tree

```mermaid
graph TD
    Start{What Version<br/>Behavior?}

    Start -->|Should be forbidden| Banned[Banned]
    Start -->|Should match exactly| Exact{Specify Version?}
    Start -->|Should be compatible| Compatible{Which Policy?}

    Exact -->|Yes, lock to version| Pinned[Pinned]
    Exact -->|No, follow another pkg| SnappedTo[SnappedTo]

    Compatible -->|Ranges must satisfy each other| SameRange[SameRange]

    Compatible -->|Allow patch differences| SameMinor[SameMinor]
    Compatible -->|Use highest found| HighestSemver[HighestSemver]
    Compatible -->|Use lowest found| LowestSemver[LowestSemver]
    Compatible -->|Don't check| Ignored[Ignored]

    style Banned fill:#ffebee
    style Pinned fill:#e8f5e9
    style SnappedTo fill:#e8f5e9
    style SameRange fill:#e8f5e9
    style SameMinor fill:#fff3e0
    style HighestSemver fill:#e3f2fd
    style LowestSemver fill:#e3f2fd
    style Ignored fill:#f5f5f5
```

## Test Builder Pattern

```mermaid
graph LR
    New[TestBuilder::new] -->|with_packages| Packages
    Packages -->|with_version_group| Groups
    Groups -->|with_config| Config
    Config -->|build_and_visit_packages| Context[Context<br/>States Assigned]

    Context --> Expect[expect &ctx]
    Expect --> Assert[to_have_instances]
    Assert --> Pass{Test Pass?}

    Pass -->|Yes| Success[✓]
    Pass -->|No| Failure[✗]

    style New fill:#e3f2fd
    style Context fill:#fff4e1
    style Success fill:#e8f5e9
    style Failure fill:#ffebee
```

## Visitor Pattern Flow

```mermaid
graph TD
    Visit[visit_packages Context] --> IterGroups[Iterate Version Groups]

    IterGroups --> Group1[Group 1: Banned]
    IterGroups --> Group2[Group 2: Pinned]
    IterGroups --> Group3[Group 3: HighestSemver]
    IterGroups --> GroupN[...]

    Group1 --> Banned[banned::visit]
    Group2 --> Pinned[pinned::visit]
    Group3 --> Preferred[preferred_semver::visit]

    Banned --> Tag1[Tag Instances<br/>IsBanned]
    Pinned --> Tag2[Tag Instances<br/>IsPinned/DiffersToPinned]
    Preferred --> Tag3[Tag Instances<br/>IsHighestSemver/Differs...]

    Tag1 --> Return[Return Context]
    Tag2 --> Return
    Tag3 --> Return

    style Visit fill:#e3f2fd
    style Return fill:#e8f5e9
```

## Command Iteration Pattern

```mermaid
graph TD
    Command[Command: lint] --> GetGroups[ctx.get_version_groups]
    GetGroups --> ForEachGroup[for_each group]

    ForEachGroup --> GetDeps[group.get_sorted_dependencies]
    GetDeps --> ForEachDep[for_each dependency]

    ForEachDep --> GetInst[dependency.get_sorted_instances]
    GetInst --> Filter[filter instance.is_invalid]

    Filter --> ForEachInst[for_each instance]
    ForEachInst --> Process[Process Instance<br/>Print/Fix/Report]

    Process --> More{More Instances?}
    More -->|Yes| ForEachInst
    More -->|No| Exit[Return Exit Code]

    style Command fill:#e3f2fd
    style Process fill:#fff3e0
    style Exit fill:#e8f5e9
```

## Specifier Type Hierarchy

```mermaid
graph TD
    Specifier[Specifier Enum]

    Specifier --> BasicSemver[BasicSemver<br/>1.2.3, ^1.2.3, ~1.2.3]
    Specifier --> ComplexSemver[ComplexSemver<br/>>=1.0.0 <2.0.0]
    Specifier --> WorkspaceProto[WorkspaceProtocol<br/>workspace:*, workspace:^]
    Specifier --> Git[Git<br/>git://github.com/...]
    Specifier --> File[File<br/>file:../path]
    Specifier --> URL[Url<br/>http://example.com/...]
    Specifier --> Alias[Alias<br/>npm:pkg@version]
    Specifier --> Tag[Tag<br/>latest, next, beta]
    Specifier --> None[None<br/>missing]
    Specifier --> Unsupported[Unsupported<br/>unrecognized]

    BasicSemver --> Latest[Latest: *]
    BasicSemver --> Major[Major: 1]
    BasicSemver --> Minor[Minor: 1.2]
    BasicSemver --> Patch[Patch: 1.2.3]

    style BasicSemver fill:#e8f5e9
    style ComplexSemver fill:#fff3e0
    style WorkspaceProto fill:#e3f2fd
    style Unsupported fill:#ffebee
```

## File Organization

```mermaid
graph TD
    Root[src/]

    Root --> Core[Core Files]
    Root --> Commands[commands/]
    Root --> VisitPkg[visit_packages/]
    Root --> VisitFmt[visit_formatting/]
    Root --> Test[test/]
    Root --> Other[Other Modules]

    Core --> Main[main.rs<br/>Entry point]
    Core --> CLI[cli.rs<br/>Arg parsing]
    Core --> Context[context.rs<br/>Main struct]
    Core --> Instance[instance.rs<br/>Dependency occurrence]
    Core --> State[instance_state.rs<br/>State machine]

    Commands --> Lint[lint.rs]
    Commands --> Fix[fix.rs]
    Commands --> Format[format.rs]
    Commands --> Update[update.rs]

    VisitPkg --> Banned[banned.rs<br/>banned_test.rs]
    VisitPkg --> Pinned[pinned.rs<br/>pinned_test.rs]
    VisitPkg --> Semver[preferred_semver.rs]

    Test --> Builder[builder.rs<br/>TestBuilder]
    Test --> Expect[expect.rs<br/>Assertions]
    Test --> Mock[mock.rs<br/>Utilities]

    style Main fill:#e3f2fd
    style Context fill:#e3f2fd
    style State fill:#e3f2fd
    style Builder fill:#fff3e0
```

## Adding a New Command - Flow

```mermaid
graph TD
    Start[Want to Add Command] --> Enum[1. Add to Subcommand enum<br/>src/cli.rs]
    Enum --> Create[2. Create src/commands/new_cmd.rs<br/>pub fn run ctx: Context -> i32]
    Create --> Module[3. Register in src/commands.rs<br/>pub mod new_cmd]
    Module --> Dispatch[4. Add match arm in src/main.rs<br/>Subcommand::NewCmd => ...]
    Dispatch --> ChooseVis{5. Choose Visitor}

    ChooseVis -->|Dependency Versions| VP[Call visit_packages ctx]
    ChooseVis -->|File Formatting| VF[Call visit_formatting ctx]

    VP --> Pass[6. Pass Context to command]
    VF --> Pass

    Pass --> Test[7. Write tests<br/>Use TestBuilder]
    Test --> Run[8. Test locally<br/>cargo run -- new-cmd]
    Run --> Done[✓ Complete]

    style Start fill:#e3f2fd
    style Done fill:#e8f5e9
```

## Version Groups - First Match Wins

```mermaid
graph TD
    Instance[Instance: react@17.0.0<br/>in package-a] --> VG1{Version Group 1<br/>dependencies: react<br/>pinned: 18.0.0}

    VG1 -->|Matches!| Assign1[Assigned to Group 1<br/>STOP searching]
    VG1 -->|No match| VG2{Version Group 2<br/>dependencies: **<br/>policy: highestSemver}

    VG2 -->|Matches!| Assign2[Assigned to Group 2<br/>STOP searching]
    VG2 -->|No match| VGN{...more groups...}

    VGN -->|Matches!| AssignN[Assigned to Group N<br/>STOP searching]
    VGN -->|No match| Default[Not assigned<br/>Uses default]

    Assign1 --> Process[Process with Group 1 rules]

    style Instance fill:#e3f2fd
    style Assign1 fill:#e8f5e9
    style Process fill:#fff3e0
```

## Context Fields and Relationships

```mermaid
classDiagram
    class Context {
        +Config config
        +Packages packages
        +Vec~Rc~Instance~~ instances
        +Vec~VersionGroup~ version_groups
        +Option~RegistryClient~ registry_client
        +create() Context
        +get_version_groups()
    }

    class Config {
        +Cli cli
        +Rcfile rcfile
    }

    class Packages {
        +Vec~PackageJson~ all
    }

    class Instance {
        +String dependency_name
        +Specifier specifier
        +RefCell~InstanceState~ state
        +is_valid() bool
        +is_invalid() bool
        +is_fixable() bool
    }

    class VersionGroup {
        +VersionGroupVariant variant
        +Vec~Dependency~ dependencies
        +get_sorted_dependencies()
    }

    class Dependency {
        +String name
        +Vec~Rc~Instance~~ instances
        +get_sorted_instances()
    }

    Context --> Config
    Context --> Packages
    Context --> Instance
    Context --> VersionGroup
    VersionGroup --> Dependency
    Dependency --> Instance
```

## Understanding These Diagrams

Each diagram serves a specific purpose:

1. **Data Flow** - Shows the complete pipeline from CLI to exit
2. **State Machine** - Shows how instances transition between states
3. **Ownership Flow** - Shows how Context moves through phases
4. **Version Group Decision** - Helps choose the right policy
5. **Test Builder** - Shows the testing workflow
6. **Visitor Pattern** - Shows how validation is organized
7. **Command Iteration** - Shows the standard loop pattern
8. **Specifier Hierarchy** - Shows all version format types
9. **File Organization** - Shows where code lives
10. **Adding Command** - Step-by-step guide
11. **First Match Wins** - Shows how instances are assigned
12. **Context Relationships** - Shows the data structure

## Viewing These Diagrams

These Mermaid diagrams can be viewed in:

- GitHub (renders automatically)
- VS Code with Mermaid extension
- Any Markdown viewer with Mermaid support
- Online at https://mermaid.live/

Copy any diagram into mermaid.live to see it rendered and experiment with modifications.
