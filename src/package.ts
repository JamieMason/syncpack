import * as _ from 'lodash';
import * as semver from 'semver';
import {
  GetMismatchedVersions,
  GetNewest,
  GetVersionNumber,
  GetVersionRange,
  GetVersions,
  IDictionary,
  IPackageJson,
  IPackageJsonKey,
  SetVersion,
  SetVersionRange,
  SetVersionsToNewestMismatch,
  SortBySemver
} from './typings';
import { getNewest, sortBySemver } from './version';

const isValid = (version: string) => semver.valid(version) !== null;
const join = ({ name, version }: IDictionary<string>) => `${name}@${version}`;

export const getVersions: GetVersions = (property, manifests) =>
  _.chain(manifests)
    .map(property)
    .flatMap((dependencies) => _.map(dependencies, (version, name) => ({ name, version })))
    .uniqBy(join)
    .sortBy(join)
    .groupBy('name')
    .reduce((index: IDictionary<string[]>, dependencies, name) => {
      index[name] = dependencies.map(_.property('version'));
      return index;
    }, {})
    .value();

export const getMismatchedVersions: GetMismatchedVersions = (property, manifests) =>
  _.chain(manifests)
    .map(property)
    .flatMap((dependencies) => _.map(dependencies, (version, name) => ({ name, version })))
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

export const setVersion: SetVersion = (property, name, version, manifests) => {
  _(manifests)
    .map(property)
    .filter((dependencies) => name in dependencies)
    .each((dependencies) => {
      dependencies[name] = version;
    });
  return manifests;
};

export const setVersionRange: SetVersionRange = (property, range, manifests) => {
  _(manifests)
    .map(property)
    .each((dependencies) => {
      _(dependencies).each((version, name) => {
        if (isValid(version)) {
          dependencies[name] = `${range}${semver.clean(version)}`;
        }
      });
    });
  return manifests;
};

export const setVersionsToNewestMismatch: SetVersionsToNewestMismatch = (property, manifests) => {
  _(getMismatchedVersions(property, manifests))
    .map((versions, name) => ({ name, newest: getNewest(versions) }))
    .filter(({ newest }) => typeof newest === 'string')
    .each(({ name, newest }) => {
      setVersion(property, name, newest as string, manifests);
    });
  return manifests;
};
