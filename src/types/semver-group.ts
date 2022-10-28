import type { ValidRange, DependencyType } from '.';

export type AnySemverGroup = IgnoredSemverGroup | SemverGroup;

export interface IgnoredSemverGroup extends Base {
  /**
   * optionally force syncpack to ignore all dependencies in this group
   */
  isIgnored: true;
}

export interface SemverGroup extends Base {
  /**
   * the semver range which dependencies in this group should use
   */
  range: ValidRange;
}

interface Base {
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
