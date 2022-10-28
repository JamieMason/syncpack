import type { DependencyType, SyncpackConfig, ValidRange } from './types';

export const ALL_PATTERNS = ['package.json', 'packages/*/package.json'];

export const DEPENDENCY_TYPES: DependencyType[] = [
  'dependencies',
  'devDependencies',
  'overrides',
  'peerDependencies',
  'pnpmOverrides',
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

export const ICON = {
  cross: '✘',
  debug: '?',
  skip: '-',
  tick: '✓',
};

export const DEFAULT_CONFIG: SyncpackConfig = {
  dependencyTypes: [],
  dev: true,
  filter: '.',
  indent: '  ',
  overrides: true,
  peer: true,
  pnpmOverrides: true,
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
