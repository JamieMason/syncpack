import type { CUSTOM_TYPES } from '../constants.js';
import type { Specifier } from '../specifier/index.js';

/**
 * Aliases for semver range formats supported by syncpack
 *
 * Defaults to `""` to ensure that exact dependency versions are used instead of
 * loose ranges, but this can be overridden in your config file.
 *
 * | Supported Range |   Example |
 * | --------------- | --------: |
 * | `"<"`           |  `<1.4.2` |
 * | `"<="`          | `<=1.4.2` |
 * | `""`            |   `1.4.2` |
 * | `"~"`           |  `~1.4.2` |
 * | `"^"`           |  `^1.4.2` |
 * | `">="`          | `>=1.4.2` |
 * | `">"`           |  `>1.4.2` |
 * | `"*"`           |       `*` |
 *
 * @default ""
 */
export type SemverRange = '' | '*' | '>' | '>=' | '.x' | '<' | '<=' | '^' | '~' | 'workspace:';

type DefaultDependencyType = keyof typeof CUSTOM_TYPES;

export type DependencyType =
  | DefaultDependencyType
  | `!${DefaultDependencyType}`
  // This is done to allow any other `string` while also offering intellisense
  // for the internal dependency types above. `(string & {})` is needed to
  // prevent typescript from ignoring these specific strings and merging them
  // all into `string`, where we'd lose any editor autocomplete for the other
  // more specific fields, using (string & {}) stops that from happening.
  //
  // eslint-disable-next-line @typescript-eslint/ban-types
  | (string & {});

export type SpecifierType =
  | Specifier.Any['name']
  | `!${Specifier.Any['name']}`
  // This is done to allow any other `string` while also offering intellisense
  // for the internal dependency types above. `(string & {})` is needed to
  // prevent typescript from ignoring these specific strings and merging them
  // all into `string`, where we'd lose any editor autocomplete for the other
  // more specific fields, using (string & {}) stops that from happening.
  //
  // eslint-disable-next-line @typescript-eslint/ban-types
  | (string & {});

export interface GroupConfig {
  dependencies?: string[];
  dependencyTypes?: DependencyType[];
  label?: string;
  packages?: string[];
  specifierTypes?: SpecifierType[];
}

export namespace SemverGroupConfig {
  export interface Disabled extends GroupConfig {
    isDisabled: true;
  }

  export interface Ignored extends GroupConfig {
    isIgnored: true;
  }

  export interface WithRange extends GroupConfig {
    range: SemverRange;
  }

  export type Any = Disabled | Ignored | WithRange;
}

export namespace VersionGroupConfig {
  export interface Banned extends GroupConfig {
    isBanned: true;
  }

  export interface Ignored extends GroupConfig {
    isIgnored: true;
  }

  export interface Pinned extends GroupConfig {
    pinVersion: string;
  }

  export interface SnappedTo extends GroupConfig {
    snapTo: string[];
  }

  export interface SameRange extends GroupConfig {
    policy: 'sameRange';
  }

  export interface Standard extends GroupConfig {
    preferVersion?: 'highestSemver' | 'lowestSemver';
  }

  export type Any = Banned | Ignored | Pinned | SameRange | SnappedTo | Standard;
}

namespace CustomTypeConfig {
  export interface NameAndVersionProps {
    namePath: string;
    path: string;
    strategy: 'name~version';
  }

  export interface NamedVersionString {
    path: string;
    strategy: 'name@version';
  }

  export interface UnnamedVersionString {
    path: string;
    strategy: 'version';
  }

  export interface VersionsByName {
    path: string;
    strategy: 'versionsByName';
  }

  export type Any =
    | NameAndVersionProps
    | NamedVersionString
    | UnnamedVersionString
    | VersionsByName;
}

export interface CliConfig {
  readonly configPath?: string;
  readonly filter: string;
  readonly indent: string;
  readonly source: string[];
  readonly specs: string;
  readonly types: string;
}

export interface RcConfig {
  /** @see https://jamiemason.github.io/syncpack/config/custom-types */
  customTypes: Record<string, CustomTypeConfig.Any>;
  /** @see https://jamiemason.github.io/syncpack/config/dependency-types */
  dependencyTypes: DependencyType[];
  /** @see https://jamiemason.github.io/syncpack/config/filter */
  filter: string;
  /** @see https://jamiemason.github.io/syncpack/config/format-bugs */
  formatBugs: boolean;
  /** @see https://jamiemason.github.io/syncpack/config/format-repository */
  formatRepository: boolean;
  /** @see https://jamiemason.github.io/syncpack/config/indent */
  indent: string;
  /** @see https://jamiemason.github.io/syncpack/config/lint-formatting */
  lintFormatting: boolean;
  /** @see https://jamiemason.github.io/syncpack/config/lint-semver-ranges */
  lintSemverRanges: boolean;
  /** @see https://jamiemason.github.io/syncpack/config/lint-versions */
  lintVersions: boolean;
  /** @see https://jamiemason.github.io/syncpack/config/semver-groups */
  semverGroups: SemverGroupConfig.Any[];
  /** @see https://jamiemason.github.io/syncpack/config/sort-az */
  sortAz: string[];
  /** @see https://jamiemason.github.io/syncpack/config/sort-exports */
  sortExports: string[];
  /** @see https://jamiemason.github.io/syncpack/config/sort-first */
  sortFirst: string[];
  /** @see https://jamiemason.github.io/syncpack/config/sort-packages */
  sortPackages: boolean;
  /** @see https://jamiemason.github.io/syncpack/config/source */
  source: string[];
  /** @see https://jamiemason.github.io/syncpack/config/specifier-types */
  specifierTypes: SpecifierType[];
  /** @see https://jamiemason.github.io/syncpack/config/version-groups */
  versionGroups: VersionGroupConfig.Any[];
}
