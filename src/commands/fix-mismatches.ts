import chalk from 'chalk';
import { writeFileSync } from 'fs-extra';
import { EOL } from 'os';
import { relative } from 'path';
import { CWD, SyncpackConfig } from '../constants';
import { getHighestVersion } from './lib/get-highest-version';
import { getWrappers, SourceWrapper } from './lib/get-wrappers';
import { getMismatchedDependencies } from './lib/installations/get-mismatched-dependencies';
import { log } from './lib/log';
import { matchesFilter } from './lib/matches-filter';

type Options = Pick<SyncpackConfig, 'dev' | 'filter' | 'indent' | 'peer' | 'prod' | 'source' | 'versionGroups'>;

const getWorkspaceVersion = (name: string, wrappers: SourceWrapper[]): string | null => {
  const local = wrappers.find((wrapper) => wrapper.contents.name === name);
  return local?.contents?.version || null;
};

export const fixMismatches = (wrappers: SourceWrapper[], options: Options): void => {
  const iterator = getMismatchedDependencies(wrappers, options);
  const mismatches = Array.from(iterator).filter(matchesFilter(options));

  mismatches.forEach((installedPackage) => {
    const versions = installedPackage.installations.map((installation) => installation.version);
    const nextVersion = getWorkspaceVersion(installedPackage.name, wrappers) || getHighestVersion(versions);
    if (nextVersion !== null) {
      installedPackage.installations.forEach(({ type, name, source }) => {
        const dependencies = source.contents[type];
        if (dependencies) {
          dependencies[name] = nextVersion;
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
    const shortPath = relative(CWD, wrapper.filePath);
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
