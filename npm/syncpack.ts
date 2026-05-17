export interface RcFile {
  /** @see https://syncpack.dev/config/syncpackrc/#json */
  $schema?: string;
  /** @see https://syncpack.dev/config/custom-types */
  customTypes?: {
    [name: string]: CustomType.Any;
  };
  /** @see https://syncpack.dev/config/dependency-groups */
  dependencyGroups?: DependencyGroup[];
  /** @see https://syncpack.dev/config/format-bugs */
  formatBugs?: boolean;
  /** @see https://syncpack.dev/config/format-repository */
  formatRepository?: boolean;
  /** @see https://syncpack.dev/config/indent */
  indent?: string;
  /** @see https://syncpack.dev/config/max-concurrent-requests */
  maxConcurrentRequests?: number;
  /**
   * Skip dependency updates published less than this many minutes ago.
   * `0` disables the filter. When omitted, the value from the project's
   * `pnpm-workspace.yaml` is used; if neither is set, defaults to `1440`
   * (one day). Setting it here always overrides the pnpm value.
   * @see https://pnpm.io/settings#minimumreleaseage
   */
  minimumReleaseAge?: number;
  /** @see https://syncpack.dev/semver-groups */
  semverGroups?: SemverGroup.Any[];
  /** @see https://syncpack.dev/update-groups */
  updateGroups?: UpdateGroup.Any[];
  /** @see https://syncpack.dev/config/sort-az */
  sortAz?: string[];
  /** @see https://syncpack.dev/config/sort-exports */
  sortExports?: string[];
  /** @see https://syncpack.dev/config/sort-first */
  sortFirst?: string[];
  /** @see https://syncpack.dev/config/sort-packages */
  sortPackages?: boolean;
  /** @see https://syncpack.dev/config/source */
  source?: string[];
  /** @see https://syncpack.dev/config/source-mode */
  sourceMode?: 'replace' | 'extend';
  /** @see https://syncpack.dev/config/strict */
  strict?: boolean;
  /** @see https://syncpack.dev/version-groups */
  versionGroups?: VersionGroup.Any[];

  /** @deprecated */
  dependencyTypes?: never;
  /** @deprecated */
  filter?: never;
  /** @deprecated */
  lintFormatting?: never;
  /** @deprecated */
  lintSemverRanges?: never;
  /** @deprecated */
  lintVersions?: never;
  /** @deprecated */
  specifierTypes?: never;
}

export interface GroupSelector {
  /** @see https://syncpack.dev/version-groups/highest-semver/#dependencies */
  dependencies?: string[];
  /** @see https://syncpack.dev/version-groups/highest-semver/#dependencytypes */
  dependencyTypes?: DependencyType[];
  /** @see https://syncpack.dev/version-groups/highest-semver/#label */
  label?: string;
  /** @see https://syncpack.dev/version-groups/highest-semver/#packages */
  packages?: string[];
  /** @see https://syncpack.dev/version-groups/highest-semver/#specifiertypes */
  specifierTypes?: SpecifierType[];
}

export interface DependencyGroup {
  /** @see https://syncpack.dev/config/dependency-groups/#aliasname */
  aliasName: string;
  /** @see https://syncpack.dev/config/dependency-groups/#dependencies */
  dependencies?: string[];
  /** @see https://syncpack.dev/config/dependency-groups/#dependencytypes */
  dependencyTypes?: DependencyType[];
  /** @see https://syncpack.dev/config/dependency-groups/#packages */
  packages?: string[];
  /** @see https://syncpack.dev/config/dependency-groups/#specifiertypes */
  specifierTypes?: SpecifierType[];
}

namespace SemverGroup {
  export interface Ignored extends GroupSelector {
    /** @see https://syncpack.dev/semver-groups/ignored/#isignored */
    isIgnored: true;
  }
  export interface WithRange extends GroupSelector {
    /** @see https://syncpack.dev/semver-groups/with-range/#range */
    range: SemverRange;
  }
  export type Any = Ignored | WithRange;
}

namespace UpdateGroup {
  export interface Ignored extends GroupSelector {
    /** @see https://syncpack.dev/update-groups/ignored/#isignored */
    isIgnored: true;
  }
  export interface Targeted extends GroupSelector {
    /** @see https://syncpack.dev/update-groups/targeted/#target */
    target: 'patch' | 'minor' | 'latest';
  }
  export type Any = Ignored | Targeted;
}

