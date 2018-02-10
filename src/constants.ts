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

export const FIX_MISMATCHES = {
  description: 'set dependencies used with different versions to the same version',
  name: 'fix-mismatches'
};
export const FORMAT = {
  description: 'sort and shorten properties according to a convention',
  name: 'format'
};
export const LIST = {
  description: 'list every dependency used in your packages',
  name: 'list'
};
export const LIST_MISMATCHES = {
  description: 'list every dependency used with different versions in your packages',
  name: 'list-mismatches'
};
export const SET_SEMVER_RANGES = {
  description: 'set semver ranges to the given format',
  name: 'set-semver-ranges'
};

export const DEFAULT_PATTERN = './packages/*/package.json';
export const DEFAULT_SEMVER_RANGE = RANGE_EXACT;
export const OPTION_PACKAGES = {
  description: `location of packages. defaults to '${DEFAULT_PATTERN}'`,
  spec: '-p, --packages <glob>'
};
export const OPTION_SEMVER_RANGE = {
  description: `${[RANGE_ANY, RANGE_EXACT, RANGE_GT, RANGE_GTE, RANGE_LOOSE, RANGE_LT, RANGE_LTE, RANGE_MINOR]
    .map((value) => `'${value}'`)
    .join(', ')}, or '${RANGE_PATCH}'. defaults to '${DEFAULT_SEMVER_RANGE}'`,
  spec: '-r, --semver-range <range>'
};
