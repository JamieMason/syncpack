import { IPackageJsonKey } from './typings';

export const DEFAULT_PACKAGES = './packages/*/package.json';
export const DEFAULT_SOURCE = './package.json';
export const DEPENDENCY_TYPES: IPackageJsonKey[] = ['dependencies', 'devDependencies', 'peerDependencies'];
export const GREATER = 1;
export const LESSER = -1;
export const SAME = 0;
export const SEMVER_ORDER = ['<', '<=', '', '~', '^', '>=', '>', '*'];
