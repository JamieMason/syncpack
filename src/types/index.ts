import type { ALL_DEPENDENCY_TYPES, RANGE } from '../constants';
import type { SemverGroup } from './semver-group';
import type { VersionGroup } from './version-group';

/** Aliases for locations within package.json files where versions can be found */
export type DependencyType = typeof ALL_DEPENDENCY_TYPES[number];

/** Aliases for semver range formats supported by syncpack */
export type ValidRange = typeof RANGE[keyof typeof RANGE];

/** All valid config which can be provided via a .syncpackrc  */
export interface SyncpackConfig {
  /** Whether to search within `devDependencies` */
  dev: boolean;
  /** Whether to search within npm `overrides` */
  overrides: boolean;
  /** Whether to search within `peerDependencies` */
  peer: boolean;
  /** Whether to search within `pnpm.overrides` */
  pnpmOverrides: boolean;
  /** Whether to search within `dependencies` */
  prod: boolean;
  /** Whether to search within yarn `resolutions` */
  resolutions: boolean;
  /**
   * Whether to include the versions of the `--source` packages developed in
   * your workspace/monorepo as part of the search for versions to sync
   */
  workspace: boolean;
  /**
   * A string which will be passed to `new RegExp()` to match against package
   * names that should be included.
   *
   * > ⚠️ `filter` was originally intended as a convenience to be used from the
   * > command line to filter the output of `syncpack list`, it is not recommended
   * > to add this to your config file to manage your project more generally.
   * >
   * > Instead use `versionGroups` and/or `semverGroups`.
   *
   */
  filter: string;
  /**
   * The character(s) to be used to indent your package.json files when writing
   * to disk
   */
  indent: string;
  /**
   * Defaulted to `""` to ensure that exact dependency versions are used instead
   * of loose ranges, but this can be overridden in your config file or via the
   * `--semver-range` command line option.
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
   */
  semverRange: ValidRange;
  /**
   * When using the `format` command, determines which fields within
   * package.json files should be sorted alphabetically. When the value is an
   * Object, its keys are sorted alphabetically. When the value is an Array, its
   * values are sorted alphabetically. There is no equivalent CLI Option for
   * this configuration.
   */
  sortAz: string[];
  /**
   * When using the `format` command, determines which fields within package.json
   * files should appear at the top, and in what order. There is no equivalent
   * CLI Option for this configuration.
   */
  sortFirst: string[];
  /**
   * Defaults to `["package.json", "packages/\*\/package.json"]` to match most
   * Projects using Lerna or Yarn Workspaces, but this can be overridden in your
   * config file or via multiple `--source` command line options.
   *
   * Supports any patterns supported by https://github.com/isaacs/node-glob
   */
  source: string[];
  /** */
  versionGroups: VersionGroup.Any[];
  /** */
  semverGroups: SemverGroup.Any[];
}

export interface InternalConfig extends SyncpackConfig {
  /**
   * Aliases for locations of versions within package.json files, it is looped
   * over by each command to operate on each are as defined by the user.
   */
  dependencyTypes: DependencyType[];
}
