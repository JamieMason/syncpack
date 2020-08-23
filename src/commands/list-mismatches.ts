import chalk from 'chalk';
import { SyncpackConfig } from '../constants';
import { getMismatchedDependencies, InstalledPackage, sortByName } from './lib/get-installations';
import { getWrappers, SourceWrapper } from './lib/get-wrappers';
import { log } from './lib/log';

type Options = Pick<SyncpackConfig, 'dev' | 'filter' | 'peer' | 'prod' | 'source'>;

export const listMismatches = (wrappers: SourceWrapper[], options: Options): InstalledPackage[] => {
  const iterator = getMismatchedDependencies(wrappers, options);
  const mismatches = Array.from(iterator).filter(({ name }) => name.search(options.filter) !== -1);

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
