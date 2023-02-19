/** Single source of truth, intended to aid testing or to override */
export const CWD = process.env.MOCK_CWD || process.cwd();

/** Where to search for packages if none are provided by the user */
export const DEFAULT_SOURCES = ['package.json', 'packages/*/package.json'];

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
} as const;

export const DEFAULT_CONFIG = {
  dependencyTypes: [
    'dev',
    'overrides',
    'peer',
    'pnpmOverrides',
    'prod',
    'resolutions',
    'workspace',
  ],
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
  source: [],
  versionGroups: [],
} as const;
