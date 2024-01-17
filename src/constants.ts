import type { RcConfig, SemverRange } from './config/types.js';

/** Single source of truth, intended to aid testing or to override */
export const CWD = process.env.MOCK_CWD || process.cwd();

/** Single source of truth for icons used in output */
export const ICON = {
  banned: '⦸',
  cross: '✘',
  debug: '?',
  info: 'i',
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
} as const satisfies Record<string, SemverRange>;

export const INTERNAL_TYPES = [
  'dev',
  'local',
  'overrides',
  'peer',
  'pnpmOverrides',
  'prod',
  'resolutions',
] as const;

export const CUSTOM_TYPES = {
  dev: {
    strategy: 'versionsByName',
    path: 'devDependencies',
  },
  local: {
    strategy: 'name~version',
    namePath: 'name',
    path: 'version',
  },
  overrides: {
    strategy: 'versionsByName',
    path: 'overrides',
  },
  peer: {
    strategy: 'versionsByName',
    path: 'peerDependencies',
  },
  pnpmOverrides: {
    strategy: 'versionsByName',
    path: 'pnpm.overrides',
  },
  prod: {
    strategy: 'versionsByName',
    path: 'dependencies',
  },
  resolutions: {
    strategy: 'versionsByName',
    path: 'resolutions',
  },
} as const;

export const DEFAULT_CONFIG = {
  customTypes: CUSTOM_TYPES,
  dependencyTypes: ['**'],
  filter: '.',
  formatBugs: true,
  formatRepository: true,
  indent: '  ',
  lintFormatting: true,
  lintSemverRanges: true,
  lintVersions: true,
  semverGroups: [],
  sortAz: [
    'bin',
    'contributors',
    'dependencies',
    'devDependencies',
    'keywords',
    'peerDependencies',
    'resolutions',
    'scripts',
  ],
  sortExports: [
    'types',
    'node-addons',
    'node',
    'browser',
    'import',
    'require',
    'development',
    'production',
    'default',
  ],
  sortFirst: ['name', 'description', 'version', 'author'],
  sortPackages: true,
  source: ['package.json', 'packages/*/package.json'],
  specifierTypes: ['**'],
  versionGroups: [],
} satisfies RcConfig;
