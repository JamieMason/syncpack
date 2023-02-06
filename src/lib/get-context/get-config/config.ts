import type { ALL_DEPENDENCY_TYPES, RANGE } from '../../../constants';

/** Aliases for locations within package.json files where versions can be found */
export type DependencyType = (typeof ALL_DEPENDENCY_TYPES)[number];

/** Aliases for semver range formats supported by syncpack */
export type ValidRange = (typeof RANGE)[keyof typeof RANGE];

export namespace Config {
  export namespace SemverGroup {
    /** All valid forms of Semver Group */
    export type Any = Ignored | WithRange;

    export interface Ignored extends Base {
      /** Optionally force syncpack to ignore all dependencies in this group */
      isIgnored: true;
    }

    export interface WithRange extends Base {
      /** The semver range which dependencies in this group should use */
      range: ValidRange;
    }

    interface Base {
      /**
       * The names of packages in your monorepo which belong to this group, taken
       * from the "name" field in package.json, not the package directory name
       */
      packages: string[];
      /** Dependency names (eg. "lodash") which belong to this group */
      dependencies: string[];
      /** Optionally limit this group to dependencies of the provided types */
      dependencyTypes?: DependencyType[];
    }
  }

  export namespace VersionGroup {
    /** All valid forms of Version Group */
    export type Any = Standard | Banned | Ignored | Pinned;

    export interface Standard {
      /**
       * The names of packages in your monorepo which belong to this group, taken
       * from the "name" field in package.json, not the package directory name
       */
      packages: string[];
      /** Dependency names (eg. "lodash") which belong to this group */
      dependencies: string[];
      /** Optionally limit this group to dependencies of the provided types */
      dependencyTypes?: DependencyType[];
    }

    export interface Banned extends Standard {
      /** Optionally force all dependencies in this group to be removed */
      isBanned: true;
    }

    export interface Ignored extends Standard {
      /** Optionally force syncpack to ignore all dependencies in this group */
      isIgnored?: true;
    }

    export interface Pinned extends Standard {
      /** Optionally force all dependencies in this group to have this version */
      pinVersion?: string;
    }
  }

  /**
   * Custom property path within package.json files where versions can be found
   */
  export type DependencyCustomPath = { name: string; path: string };

  /** All valid config which can only be provided via .syncpackrc */
  interface RcFileOnly {
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
    /** */
    versionGroups: VersionGroup.Any[];
    /** */
    semverGroups: SemverGroup.Any[];
    /** Custom path in the package.json that point to a dependencies
     * @example {name: 'foo', path: 'x:y.z'}
     */
    dependenciesCustomPaths: DependencyCustomPath[];
  }

  /** All valid config which can only be provided via the CLI */
  interface CliOnly {
    /** Absolute or relative path to a .syncpackrc */
    configPath: string;
  }

  /** All valid config which can be provided via the CLI or .syncpackrc */
  export interface CliAndRcFile {
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
     * Defaults to `["package.json", "packages/\*\/package.json"]` to match most
     * Projects using Lerna or Yarn Workspaces, but this can be overridden in your
     * config file or via multiple `--source` command line options.
     *
     * Supports any patterns supported by https://github.com/isaacs/node-glob
     */
    source: string[];
  }

  /** All valid config in .syncpackrc */
  export type RcFile = RcFileOnly & CliAndRcFile;

  /** All valid config recognised by Syncpack */
  export type All = CliOnly & RcFile;
}
