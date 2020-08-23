import chalk from 'chalk';
import { writeFileSync } from 'fs-extra';
import { EOL } from 'os';
import { relative } from 'path';
import { SyncpackConfig } from '../constants';
import { getHighestVersion } from './lib/get-highest-version';
import { getMismatchedDependencies } from './lib/get-installations';
import { getWrappers, SourceWrapper } from './lib/get-wrappers';
import { log } from './lib/log';

type Options = Pick<SyncpackConfig, 'dev' | 'filter' | 'indent' | 'peer' | 'prod' | 'source'>;

export const fixMismatches = (wrappers: SourceWrapper[], options: Options): void => {
  const iterator = getMismatchedDependencies(wrappers, options);
  const mismatches = Array.from(iterator).filter(({ name }) => name.search(options.filter) !== -1);

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

export const fixMismatchesToDisk = (options: Options): void => {
  const { indent, source } = options;
  const toJson = (wrapper: SourceWrapper): string => `${JSON.stringify(wrapper.contents, null, indent)}${EOL}`;
  const wrappers = getWrappers({ source });
  const allBefore = wrappers.map(toJson);

  fixMismatches(wrappers, options);

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