namespace VersionGroup {
  export interface Banned extends GroupSelector {
    /** @see https://syncpack.dev/version-groups/banned/#isbanned */
    isBanned: true;
    /** @see https://syncpack.dev/version-groups/banned/#severity */
    severity?: { IsBanned?: Severity };
  }
  export interface Ignored extends GroupSelector {
    /** @see https://syncpack.dev/version-groups/ignored/#isignored */
    isIgnored: true;
    // `severity` on an Ignored group is accepted and silently discarded —
    // the group emits no statuses to tune. Omitted from the type so writing it
    // surfaces as a compile-time hint that it does nothing.
  }
  export interface Pinned extends GroupSelector {
    /** @see https://syncpack.dev/version-groups/pinned/#pinversion */
    pinVersion: string;
    /** @see https://syncpack.dev/version-groups/pinned/#severity */
    severity?: {
      DiffersToPin?: Severity;
      PinOverridesSemverRange?: Severity;
      PinOverridesSemverRangeMismatch?: Severity;
      RefuseToPinLocal?: Severity;
    };
  }
  export interface SnappedTo extends GroupSelector {
    /** @see https://syncpack.dev/version-groups/snapped-to/#snapto */
    snapTo: string[];
    /** @see https://syncpack.dev/version-groups/snapped-to/#severity */
    severity?: {
      DiffersToSnapTarget?: Severity;
      SemverRangeMismatch?: Severity;
      RefuseToSnapLocal?: Severity;
    };
  }
  export interface SameRange extends GroupSelector {
    /** @see https://syncpack.dev/version-groups/same-range/#policy */
    policy: 'sameRange';
    /** @see https://syncpack.dev/version-groups/same-range/#severity */
    severity?: { SemverRangeMismatch?: Severity };
  }
  export interface SameMinor extends GroupSelector {
    /** @see https://syncpack.dev/version-groups/same-minor/#policy */
    policy: 'sameMinor';
    /** @see https://syncpack.dev/version-groups/same-minor/#severity */
    severity?: {
      DiffersToHighestOrLowestSemverMinor?: Severity;
      SemverRangeMismatch?: Severity;
      SameMinorOverridesSemverRange?: Severity;
      SameMinorOverridesSemverRangeMismatch?: Severity;
    };
  }
  export interface Standard extends GroupSelector {
    /** @see https://syncpack.dev/version-groups/lowest-semver/#preferversion */
    preferVersion?: 'highestSemver' | 'lowestSemver';
    /** @see https://syncpack.dev/version-groups/highest-semver/#severity */
    severity?: {
      SemverRangeMismatch?: Severity;
      DiffersToLocal?: Severity;
      DiffersToCatalog?: Severity;
      DiffersToHighestOrLowestSemver?: Severity;
    };
  }
  export interface Catalog extends GroupSelector {
    /** @see https://syncpack.dev/version-groups/catalog/#policy */
    policy: 'catalog';
    /** @see https://syncpack.dev/version-groups/catalog/#severity */
    severity?: {
      NotUsingCatalog?: Severity;
      MissingFromCatalog?: Severity;
    };
  }
  export type Any = Banned | Catalog | Ignored | Pinned | SameRange | SameMinor | SnappedTo | Standard;
}

namespace CustomType {
  export interface NameAndVersionProps {
    /** @see https://syncpack.dev/config/custom-types/#namepath */
    namePath: string;
    /** @see https://syncpack.dev/config/custom-types/#name */
    path: string;
    /** @see https://syncpack.dev/config/custom-types/#namestrategy */
    strategy: 'name~version';
  }
  export interface NamedVersionString {
    /** @see https://syncpack.dev/config/custom-types/#name */
    path: string;
    /** @see https://syncpack.dev/config/custom-types/#namestrategy */
    strategy: 'name@version';
  }
  export interface UnnamedVersionString {
    /** @see https://syncpack.dev/config/custom-types/#name */
    path: string;
    /** @see https://syncpack.dev/config/custom-types/#namestrategy */
    strategy: 'version';
  }
  export interface VersionsByName {
    /** @see https://syncpack.dev/config/custom-types/#name */
    path: string;
    /** @see https://syncpack.dev/config/custom-types/#namestrategy */
    strategy: 'versionsByName';
  }
  export type Any = NameAndVersionProps | NamedVersionString | UnnamedVersionString | VersionsByName;
}

type SemverRange = '' | '*' | '>' | '>=' | '.x' | '<' | '<=' | '^' | '~';

