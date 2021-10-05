import { collect } from './lib/collect';

export type DependencyType = 'dependencies' | 'devDependencies' | 'peerDependencies';
export const DEPENDENCY_TYPES: DependencyType[] = ['dependencies', 'devDependencies', 'peerDependencies'];

export const GREATER = 1;
export const LESSER = -1;
export const SAME = 0;

export type ValidRange = '*' | '' | '>' | '>=' | '.x' | '<' | '<=' | '^' | '~';
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
}

export type SyncpackConfig = Readonly<{
  /**
   * whether to search within devDependencies
   */
  dev: boolean;
  /**
   * A string which will be passed to `new RegExp()` to match against package
   * names that should be included
   */
  filter: string;
  /**
   * The character(s) to be used to indent your package.json files when writing
   * to disk
   */
  indent: string;
  /**
   * whether to search within peerDependencies
   */
  peer: boolean;
  /**
   * whether to search within dependencies
   */
  prod: boolean;
  /**
   * whether versions should be considered equal if their version ranges match
   */
  matchRanges: boolean;
  /**
   * defaults to `""` to ensure that exact dependency versions are used instead
   * of loose ranges
   */
  semverRange: string;
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
}>;

export const DEFAULT_CONFIG: SyncpackConfig = {
  dev: true,
  filter: '.',
  indent: '  ',
  peer: true,
  prod: true,
  matchRanges: false,
  semverRange: '',
  sortAz: ['contributors', 'dependencies', 'devDependencies', 'keywords', 'peerDependencies', 'resolutions', 'scripts'],
  sortFirst: ['name', 'description', 'version', 'author'],
  source: [],
  versionGroups: [],
};

const MONOREPO_PATTERN = 'package.json';
const PACKAGES_PATTERN = 'packages/*/package.json';

export const ALL_PATTERNS = [MONOREPO_PATTERN, PACKAGES_PATTERN];

interface OptionsByName {
  dev: [string, string];
  filter: [string, string];
  indent: [string, string];
  peer: [string, string];
  prod: [string, string];
  matchRanges: [string, string];
  semverRange: [string, string];
  source: [string, string, typeof collect, string[]];
}

export const option: OptionsByName = {
  dev: ['-d, --dev', 'include devDependencies'],
  filter: ['-f, --filter [pattern]', 'regex for dependency filter'],
  indent: ['-i, --indent [value]', `override indentation. defaults to "${DEFAULT_CONFIG.indent}"`],
  peer: ['-P, --peer', 'include peerDependencies'],
  prod: ['-p, --prod', 'include dependencies'],
  matchRanges: ['-m, --match-ranges', 'include dependencies'],
  semverRange: [
    '-r, --semver-range <range>',
    `see supported ranges below. defaults to "${DEFAULT_CONFIG.semverRange}"`,
  ],
  source: ['-s, --source [pattern]', 'glob pattern for package.json files to read from', collect, []],
};
