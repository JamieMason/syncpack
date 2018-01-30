import { DEFAULT_PATTERN } from '../constants';
import { IDictionary, IManifest, IManifestDescriptor } from '../typings';
import { getManifests } from './get-manifests';
import { manifestData } from './manifest-data';

export type GetMismatchedVersions = (pattern: string) => Promise<IDictionary<string[]>>;
export type GetVersions = (pattern: string) => Promise<IDictionary<string[]>>;
export type SetVersion = (name: string, version: string, pattern: string) => Promise<IManifest[]>;
export type SetVersionRange = (range: string, pattern: string) => Promise<IManifest[]>;
export type SetVersionsToNewestMismatch = (pattern: string) => Promise<IManifest[]>;

const unwrap = (descriptors: IManifestDescriptor[]) => descriptors.map((descriptor) => descriptor.data);

export const getVersions: GetVersions = (pattern = DEFAULT_PATTERN) =>
  getManifests(pattern)
    .then(unwrap)
    .then(manifestData.getVersions);

export const getMismatchedVersions: GetMismatchedVersions = (pattern = DEFAULT_PATTERN) =>
  getManifests(pattern)
    .then(unwrap)
    .then(manifestData.getMismatchedVersions);

export const setVersion: SetVersion = (name, version, pattern = DEFAULT_PATTERN) =>
  getManifests(pattern)
    .then(unwrap)
    .then(manifestData.setVersion.bind(null, name, version));

export const setVersionRange: SetVersionRange = (range, pattern = DEFAULT_PATTERN) =>
  getManifests(pattern)
    .then(unwrap)
    .then(manifestData.setVersionRange.bind(null, range));

export const setVersionsToNewestMismatch: SetVersionsToNewestMismatch = (pattern = DEFAULT_PATTERN) =>
  getManifests(pattern)
    .then(unwrap)
    .then(manifestData.setVersionsToNewestMismatch);
