import * as _ from 'lodash';
import * as semver from 'semver';
import { DEPENDENCY_TYPES } from './constants';
import {
  GetMismatchedPackageVersions,
  GetPackageVersions,
  IDictionary,
  IPackageJson,
  SetPackageVersion,
  SetPackageVersionRange,
  SetPackageVersionsToNewestMismatch
} from './typings';
import { getNewest } from './version';

const isValid = (version: string) => semver.valid(version) !== null;
const join = ({ name, version }: IDictionary<string>) => `${name}@${version}`;
const gatherDependencies = (manifest: IPackageJson) =>
  _.chain(DEPENDENCY_TYPES)
    .map((property) => manifest[property])
    .flatMap((dependencies) => _.map(dependencies, (version, name) => ({ name, version })))
    .value();

export const isPackageJson = (value: any): boolean =>
  Boolean(
    value &&
      typeof value === 'object' &&
      'dependencies' in value &&
      'devDependencies' in value &&
      'peerDependencies' in value
  );

export const getMismatchedPackageVersions: GetMismatchedPackageVersions = (manifests) =>
  _.chain(manifests)
    .map(gatherDependencies)
    .flatten()
    .uniqBy(join)
    .sortBy(join)
    .groupBy('name')
    .reduce((index: IDictionary<string[]>, dependencies, name) => {
      if (dependencies.length > 1) {
        index[name] = dependencies.map(_.property('version'));
      }
      return index;
    }, {})
    .value();

export const getPackageVersions: GetPackageVersions = (manifests) =>
  _.chain(manifests)
    .map(gatherDependencies)
    .flatten()
    .uniqBy(join)
    .sortBy(join)
    .groupBy('name')
    .reduce((index: IDictionary<string[]>, dependencies, name) => {
      index[name] = dependencies.map(_.property('version'));
      return index;
    }, {})
    .value();

export const setPackageVersion: SetPackageVersion = (name, version, manifests) => {
  _(manifests).each((manifest) =>
    _(DEPENDENCY_TYPES)
      .map((property) => manifest[property])
      .filter((dependencies) => name in dependencies)
      .each((dependencies) => {
        dependencies[name] = version;
      })
  );
  return manifests;
};

export const setPackageVersionRange: SetPackageVersionRange = (range, manifests) => {
  _(manifests).each((manifest) =>
    _(DEPENDENCY_TYPES)
      .map((property) => manifest[property])
      .each((dependencies) => {
        _(dependencies).each((version, name) => {
          if (isValid(version)) {
            dependencies[name] = `${range}${semver.clean(version)}`;
          }
        });
      })
  );
  return manifests;
};

export const setPackageVersionsToNewestMismatch: SetPackageVersionsToNewestMismatch = (manifests) => {
  _(getMismatchedPackageVersions(manifests))
    .map((versions, name) => ({ name, newest: getNewest(versions) }))
    .filter(({ newest }) => typeof newest === 'string')
    .each(({ name, newest }) => {
      setPackageVersion(name, newest as string, manifests);
    });
  return manifests;
};
