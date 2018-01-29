import { getManifests } from './lib/get-manifests';
import {
  getMismatchedPackageVersions,
  getPackageVersions,
  setPackageVersion,
  setPackageVersionRange,
  setPackageVersionsToNewestMismatch
} from './package';
import {
  GetFileVersions,
  GetMismatchedFileVersions,
  SetFileVersion,
  SetFileVersionRange,
  SetFileVersionsToNewestMismatch
} from './typings';

export const getFileVersions: GetFileVersions = (pattern) => getManifests(pattern).then(getPackageVersions);

export const getMismatchedFileVersions: GetMismatchedFileVersions = (pattern) =>
  getManifests(pattern).then(getMismatchedPackageVersions);

export const setFileVersion: SetFileVersion = (name, version, pattern) =>
  getManifests(pattern).then(setPackageVersion.bind(null, name, version));

export const setFileVersionRange: SetFileVersionRange = (range, pattern) =>
  getManifests(pattern).then(setPackageVersionRange.bind(null, range));

export const setFileVersionsToNewestMismatch: SetFileVersionsToNewestMismatch = (pattern) =>
  getManifests(pattern).then(setPackageVersionsToNewestMismatch);
