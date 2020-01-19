import { IManifestKey } from './typings';

export const DEPENDENCY_TYPES: IManifestKey[] = [
  'dependencies',
  'devDependencies',
  'peerDependencies',
];

export const SORT_AZ = [
  'contributors',
  'dependencies',
  'devDependencies',
  'files',
  'keywords',
  'peerDependencies',
  'scripts',
];

export const SORT_FIRST = ['name', 'description', 'version', 'author'];
export const VERSION = require('../package.json').version;
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

export const SEMVER_ORDER = [
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
const ALL_PATTERNS = [MONOREPO_PATTERN, PACKAGES_PATTERN];

export const FIX_MISMATCHES = {
  command: 'fix-mismatches',
  description:
    'set dependencies used with different versions to the same version',
};

export const FORMAT = {
  command: 'format',
  description: 'sort and shorten properties according to a convention',
};

export const LIST = {
  command: 'list',
  description: 'list every dependency used in your packages',
};

export const LIST_MISMATCHES = {
  command: 'list-mismatches',
  description:
    'list every dependency used with different versions in your packages',
};

export const SET_SEMVER_RANGES = {
  command: 'set-semver-ranges',
  description: 'set semver ranges to the given format',
};

export const OPTION_SEMVER_RANGE = {
  default: DEFAULT_SEMVER_RANGE,
  description:
    `${RANGE_LT}, ${RANGE_LTE}, "${RANGE_EXACT}", ${RANGE_PATCH}, ${RANGE_MINOR}, ` +
    `${RANGE_GTE}, ${RANGE_GT}, or ${RANGE_ANY}. defaults to "${DEFAULT_SEMVER_RANGE}"`,
  spec: '-r, --semver-range <range>',
};

export const OPTION_SOURCES = {
  default: ALL_PATTERNS,
  description: 'glob pattern for package.json files to read from',
  spec: '-s, --source [pattern]',
};

export const OPTIONS_PROD = {
  description: 'include dependencies',
  spec: '-p, --prod',
};

export const OPTIONS_DEV = {
  description: 'include devDependencies',
  spec: '-d, --dev',
};

export const OPTIONS_PEER = {
  description: 'include peerDependencies',
  spec: '-P, --peer',
};

export const OPTIONS_FILTER_DEPENDENCIES = {
  description: 'regex for depdendency filter',
  spec: '-f, --filter [pattern]',
};

export const OPTION_INDENT = {
  default: DEFAULT_INDENT,
  description: `override indentation. defaults to "${DEFAULT_INDENT}"`,
  spec: '-i, --indent [value]',
};
