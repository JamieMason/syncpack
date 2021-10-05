import chalk from 'chalk';
import { SyncpackConfig } from '../constants';
import { getWrappers, SourceWrapper } from './lib/get-wrappers';
import { InstalledPackage } from './lib/installations/get-dependencies';
import { getMismatchedDependencies } from './lib/installations/get-mismatched-dependencies';
import { sortByName } from './lib/installations/sort-by-name';
import { log } from './lib/log';
import { matchesFilter } from './lib/matches-filter';

type Options = Pick<SyncpackConfig, 'dev' | 'filter' | 'peer' | 'prod' | 'matchRanges' | 'source' | 'versionGroups'>;

export const listMismatches = (wrappers: SourceWrapper[], options: Options): InstalledPackage[] => {
  const iterator = getMismatchedDependencies(wrappers, options);
  const mismatches = Array.from(iterator).filter(matchesFilter(options));

  mismatches.sort(sortByName).forEach(({ name, installations }) => {
    log(chalk`{red ✕ ${name}}`);
    installations.forEach(({ source, type, version }) => {
      log(chalk`{dim -} ${version} {dim in ${type} of ${source.contents.name}}`);
    });
  });

  return mismatches;
};

export const listMismatchesFromDisk = (options: Options): void | never => {
  const wrappers = getWrappers(options);
  const mismatches = listMismatches(wrappers, options);

  if (mismatches.length > 0) {
    process.exit(1);
  }
};
