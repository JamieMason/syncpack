import _ = require('lodash');
import { SORT_AZ, SORT_FIRST } from '../../constants';
import { IManifest } from '../../typings';

export type Format = (manifests: IManifest[]) => IManifest[];
export type ManifestMapper = (manifest: IManifest) => IManifest;

const shortenBugs: ManifestMapper = (manifest: IManifest) => {
  if (manifest.bugs && typeof manifest.bugs === 'object' && manifest.bugs.url) {
    return {
      ...manifest,
      bugs: manifest.bugs.url
    };
  }
  return manifest;
};

const shortenRepository: ManifestMapper = (manifest) => {
  if (
    manifest.repository &&
    typeof manifest.repository === 'object' &&
    manifest.repository.url &&
    manifest.repository.url.indexOf('github.com') !== -1
  ) {
    return {
      ...manifest,
      repository: manifest.repository.url.split('github.com/')[1]
    };
  }
  return manifest;
};

const sortObject = (obj: IManifest) =>
  _(obj)
    .entries()
    .sortBy('0')
    .reduce((next, [key, value]) => ({ ...next, [key]: value }), {});

const sortValue = (value: any) =>
  _.isArray(value)
    ? value.slice(0).sort()
    : _.isObject(value)
      ? sortObject(value)
      : value;

const sortManifest: ManifestMapper = (manifest) => {
  const [first, rest] = _(manifest)
    .entries()
    .sortBy('0')
    .partition(([key, value]) => SORT_FIRST.indexOf(key) !== -1)
    .value();
  const firstSorted = [...first].sort(
    ([keyA], [keyB]) => SORT_FIRST.indexOf(keyA) - SORT_FIRST.indexOf(keyB)
  );
  const restSorted = _(rest)
    .map(([key, value]) => [
      key,
      SORT_AZ.indexOf(key) !== -1 ? sortValue(value) : value
    ])
    .value();
  return _([...firstSorted, ...restSorted]).reduce(
    (obj, [key, value]) => ({ ...obj, [key]: value }),
    {} as IManifest
  );
};

export const format: Format = (manifests) =>
  _.map(manifests, (manifest) =>
    sortManifest(shortenBugs(shortenRepository(manifest)))
  );
