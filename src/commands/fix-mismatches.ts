import chalk from 'chalk';
import { writeFileSync } from 'fs-extra';
import { EOL } from 'os';
import { relative } from 'path';
import { DependencyType } from '../constants';
import { getDependencyTypes } from './lib/get-dependency-types';
import { getHighestVersion } from './lib/get-highest-version';
import { getMismatchedDependencies } from './lib/get-installations';
import { getWrappers, SourceWrapper } from './lib/get-wrappers';
import { log } from './lib/log';

interface Options {
  dev: boolean;
  filter: RegExp;
  indent: string;
  peer: boolean;
  prod: boolean;
  sources: string[];
}

export const fixMismatches = (dependencyTypes: DependencyType[], filter: RegExp, wrappers: SourceWrapper[]) => {
  const iterator = getMismatchedDependencies(dependencyTypes, wrappers);
  const mismatches = Array.from(iterator).filter(({ name }) => name.search(filter) !== -1);

  mismatches.forEach((installedPackage) => {
    const newest = getHighestVersion(installedPackage.installations.map((installation) => installation.version));
    installedPackage.installations.forEach(({ type, name, source }) => {
      source.contents[type][name] = newest;
    });
  });
};

export const fixMismatchesToDisk = ({ dev, filter, indent, peer, prod, sources }: Options) => {
  const toJson = (wrapper: SourceWrapper) => `${JSON.stringify(wrapper.contents, null, indent)}${EOL}`;
  const dependencyTypes = getDependencyTypes({ dev, peer, prod });
  const wrappers = getWrappers({ sources });
  const allBefore = wrappers.map(toJson);

  fixMismatches(dependencyTypes, filter, wrappers);

  wrappers.forEach((wrapper, i) => {
    const shortPath = relative(process.cwd(), wrapper.filePath);
    const before = allBefore[i];
    const after = toJson(wrapper);
    if (before !== after) {
      writeFileSync(wrapper.filePath, after);
      log(chalk.green('✓'), shortPath);
    } else {
      log(chalk.dim('-', shortPath));
    }
  });
};
