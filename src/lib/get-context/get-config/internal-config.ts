import type { Config, DependencyType } from './config';

export interface InternalConfig extends Config.RcFile {
  /**
   * The standard/catch-all semver group.
   *
   * + When no semver groups are defined, this will be the only group.
   * + Otherwise this group will appear last, to be used when none of the user's
   *   groups found a match.
   */
  defaultSemverGroup: Config.SemverGroup.WithRange;
  /**
   * The standard/catch-all version group.
   *
   * + When no version groups are defined, this will be the only group.
   * + Otherwise this group will appear last, to be used when none of the user's
   *   groups found a match.
   */
  defaultVersionGroup: Config.VersionGroup.Standard;
  /**
   * Aliases for locations of versions within package.json files, it is looped
   * over by each command to operate on each are as defined by the user.
   */
  dependencyTypes: DependencyType[];
}
