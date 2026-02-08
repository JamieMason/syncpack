# preferred_semver.rs Decision Tree

```mermaid
flowchart TD
    Start["visit(dependency)"] --> InvalidLocal{"has invalid<br/>local instance?"}

    %% ── Invalid local ──
    InvalidLocal -->|Y| ForEachInvalid["for each instance"]
    ForEachInvalid --> IsLocalInvalid{"is_local?"}
    IsLocalInvalid -->|Y| S_InvalidLocalVersion["⚠️ InvalidLocalVersion"]
    IsLocalInvalid -->|N| E_DependsOnInvalid["❌ DependsOnInvalidLocalPackage"]

    %% ── Valid local ──
    InvalidLocal -->|N| HasLocal{"has local<br/>instance?"}
    HasLocal -->|Y| ForEachLocal["for each instance"]
    ForEachLocal --> IsLocalValid{"is_local?"}
    IsLocalValid -->|Y| V_IsLocalAndValid["✅ IsLocalAndValid"]
    IsLocalValid -->|N| IsLink{"is link<br/>specifier?"}

    IsLink -->|Y| LinkResolves{"link resolves to<br/>local package?"}
    LinkResolves -->|Y| V_SatisfiesLocal_Link["✅ SatisfiesLocal"]
    LinkResolves -->|N| F_DiffersToLocal_Link["🔧 DiffersToLocal"]

    IsLink -->|N| IsWorkspace{"is workspace<br/>protocol?"}
    IsWorkspace -->|Y| StrictMode{"strict mode?"}
    StrictMode -->|N| V_SatisfiesLocal_WS["✅ SatisfiesLocal"]
    StrictMode -->|Y| LocalVersionCheck
    IsWorkspace -->|N| LocalVersionCheck

    LocalVersionCheck{"same version<br/>as local?"}
    LocalVersionCheck -->|N| F_DiffersToLocal["🔧 DiffersToLocal"]
    LocalVersionCheck -->|Y| LocalSemverGroup{"semver group prefers<br/>range ≠ Exact?"}

    LocalSemverGroup -->|N| LocalAlreadyEquals{"already equals<br/>local?"}
    LocalAlreadyEquals -->|Y| V_IsIdenticalToLocal["✅ IsIdenticalToLocal"]
    LocalAlreadyEquals -->|N| F_DiffersToLocal2["🔧 DiffersToLocal"]

    LocalSemverGroup -->|Y| LocalMatchesPreferred{"actual range =<br/>preferred range?"}
    LocalMatchesPreferred -->|Y| LocalPrefSatisfies{"preferred range<br/>satisfies local?"}
    LocalPrefSatisfies -->|Y| V_SatisfiesLocal_SG["✅ SatisfiesLocal"]
    LocalPrefSatisfies -->|N| C_MatchConflictsLocal["💥 MatchConflictsWithLocal"]

    LocalMatchesPreferred -->|N| LocalPrefSatisfies2{"preferred range<br/>satisfies local?"}
    LocalPrefSatisfies2 -->|Y| F_SemverRangeMismatch_Local["🔧 SemverRangeMismatch"]
    LocalPrefSatisfies2 -->|N| C_MismatchConflictsLocal["💥 MismatchConflictsWithLocal"]

    %% ── Catalog ──
    HasLocal -->|N| HasCatalog{"any instance uses<br/>catalog: protocol?"}
    HasCatalog -->|Y| ForEachCatalog["for each instance"]
    ForEachCatalog --> IsCatalog{"is catalog:?"}
    IsCatalog -->|Y| V_IsCatalog["✅ IsCatalog"]
    IsCatalog -->|N| F_DiffersToCatalog["🔧 DiffersToCatalog"]

    %% ── Registry updates ──
    HasCatalog -->|N| HasUpdates{"eligible registry<br/>updates?"}
    HasUpdates -->|Y| ForEachUpdate["for each instance"]
    ForEachUpdate --> F_DiffersToNpmRegistry["🔧 DiffersToNpmRegistry"]

    %% ── Highest/Lowest semver ──
    HasUpdates -->|N| HasHighest{"has highest/lowest<br/>semver specifier?"}
    HasHighest -->|Y| ForEachHighest["for each instance"]
    ForEachHighest --> SameVersion{"same version<br/>as highest?"}

    SameVersion -->|N| F_DiffersToHighest["🔧 DiffersToHighestOrLowestSemver<br/>(with preferred range applied)"]

    SameVersion -->|Y| HighestSemverGroup{"semver group prefers<br/>range ≠ highest range?"}

    HighestSemverGroup -->|Y| HighestMatchesPref{"actual range =<br/>preferred range?"}

    HighestMatchesPref -->|Y| HighestPrefSatisfies{"preferred range<br/>satisfies highest?"}
    HighestPrefSatisfies -->|Y| V_SatisfiesHighest["✅ SatisfiesHighestOrLowestSemver"]
    HighestPrefSatisfies -->|N| C_MatchConflictsHighest["💥 MatchConflictsWithHighestOrLowestSemver"]

    HighestMatchesPref -->|N| HighestPrefSatisfies2{"preferred range<br/>satisfies highest?"}
    HighestPrefSatisfies2 -->|Y| F_SemverRangeMismatch_Highest["🔧 SemverRangeMismatch"]
    HighestPrefSatisfies2 -->|N| C_MismatchConflictsHighest["💥 MismatchConflictsWithHighestOrLowestSemver"]

    HighestSemverGroup -->|N| HasPrefMismatch{"has preferred range<br/>AND actual ≠ preferred?"}

    HasPrefMismatch -->|Y| PrefSatisfiesHighest3{"preferred range<br/>satisfies highest?"}
    PrefSatisfiesHighest3 -->|Y| F_SemverRangeMismatch_Adj["🔧 SemverRangeMismatch"]
    PrefSatisfiesHighest3 -->|N| C_MismatchConflictsHighest2["💥 MismatchConflictsWithHighestOrLowestSemver"]

    HasPrefMismatch -->|N| HighestAlreadyEquals{"already equals<br/>highest?"}
    HighestAlreadyEquals -->|Y| V_IsHighest["✅ IsHighestOrLowestSemver"]
    HighestAlreadyEquals -->|N| F_DiffersToHighest2["🔧 DiffersToHighestOrLowestSemver"]

    %% ── No semver ──
    HasHighest -->|N| AllIdentical{"every specifier<br/>identical?"}
    AllIdentical -->|Y| V_NonSemverIdentical["✅ IsNonSemverButIdentical"]
    AllIdentical -->|N| E_NonSemverMismatch["❌ NonSemverMismatch"]

    %% ── Styling ──
    classDef valid fill:#d4edda,stroke:#28a745,color:#000
    classDef fixable fill:#fff3cd,stroke:#ffc107,color:#000
    classDef conflict fill:#f8d7da,stroke:#dc3545,color:#000
    classDef suspect fill:#e2e3e5,stroke:#6c757d,color:#000
    classDef unfixable fill:#f8d7da,stroke:#dc3545,color:#000

    class V_IsLocalAndValid,V_SatisfiesLocal_Link,V_SatisfiesLocal_WS,V_IsIdenticalToLocal,V_SatisfiesLocal_SG,V_IsCatalog,V_SatisfiesHighest,V_IsHighest,V_NonSemverIdentical valid
    class F_DiffersToLocal_Link,F_DiffersToLocal,F_DiffersToLocal2,F_SemverRangeMismatch_Local,F_DiffersToCatalog,F_DiffersToNpmRegistry,F_DiffersToHighest,F_SemverRangeMismatch_Highest,F_SemverRangeMismatch_Adj,F_DiffersToHighest2 fixable
    class C_MatchConflictsLocal,C_MismatchConflictsLocal,C_MatchConflictsHighest,C_MismatchConflictsHighest,C_MismatchConflictsHighest2 conflict
    class S_InvalidLocalVersion suspect
    class E_DependsOnInvalid,E_NonSemverMismatch unfixable
```

## Legend

| Icon | Category  | Meaning                                        |
| ---- | --------- | ---------------------------------------------- |
| ✅   | Valid     | No action needed                               |
| 🔧   | Fixable   | Can be auto-fixed                              |
| 💥   | Conflict  | Semver group and version goal are incompatible |
| ⚠️   | Suspect   | Questionable but not fixable                   |
| ❌   | Unfixable | Error that cannot be auto-fixed                |

## Branch Priority

The top-level branches are evaluated in order — first match wins:

1. **Invalid local** — local package has missing/invalid `.version`
2. **Valid local** — dependency is developed in this monorepo
3. **Catalog** — any instance uses `catalog:` protocol
4. **Registry updates** — npm registry has eligible updates
5. **Highest/lowest semver** — compare against highest (or lowest) version
6. **No semver** — none of the above apply
