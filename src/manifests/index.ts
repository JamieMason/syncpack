import { IDictionary, IManifest } from '../typings';
import { getManifests } from './get-manifests';
import { manifestData } from './manifest-data';

export type GetMismatchedVersions = (pattern: string) => Promise<IDictionary<string[]>>;
export type GetVersions = (pattern: string) => Promise<IDictionary<string[]>>;
export type SetVersion = (name: string, version: string, pattern: string) => Promise<IManifest[]>;
export type SetVersionRange = (range: string, pattern: string) => Promise<IManifest[]>;
export type SetVersionsToNewestMismatch = (pattern: string) => Promise<IManifest[]>;

export const getVersions: GetVersions = (pattern) => getManifests(pattern).then(manifestData.getVersions);

export const getMismatchedVersions: GetMismatchedVersions = (pattern) =>
  getManifests(pattern).then(manifestData.getMismatchedVersions);

export const setVersion: SetVersion = (name, version, pattern) =>
  getManifests(pattern).then(manifestData.setVersion.bind(null, name, version));

export const setVersionRange: SetVersionRange = (range, pattern) =>
  getManifests(pattern).then(manifestData.setVersionRange.bind(null, range));

export const setVersionsToNewestMismatch: SetVersionsToNewestMismatch = (pattern) =>
  getManifests(pattern).then(manifestData.setVersionsToNewestMismatch);
