import { IManifestKey } from './typings';

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
export const SORT_AZ = ['dependencies', 'devDependencies', 'files', 'keywords', 'peerDependencies', 'scripts'];
export const SORT_FIRST = ['name', 'description', 'version', 'author'];
export const VERSION = execSync(`npm view ${__dirname} version`, { encoding: 'utf8' });
