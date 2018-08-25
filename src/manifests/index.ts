import { writeJson } from '../lib/write-json';
import { IDictionary, IManifest, IManifestDescriptor } from '../typings';
import { getManifests } from './get-manifests';
import { manifestData } from './manifest-data';

export type Format = (...sources: string[]) => Promise<IManifestDescriptor[]>;
export type GetMismatchedVersions = (...sources: string[]) => Promise<IDictionary<string[]>>;
export type GetVersions = (...sources: string[]) => Promise<IDictionary<string[]>>;
export type SetVersion = (name: string, version: string, ...sources: string[]) => Promise<IManifestDescriptor[]>;
export type SetVersionRange = (range: string, ...sources: string[]) => Promise<IManifestDescriptor[]>;
export type SetVersionsToNewestMismatch = (...sources: string[]) => Promise<IManifestDescriptor[]>;

const unwrap = (descriptors: IManifestDescriptor[]) => descriptors.map((descriptor) => descriptor.data);

const writeDescriptors = (descriptors: IManifestDescriptor[]): Promise<IManifestDescriptor[]> =>
  Promise.all(descriptors.map((descriptor) => writeJson(descriptor.path, descriptor.data))).then(() => descriptors);

export const format: Format = (...sources) =>
  getManifests(...sources)
    .then((descriptors) => {
      const data = unwrap(descriptors);
      const nextData = manifestData.format(data);
      return descriptors.map((descriptor, i) => ({
        data: nextData[i],
        path: descriptor.path
      }));
    })
    .then(writeDescriptors);

export const getMismatchedVersions: GetMismatchedVersions = (...sources) =>
  getManifests(...sources)
    .then(unwrap)
    .then(manifestData.getMismatchedVersions);

export const getVersions: GetVersions = (...sources) =>
  getManifests(...sources)
    .then(unwrap)
    .then(manifestData.getVersions);

export const setVersion: SetVersion = (name, version, ...sources) =>
  getManifests(...sources).then((descriptors) => {
    manifestData.setVersion(name, version, unwrap(descriptors));
    return descriptors;
  });

export const setVersionRange: SetVersionRange = (range, ...sources) =>
  getManifests(...sources)
    .then((descriptors) => {
      manifestData.setVersionRange(range, unwrap(descriptors));
      return descriptors;
    })
    .then(writeDescriptors);

export const setVersionsToNewestMismatch: SetVersionsToNewestMismatch = (...sources) =>
  getManifests(...sources)
    .then((descriptors) => {
      const data = unwrap(descriptors);
      const nextData = manifestData.setVersionsToNewestMismatch(data);
      return descriptors.map((descriptor, i) => ({
        data: nextData[i],
        path: descriptor.path
      }));
    })
    .then(writeDescriptors);
