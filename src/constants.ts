import { collect } from './lib/collect';

export type DependencyType = 'dependencies' | 'devDependencies' | 'peerDependencies';
export const DEPENDENCY_TYPES: DependencyType[] = ['dependencies', 'devDependencies', 'peerDependencies'];

export const SORT_AZ = ['contributors', 'dependencies', 'devDependencies', 'keywords', 'peerDependencies', 'scripts'];

export const SORT_FIRST = ['name', 'description', 'version', 'author'];
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

const DEFAULT_INDENT = '  ';
const DEFAULT_SEMVER_RANGE = RANGE_EXACT;
const MONOREPO_PATTERN = 'package.json';
const PACKAGES_PATTERN = 'packages/*/package.json';

export const ALL_PATTERNS = [MONOREPO_PATTERN, PACKAGES_PATTERN];

interface OptionsByName {
  dev: [string, string];
  filter: [string, string];
  indent: [string, string];
  peer: [string, string];
  prod: [string, string];
  semverRange: [string, string];
  source: [string, string, typeof collect, string[]];
}

export const option: OptionsByName = {
  dev: ['-d, --dev', 'include devDependencies'],
  filter: ['-f, --filter [pattern]', 'regex for dependency filter'],
  indent: ['-i, --indent [value]', `override indentation. defaults to "${DEFAULT_INDENT}"`],
  peer: ['-P, --peer', 'include peerDependencies'],
  prod: ['-p, --prod', 'include dependencies'],
  semverRange: ['-r, --semver-range <range>', `see supported ranges below. defaults to "${DEFAULT_SEMVER_RANGE}"`],
  source: ['-s, --source [pattern]', 'glob pattern for package.json files to read from', collect, []],
};
