export type DependencyType =
  | 'dependencies'
  | 'devDependencies'
  | 'overrides'
  | 'peerDependencies'
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

export interface SemverGroup {
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
   * the semver range which dependencies in this group should use
   */
  range: ValidRange;
  /**
   * optionally only apply this group to dependencies of the provided types
   */
  dependencyTypes?: DependencyType[];
}

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
   * optionally force all dependencies in this group to be removed
   */
  isBanned?: true;
  /**
   * optionally force all dependencies in this group to have this version
   */
  pinVersion?: string;
  /**
   * optionally only apply this group to dependencies of the provided types
   */
  dependencyTypes?: DependencyType[];
}

export type SyncpackConfig = Readonly<{
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
   * whether to search within pnpm overrides
   */
  overrides: boolean;
  /**
   * whether to search within peerDependencies
   */
  peer: boolean;
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
  semverGroups: SemverGroup[];
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
  versionGroups: VersionGroup[];
  /**
   * whether to include the versions of the `--source` packages developed in
   * your workspace/monorepo as part of the search for versions to sync
   */
  workspace: boolean;
}>;

export const ALL_PATTERNS = ['package.json', 'packages/*/package.json'];

export const DEPENDENCY_TYPES: DependencyType[] = [
  'dependencies',
  'devDependencies',
  'overrides',
  'peerDependencies',
  'resolutions',
  'workspace',
];

export const CWD = process.cwd();

export const GREATER = 1;
export const LESSER = -1;
export const SAME = 0;

export const RANGE_ANY = '*';
export const RANGE_EXACT = '';
export const RANGE_GT = '>';
export const RANGE_GTE = '>=';
export const RANGE_LOOSE = '.x';
export const RANGE_LT = '<';
export const RANGE_LTE = '<=';
export const RANGE_MINOR = '^';
export const RANGE_PATCH = '~';

export const SEMVER_ORDER: ValidRange[] = [
  RANGE_LT,
  RANGE_LTE,
  RANGE_EXACT,
  RANGE_PATCH,
  RANGE_MINOR,
  RANGE_GTE,
  RANGE_GT,
  RANGE_ANY,
];

export const DEFAULT_CONFIG: SyncpackConfig = {
  dependencyTypes: [],
  dev: true,
  filter: '.',
  indent: '  ',
  overrides: true,
  peer: true,
  prod: true,
  resolutions: true,
  workspace: true,
  semverGroups: [],
  semverRange: '',
  sortAz: [
    'contributors',
    'dependencies',
    'devDependencies',
    'keywords',
    'peerDependencies',
    'resolutions',
    'scripts',
  ],
  sortFirst: ['name', 'description', 'version', 'author'],
  source: [],
  versionGroups: [],
};
