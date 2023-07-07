import * as Context from '@effect/data/Context';

/**
 * Aliases for semver range formats supported by syncpack
 *
 * Defaults to `""` to ensure that exact dependency versions are used
 * instead of loose ranges, but this can be overridden in your config file
 * or via the `--semver-range` command line option.
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
export type SemverRange = '' | '*' | '>' | '>=' | '.x' | '<' | '<=' | '^' | '~';

export interface GroupConfig {
  dependencies: string[];
  dependencyTypes?: string[];
  label?: string;
  packages: string[];
}

export namespace SemverGroupConfig {
  export interface Ignored extends GroupConfig {
    isIgnored: true;
  }

  export interface WithRange extends GroupConfig {
    range: SemverRange;
  }

  export type Any = GroupConfig & Partial<Ignored & WithRange>;
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

  export interface SnappedTo extends GroupConfig {
    snapTo: string[];
  }

  export interface Standard extends GroupConfig {
    preferVersion?: 'highestSemver' | 'lowestSemver';
  }

  export type Any = GroupConfig & Partial<Banned & Ignored & Pinned & SameRange & SnappedTo & Standard>;
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

  export type Any = NameAndVersionProps | NamedVersionString | UnnamedVersionString | VersionsByName;
}

export interface CliConfig {
  readonly configPath?: string;
  readonly filter: string;
  readonly indent: string;
  readonly semverRange: SemverRange;
  readonly source: string[];
  readonly types: string;
}

export const CliConfigTag = Context.Tag<Partial<CliConfig>>();

// @TODO formatBugs: boolean // whether to format "bugs" prop (default: true)
// @TODO formatRepository: boolean // whether to format "repository" prop (default: true)
// @TODO sortPackages: boolean // whether to sort root props (default: true)

export interface RcConfig {
  /** @see https://jamiemason.github.io/syncpack/config/custom-types */
  customTypes: Record<string, CustomTypeConfig.Any>;
  /** @see https://jamiemason.github.io/syncpack/config/dependency-types */
  dependencyTypes: string[];
  /** @see https://jamiemason.github.io/syncpack/config/filter */
  filter: string;
  /** @see https://jamiemason.github.io/syncpack/config/indent */
  indent: string;
  /** @see https://jamiemason.github.io/syncpack/config/semver-groups */
  semverGroups: SemverGroupConfig.Any[];
  /** @see https://jamiemason.github.io/syncpack/config/semver-range */
  semverRange: SemverRange;
  /** @see https://jamiemason.github.io/syncpack/config/sort-az */
  sortAz: string[];
  /** @see https://jamiemason.github.io/syncpack/config/sort-first */
  sortFirst: string[];
  /** @see https://jamiemason.github.io/syncpack/config/source */
  source: string[];
  /** @see https://jamiemason.github.io/syncpack/config/version-groups */
  versionGroups: VersionGroupConfig.Any[];
}