/** @see https://syncpack.dev/severity/ */
export type Severity = 'fix' | 'warn' | 'error';

/** Severity values that appear in JSON output. `'none'` is emitted for Valid
 * instances in `syncpack json` and is not writable in rcfile severity maps. */
export type JsonSeverity = Severity | 'none';

type DependencyType = 'dev' | 'local' | 'overrides' | 'peer' | 'pnpmOverrides' | 'prod' | 'resolutions' | AnyString;

type SpecifierType =
  | 'alias'
  | 'exact'
  | 'file'
  | 'git'
  | 'latest'
  | 'major'
  | 'minor'
  | 'missing'
  | 'range'
  | 'range-complex'
  | 'range-major'
  | 'range-minor'
  | 'tag'
  | 'unsupported'
  | 'url'
  | 'workspace-protocol'
  | AnyString;

type AnyString = string & {};

/** Each Instance printed by `syncpack json` and `syncpack fix --reporter json` */
export type JsonOutput = {
  dependency: string;
  dependencyGroup: string;
  dependencyType: DependencyType;
  package: string;
  property: string[];
  strategy: CustomType.Any['strategy'];
  versionGroup: VersionGroupVariant;
  preferredSemverRange: SemverRange | null;
  statusCode: StatusCode;
  statusType: StatusType;
  /** @see https://syncpack.dev/config/severity/ */
  severity: JsonSeverity;
  actual: {
    raw: string;
    type: SpecifierType;
  };
  expected: {
    raw: string;
    type: SpecifierType;
  } | null;
};

export type StatusType = 'Valid' | 'Fixable' | 'Unfixable' | 'Suspect' | 'Conflict';

/** Each formatting mismatch printed by `syncpack format --reporter json` */
export type FormatJsonOutput = {
  package: string;
  filePath: string;
  property: string[];
  statusCode: FormatStatusCode;
};

export type FormatStatusCode =
  | 'BugsPropertyIsNotFormatted'
  | 'RepositoryPropertyIsNotFormatted'
  | 'PropertyIsNotSortedAz'
  | 'PackagePropertiesAreNotSorted'
  | 'ExportsPropertyIsNotSorted';

export type VersionGroupVariant =
  | 'Banned'
  | 'Catalog'
  | 'HighestSemver'
  | 'Ignored'
  | 'LowestSemver'
  | 'Pinned'
  | 'SameRange'
  | 'SameMinor'
  | 'SnappedTo';

export type StatusCode =
  | 'IsHighestOrLowestSemver'
  | 'IsIdenticalToLocal'
  | 'IsIdenticalToPin'
  | 'IsIdenticalToSnapTarget'
  | 'IsIgnored'
  | 'IsLocalAndValid'
  | 'IsNonSemverButIdentical'
  | 'SatisfiesHighestOrLowestSemver'
  | 'SatisfiesLocal'
  | 'SatisfiesSameRangeGroup'
  | 'SatisfiesSameMinorGroup'
  | 'SatisfiesSnapTarget'
  | 'DiffersToCatalog'
  | 'DiffersToHighestOrLowestSemver'
  | 'DiffersToHighestOrLowestSemverMinor'
  | 'DiffersToLocal'
  | 'DiffersToNpmRegistry'
  | 'DiffersToPin'
  | 'DiffersToSnapTarget'
  | 'IsBanned'
  | 'MissingFromCatalog'
  | 'NotUsingCatalog'
  | 'PinOverridesSemverRange'
  | 'PinOverridesSemverRangeMismatch'
  | 'SameMinorOverridesSemverRange'
  | 'SameMinorOverridesSemverRangeMismatch'
  | 'SemverRangeMismatch'
  | 'DependsOnInvalidLocalPackage'
  | 'NonSemverMismatch'
  | 'SameRangeMismatch'
  | 'SameMinorMismatch'
  | 'DependsOnMissingSnapTarget'
  | 'InvalidLocalVersion'
  | 'RefuseToBanLocal'
  | 'RefuseToPinLocal'
  | 'RefuseToSnapLocal'
  | 'MatchConflictsWithHighestOrLowestSemver'
  | 'MatchConflictsWithLocal'
  | 'MatchConflictsWithSnapTarget'
  | 'MismatchConflictsWithHighestOrLowestSemver'
  | 'MismatchConflictsWithLocal'
  | 'MismatchConflictsWithSnapTarget';
