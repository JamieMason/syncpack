import type {
  RANGE_ANY,
  RANGE_EXACT,
  RANGE_GT,
  RANGE_GTE,
  RANGE_LOOSE,
  RANGE_LT,
  RANGE_LTE,
  RANGE_MINOR,
  RANGE_PATCH,
} from '../constants';
import type { AnySemverGroup } from './semver-group';
import type { AnyVersionGroup } from './version-group';

export type DependencyType =
  | 'dependencies'
  | 'devDependencies'
  | 'overrides'
  | 'peerDependencies'
  | 'pnpmOverrides'
  | 'resolutions'
  | 'workspace';

export type DependencyOption = Pick<
  SyncpackConfig,
  'dev' | 'workspace' | 'overrides' | 'peer' | 'prod' | 'resolutions'
>;

export type ValidRange =
  | typeof RANGE_ANY
  | typeof RANGE_EXACT
  | typeof RANGE_GT
  | typeof RANGE_GTE
  | typeof RANGE_LOOSE
  | typeof RANGE_LT
  | typeof RANGE_LTE
  | typeof RANGE_MINOR
  | typeof RANGE_PATCH;

export interface SyncpackConfig {
  /**
   * which dependency properties to search within
   */
  dependencyTypes: DependencyType[];
  /**
   * whether to search within devDependencies
   */
  dev: boolean;
  /**
   * a string which will be passed to `new RegExp()` to match against package
   * names that should be included
   */
  filter: string;
  /**
   * the character(s) to be used to indent your package.json files when writing
   * to disk
   */
  indent: string;
  /**
   * whether to search within npm overrides
   */
  overrides: boolean;
  /**
   * whether to search within peerDependencies
   */
  peer: boolean;
  /**
   * whether to search within pnpm overrides
   */
  pnpmOverrides: boolean;
  /**
   * whether to search within dependencies
   */
  prod: boolean;
  /**
   * whether to search within yarn resolutions
   */
  resolutions: boolean;
  /**
   *
   */
  semverGroups: AnySemverGroup[];
  /**
   * defaults to `""` to ensure that exact dependency versions are used instead
   * of loose ranges
   */
  semverRange: ValidRange;
  /**
   * which fields within package.json files should be sorted alphabetically
   */
  sortAz: string[];
  /**
   * which fields within package.json files should appear at the top, and in
   * what order
   */
  sortFirst: string[];
  /**
   * glob patterns for package.json file locations
   */
  source: string[];
  /**
   *
   */
  versionGroups: AnyVersionGroup[];
  /**
   * whether to include the versions of the `--source` packages developed in
   * your workspace/monorepo as part of the search for versions to sync
   */
  workspace: boolean;
}
