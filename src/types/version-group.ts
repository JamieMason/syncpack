import type { DependencyType } from '.';

export type AnyVersionGroup =
  | VersionGroup
  | BannedVersionGroup
  | IgnoredVersionGroup
  | PinnedVersionGroup;

export interface VersionGroup {
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
  dependencyTypes?: DependencyType[];
}

export interface BannedVersionGroup extends VersionGroup {
  /**
   * optionally force all dependencies in this group to be removed
   */
  isBanned: true;
}

export interface IgnoredVersionGroup extends VersionGroup {
  /**
   * optionally force syncpack to ignore all dependencies in this group
   */
  isIgnored?: true;
}

export interface PinnedVersionGroup extends VersionGroup {
  /**
   * optionally force all dependencies in this group to have this version
   */
  pinVersion?: string;
}
