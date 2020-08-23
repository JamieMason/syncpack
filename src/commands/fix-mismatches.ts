import chalk from 'chalk';
import { writeFileSync } from 'fs-extra';
import { EOL } from 'os';
import { relative } from 'path';
import { DependencyType, SyncpackConfig } from '../constants';
import { getDependencyTypes } from './lib/get-dependency-types';
import { getHighestVersion } from './lib/get-highest-version';
import { getMismatchedDependencies } from './lib/get-installations';
import { getWrappers, SourceWrapper } from './lib/get-wrappers';
import { log } from './lib/log';

type Options = Pick<SyncpackConfig, 'dev' | 'filter' | 'indent' | 'peer' | 'prod' | 'source'>;

export const fixMismatches = (dependencyTypes: DependencyType[], filter: RegExp, wrappers: SourceWrapper[]): void => {
  const iterator = getMismatchedDependencies(dependencyTypes, wrappers);
  const mismatches = Array.from(iterator).filter(({ name }) => name.search(filter) !== -1);

  mismatches.forEach((installedPackage) => {
    const versions = installedPackage.installations.map((installation) => installation.version);
    const newest = getHighestVersion(versions);
    if (newest !== null) {
      installedPackage.installations.forEach(({ type, name, source }) => {
        const dependencies = source.contents[type];
        if (dependencies) {
          dependencies[name] = newest;
        }
      });
    }
  });
};

export const fixMismatchesToDisk = ({ dev, filter, indent, peer, prod, source }: Options): void => {
  const toJson = (wrapper: SourceWrapper): string => `${JSON.stringify(wrapper.contents, null, indent)}${EOL}`;
  const dependencyTypes = getDependencyTypes({ dev, peer, prod });
  const wrappers = getWrappers({ source });
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
