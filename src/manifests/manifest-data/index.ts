import * as _ from 'lodash';
import * as semver from 'semver';
import { DEPENDENCY_TYPES, RANGE_ANY, RANGE_LOOSE } from '../../constants';
import { IDictionary, IManifest } from '../../typings';
import { getNewest, getVersionNumber } from '../../version';
import { format } from './format';

export type GetMismatchedVersions = (manifests: IManifest[]) => IDictionary<string[]>;
export type GetVersions = (manifests: IManifest[]) => IDictionary<string[]>;
export type SetVersion = (name: string, version: string, manifests: IManifest[]) => IManifest[];
export type SetVersionRange = (range: string, manifests: IManifest[]) => IManifest[];
export type SetVersionsToNewestMismatch = (manifests: IManifest[]) => IManifest[];

const isObject = (value: any) => Boolean(value && typeof value === 'object');
const join = ({ name, version }: IDictionary<string>) => `${name}@${version}`;
const gatherDependencies = (manifest: IManifest) =>
  _.chain(DEPENDENCY_TYPES)
    .map((property) => manifest[property])
    .filter(Boolean)
    .flatMap((dependencies) => _.map(dependencies, (version, name) => ({ name, version })))
    .value();

const isManifest = (value: any): boolean =>
  Boolean(
    isObject(value) &&
      'name' in value &&
      ('dependencies' in value || 'devDependencies' in value || 'peerDependencies' in value)
  );

const getMismatchedVersions: GetMismatchedVersions = (manifests) =>
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

const getVersions: GetVersions = (manifests) =>
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

const setVersion: SetVersion = (name, version, manifests) => {
  _(manifests).each((manifest) =>
    _(DEPENDENCY_TYPES)
      .map((property) => manifest[property])
      .filter(Boolean)
      .filter((dependencies) => name in dependencies)
      .each((dependencies) => {
        dependencies[name] = version;
      })
  );
  return manifests;
};

const setVersionRange: SetVersionRange = (range, manifests) => {
  _(manifests).each((manifest) =>
    _(DEPENDENCY_TYPES)
      .map((property) => manifest[property])
      .filter(Boolean)
      .each((dependencies) => {
        _(dependencies).each((version, name) => {
          const versionNumber = getVersionNumber(version);
          if (version !== '*' && semver.validRange(version)) {
            if (range === RANGE_ANY) {
              dependencies[name] = '*';
            } else if (range === RANGE_LOOSE) {
              dependencies[name] =
                semver.major(versionNumber) === 0
                  ? `${semver.major(versionNumber)}.${semver.minor(versionNumber)}.x`
                  : `${semver.major(versionNumber)}.x.x`;
            } else {
              dependencies[name] = `${range}${versionNumber}`;
            }
          }
        });
      })
  );
  return manifests;
};

const setVersionsToNewestMismatch: SetVersionsToNewestMismatch = (manifests) => {
  _(getMismatchedVersions(manifests))
    .map((versions, name) => ({ name, newest: getNewest(versions) }))
    .filter(({ newest }) => typeof newest === 'string')
    .each(({ name, newest }) => {
      setVersion(name, newest as string, manifests);
    });
  return manifests;
};

export const manifestData = {
  format,
  getMismatchedVersions,
  getVersions,
  isManifest,
  setVersion,
  setVersionRange,
  setVersionsToNewestMismatch
};
