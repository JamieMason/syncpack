export interface RcFile {
  /** @see https://jamiemason.github.io/syncpack/integrations/json-schema */
  $schema?: string;
  /** @see https://jamiemason.github.io/syncpack/config/custom-types */
  customTypes?: {
    [name: string]: CustomType.Any;
  };
  /** @see https://jamiemason.github.io/syncpack/config/dependency-groups */
  dependencyGroups?: DependencyGroup[];
  /** @see https://jamiemason.github.io/syncpack/config/format-bugs */
  formatBugs?: boolean;
  /** @see https://jamiemason.github.io/syncpack/config/format-repository */
  formatRepository?: boolean;
  /** @see https://jamiemason.github.io/syncpack/config/indent */
  indent?: string;
  /** @see https://jamiemason.github.io/syncpack/config/semver-groups */
  semverGroups?: SemverGroup.Any[];
  /** @see https://jamiemason.github.io/syncpack/config/sort-az */
  sortAz?: string[];
  /** @see https://jamiemason.github.io/syncpack/config/sort-exports */
  sortExports?: string[];
  /** @see https://jamiemason.github.io/syncpack/config/sort-first */
  sortFirst?: string[];
  /** @see https://jamiemason.github.io/syncpack/config/sort-packages */
  sortPackages?: boolean;
  /** @see https://jamiemason.github.io/syncpack/config/source */
  source?: string[];
  /** @see https://jamiemason.github.io/syncpack/config/version-groups */
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
  /** @see https://jamiemason.github.io/syncpack/config/version-groups/standard/#dependencies */
  dependencies?: string[];
  /** @see https://jamiemason.github.io/syncpack/config/version-groups/standard/#dependencytypes */
  dependencyTypes?: DependencyType[];
  /** @see https://jamiemason.github.io/syncpack/config/version-groups/standard/#label */
  label?: string;
  /** @see https://jamiemason.github.io/syncpack/config/version-groups/standard/#packages */
  packages?: string[];
  /** @see https://jamiemason.github.io/syncpack/config/version-groups/standard/#specifiertypes */
  specifierTypes?: SpecifierType[];
}

export interface DependencyGroup {
  /** @see https://jamiemason.github.io/syncpack/config/dependency-groups/#aliasname */
  aliasName: string;
  /** @see https://jamiemason.github.io/syncpack/config/dependency-groups/#dependencies */
  dependencies?: string[];
  /** @see https://jamiemason.github.io/syncpack/config/dependency-groups/#dependencytypes */
  dependencyTypes?: DependencyType[];
  /** @see https://jamiemason.github.io/syncpack/config/dependency-groups/#packages */
  packages?: string[];
  /** @see https://jamiemason.github.io/syncpack/config/dependency-groups/#specifiertypes */
  specifierTypes?: SpecifierType[];
}

namespace SemverGroup {
  export interface Ignored extends GroupSelector {
    /** @see https://jamiemason.github.io/syncpack/config/semver-groups/ignored/#isignored */
    isIgnored: true;
  }
  export interface WithRange extends GroupSelector {
    /** @see https://jamiemason.github.io/syncpack/config/semver-groups/with-range/#range */
    range: SemverRange;
  }
  export type Any = Ignored | WithRange;
}

namespace VersionGroup {
  export interface Banned extends GroupSelector {
    /** @see https://jamiemason.github.io/syncpack/config/version-groups/banned/#isbanned */
    isBanned: true;
  }
  export interface Ignored extends GroupSelector {
    /** @see https://jamiemason.github.io/syncpack/config/version-groups/ignored/#isignored */
    isIgnored: true;
  }
  export interface Pinned extends GroupSelector {
    /** @see https://jamiemason.github.io/syncpack/config/version-groups/pinned/#pinversion */
    pinVersion: string;
  }
  export interface SnappedTo extends GroupSelector {
    /** @see https://jamiemason.github.io/syncpack/config/version-groups/snapped-to/#snapto */
    snapTo: string[];
  }
  export interface SameRange extends GroupSelector {
    /** @see https://jamiemason.github.io/syncpack/config/version-groups/same-range/#policy */
    policy: 'sameRange';
  }
  export interface Standard extends GroupSelector {
    /** @see https://jamiemason.github.io/syncpack/config/version-groups/lowest-version/#preferversion */
    preferVersion?: 'highestSemver' | 'lowestSemver';
  }
  export type Any = Banned | Ignored | Pinned | SameRange | SnappedTo | Standard;
}

namespace CustomType {
  export interface NameAndVersionProps {
    /** @see https://jamiemason.github.io/syncpack/config/custom-types/#namepath */
    namePath: string;
    /** @see https://jamiemason.github.io/syncpack/config/custom-types/#name */
    path: string;
    /** @see https://jamiemason.github.io/syncpack/config/custom-types/#namestrategy */
    strategy: 'name~version';
  }
  export interface NamedVersionString {
    /** @see https://jamiemason.github.io/syncpack/config/custom-types/#name */
    path: string;
    /** @see https://jamiemason.github.io/syncpack/config/custom-types/#namestrategy */
    strategy: 'name@version';
  }
  export interface UnnamedVersionString {
    /** @see https://jamiemason.github.io/syncpack/config/custom-types/#name */
    path: string;
    /** @see https://jamiemason.github.io/syncpack/config/custom-types/#namestrategy */
    strategy: 'version';
  }
  export interface VersionsByName {
    /** @see https://jamiemason.github.io/syncpack/config/custom-types/#name */
    path: string;
    /** @see https://jamiemason.github.io/syncpack/config/custom-types/#namestrategy */
    strategy: 'versionsByName';
  }
  export type Any = NameAndVersionProps | NamedVersionString | UnnamedVersionString | VersionsByName;
}

type SemverRange = '' | '*' | '>' | '>=' | '.x' | '<' | '<=' | '^' | '~';

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
