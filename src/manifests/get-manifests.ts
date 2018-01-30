import { readJson } from 'fs-extra';
import * as globby from 'globby';
import { IManifest } from '../typings';
import { manifestData } from './manifest-data';

const { isManifest } = manifestData;

export const getManifests = (...patterns: string[]): Promise<IManifest[]> =>
  globby(patterns, { absolute: true })
    .then((paths) => Promise.all(paths.map((path) => readJson(path))))
    .then((files) => files.filter(isManifest));
