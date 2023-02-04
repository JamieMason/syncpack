import type { InternalConfig } from '.';

export namespace VersionGroup {
  export type Any = Standard | Banned | Ignored | Pinned;

  export interface Standard {
    /**
     * the names of packages in your monorepo which belong to this group, taken
     * from the "name" field in package.json, not the package directory name
     */
    packages: string[];
    /**
     * the names of the dependencies (eg. "lodash") which belong to this group
     */
    dependencies: string[];
    /**
     * optionally only apply this group to dependencies of the provided types
     */
    dependencyTypes?: InternalConfig['dependencyTypes'];
  }

  export interface Banned extends Standard {
    /**
     * optionally force all dependencies in this group to be removed
     */
    isBanned: true;
  }

  export interface Ignored extends Standard {
    /**
     * optionally force syncpack to ignore all dependencies in this group
     */
    isIgnored?: true;
  }

  export interface Pinned extends Standard {
    /**
     * optionally force all dependencies in this group to have this version
     */
    pinVersion?: string;
  }
}
