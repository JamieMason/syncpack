import { IManifestKey } from './typings';

export const DEPENDENCY_TYPES: IManifestKey[] = ['dependencies', 'devDependencies', 'peerDependencies'];
export const SORT_AZ = ['dependencies', 'devDependencies', 'files', 'keywords', 'peerDependencies', 'scripts'];
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
  RANGE_ANY
];

const DEFAULT_SEMVER_RANGE = RANGE_EXACT;
const MONOREPO_PATTERN = './package.json';
const PACKAGES_PATTERN = './packages/*/package.json';
const PACKAGES_PATTERNS = [PACKAGES_PATTERN];
const ALL_PATTERNS = [MONOREPO_PATTERN, PACKAGES_PATTERN];

export const FIX_MISMATCHES = {
  args: '[packages...]',
  command: 'fix-mismatches',
  defaultPatterns: PACKAGES_PATTERNS,
  description: 'set dependencies used with different versions to the same version'
};

export const FORMAT = {
  args: '[packages...]',
  command: 'format',
  defaultPatterns: ALL_PATTERNS,
  description: 'sort and shorten properties according to a convention'
};

export const LIST = {
  args: '[packages...]',
  command: 'list',
  defaultPatterns: PACKAGES_PATTERNS,
  description: 'list every dependency used in your packages'
};

export const LIST_MISMATCHES = {
  args: '[packages...]',
  command: 'list-mismatches',
  defaultPatterns: PACKAGES_PATTERNS,
  description: 'list every dependency used with different versions in your packages'
};

export const SET_SEMVER_RANGES = {
  args: '[packages...]',
  command: 'set-semver-ranges',
  defaultPatterns: ALL_PATTERNS,
  description: 'set semver ranges to the given format'
};

export const OPTION_SEMVER_RANGE = {
  default: DEFAULT_SEMVER_RANGE,
  description:
    `${RANGE_LT}, ${RANGE_LTE}, "${RANGE_EXACT}", ${RANGE_PATCH}, ${RANGE_MINOR}, ` +
    `${RANGE_GTE}, ${RANGE_GT}, or ${RANGE_ANY}. defaults to "${DEFAULT_SEMVER_RANGE}"`,
  spec: '-r, --semver-range <range>'
};
