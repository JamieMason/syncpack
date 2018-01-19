import * as _ from 'lodash';
import { DEPENDENCY_TYPES } from './constants';
import { getMismatchedVersions as getSingleMismatchedVersions, getVersions as getSingleVersions } from './package';
import { IDictionary, IPackageJson } from './typings';

export type BatchGetVersions = (manifests: IPackageJson[]) => IDictionary<string[]>;
export type BatchGetMismatchedVersions = (manifests: IPackageJson[]) => IDictionary<string[]>;
export type BatchSetVersion = (name: string, version: string, manifests: IPackageJson[]) => IPackageJson[];
export type BatchSetVersionRange = (range: string, manifests: IPackageJson[]) => IPackageJson[];
export type BatchSetVersionsToNewestMismatch = (manifests: IPackageJson[]) => IPackageJson[];

export const getMismatchedVersions: BatchGetMismatchedVersions = (manifests) =>
  _.chain(DEPENDENCY_TYPES)
    .map((property) => getSingleMismatchedVersions(property, manifests))
    .reduce((index: IDictionary<string[]>, mismatches) => {
      _(mismatches).each((versions, name) => {
        index[name] = _.chain(index[name] || [])
          .concat(versions)
          .uniq()
          .value();
      });
      return index;
    }, {})
    .value();

export const getVersions: BatchGetVersions = (manifests) => ({});
export const setVersion: BatchSetVersion = (name, version, manifests) => [];
export const setVersionRange: BatchSetVersionRange = (range, manifests) => [];
export const setVersionsToNewestMismatch: BatchSetVersionsToNewestMismatch = (manifests) => [];
