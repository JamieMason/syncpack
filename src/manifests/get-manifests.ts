import { readJson } from 'fs-extra';
import globby = require('globby');
import { IFileDescriptor, IManifestDescriptor } from '../typings';
import { manifestData } from './manifest-data';

type GetDescriptor = (path: string) => Promise<IFileDescriptor>;
type GetDescriptors = (paths: string[]) => Promise<IFileDescriptor[]>;
type FilterDescriptors = (
  descriptors: IFileDescriptor[]
) => IManifestDescriptor[];

const { isManifest } = manifestData;

const getDescriptor: GetDescriptor = (path) =>
  readJson(path).then((data) => ({ data, path }));
const getDescriptors: GetDescriptors = (paths) =>
  Promise.all(paths.map(getDescriptor));
const filterDescriptors: FilterDescriptors = (descriptors) =>
  descriptors
    .filter((descriptor) => isManifest(descriptor.data))
    .map((descriptor) => descriptor as IManifestDescriptor);

export const getManifests = (
  ...patterns: string[]
): Promise<IManifestDescriptor[]> =>
  globby(patterns, { absolute: true })
    .then(getDescriptors)
    .then(filterDescriptors);
