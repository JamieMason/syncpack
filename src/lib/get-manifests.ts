import { readJson } from 'fs-extra';
import * as globby from 'globby';
import { isPackageJson } from '../package';
import { IPackageJson } from '../typings';

export const getManifests = (...patterns: string[]): Promise<IPackageJson[]> =>
  globby(patterns, { absolute: true })
    .then((paths) => Promise.all(paths.map((path) => readJson(path))))
    .then((files) => files.filter(isPackageJson));
