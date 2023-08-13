/** Single source of truth, intended to aid testing or to override */
export const CWD = process.env.MOCK_CWD || process.cwd();

/** Single source of truth for icons used in output */
export const ICON = {
  cross: '✘',
  debug: '?',
  panic: '!',
  rightArrow: '→',
  skip: '-',
  tick: '✓',
} as const;

export const RANGE = {
  ANY: '*',
  EXACT: '',
  GT: '>',
  GTE: '>=',
  LOOSE: '.x',
  LT: '<',
  LTE: '<=',
  MINOR: '^',
  PATCH: '~',
  WORKSPACE: 'workspace:',
} as const;

export const INTERNAL_TYPES = [
  'dev',
  'local',
  'overrides',
  'peer',
  'pnpmOverrides',
  'prod',
  'resolutions',
] as const;

export const DEFAULT_CONFIG = {
  dependencyTypes: ['**'],
  filter: '.',
  indent: '  ',
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
  source: ['package.json', 'packages/*/package.json'],
  versionGroups: [],
} as const;
