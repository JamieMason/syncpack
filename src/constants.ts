import { IManifestKey } from './typings';

export const FIX_MISMATCHES = {
  description: 'set dependencies used with different versions to the same version',
  name: 'fix-mismatches'
};
export const LIST = {
  description: 'list every dependency used in your packages',
  name: 'list'
};
export const LIST_MISMATCHES = {
  description: 'list every dependency used with different versions in your packages',
  name: 'list-mismatches'
};
export const DEFAULT_PATTERN = './packages/*/package.json';
export const DEPENDENCY_TYPES: IManifestKey[] = ['dependencies', 'devDependencies', 'peerDependencies'];
export const GREATER = 1;
export const LESSER = -1;
export const OPTION_PACKAGES = {
  description: `location of packages, defaults to "${DEFAULT_PATTERN}"`,
  spec: '-p, --packages <glob>'
};
export const SAME = 0;
export const SEMVER_ORDER = ['<', '<=', '', '~', '^', '>=', '>', '*'];
