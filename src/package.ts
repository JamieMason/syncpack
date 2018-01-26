import * as _ from 'lodash';
import * as semver from 'semver';
import { DEPENDENCY_TYPES } from './constants';
import {
  GetMismatchedVersions,
  GetVersions,
  IDictionary,
  IPackageJson,
  SetVersion,
  SetVersionRange,
  SetVersionsToNewestMismatch
} from './typings';
import { getNewest } from './version';

const isValid = (version: string) => semver.valid(version) !== null;
const join = ({ name, version }: IDictionary<string>) => `${name}@${version}`;
const gatherDependencies = (manifest: IPackageJson) =>
  _.chain(DEPENDENCY_TYPES)
    .map((property) => manifest[property])
    .flatMap((dependencies) => _.map(dependencies, (version, name) => ({ name, version })))
    .value();

export const getMismatchedVersions: GetMismatchedVersions = (manifests) =>
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

export const getVersions: GetVersions = (manifests) =>
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

export const setVersion: SetVersion = (name, version, manifests) => {
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

export const setVersionRange: SetVersionRange = (range, manifests) => {
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

export const setVersionsToNewestMismatch: SetVersionsToNewestMismatch = (manifests) => {
  _(getMismatchedVersions(manifests))
    .map((versions, name) => ({ name, newest: getNewest(versions) }))
    .filter(({ newest }) => typeof newest === 'string')
    .each(({ name, newest }) => {
      setVersion(name, newest as string, manifests);
    });
  return manifests;
};
