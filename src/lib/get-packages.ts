import { readJsonSync } from 'fs-extra';
import globby = require('globby');
import { join, resolve } from 'path';
import { OPTION_SOURCES } from '../constants';
import { CommanderApi, IManifestDescriptor } from '../typings';

export const getSources = (program: CommanderApi): string[] => {
  if (program.source && program.source.length) {
    return program.source;
  }

  return getYarnWorkspaces() || getLernaPackages() || OPTION_SOURCES.default;
};

export const getPackages = (program: CommanderApi): IManifestDescriptor[] =>
  globby.sync(getSources(program)).map((filePath) => ({
    data: readJsonSync(filePath),
    path: filePath
  }));

const getYarnWorkspaces = (): string[] | null => {
  const rootPackageJson = resolve(process.cwd(), 'package.json');
  const pkgJson = readJsonSync(rootPackageJson, { throws: false });
  if (pkgJson && pkgJson.workspaces && pkgJson.workspaces.length) {
    return pkgJson.workspaces.map((glob: string) => join(glob, 'package.json'));
  }
  return null;
};

const getLernaPackages = (): string[] | null => {
  const lernaPath = resolve(process.cwd(), 'lerna.json');
  const lerna = readJsonSync(lernaPath, { throws: false });
  if (lerna && lerna.packages && lerna.packages.length) {
    return lerna.packages.map((glob: string) => join(glob, 'package.json'));
  }
  return null;
};
