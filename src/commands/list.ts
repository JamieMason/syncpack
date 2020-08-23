import chalk from 'chalk';
import { SyncpackConfig } from '../constants';
import { getDependencies, sortByName } from './lib/get-installations';
import { getWrappers, SourceWrapper } from './lib/get-wrappers';
import { log } from './lib/log';

type Options = Pick<SyncpackConfig, 'dev' | 'filter' | 'peer' | 'prod' | 'source'>;

export const list = (wrappers: SourceWrapper[], options: Options): void => {
  const iterator = getDependencies(wrappers, options);
  const packages = Array.from(iterator).filter(({ name }) => name.search(options.filter) !== -1);

  packages.sort(sortByName).forEach(({ name, installations }) => {
    const versions = installations.map(({ version }) => version);
    const uniques = Array.from(new Set(versions));
    const hasMismatches = uniques.length > 1;
    const uniquesList = uniques.sort().join(', ');
    const message = hasMismatches
      ? chalk`{red ✕ ${name}} {dim.red ${uniquesList}}`
      : chalk`{dim -} {white ${name}} {dim ${uniquesList}}`;
    log(message);
  });
};

export const listFromDisk = (options: Options): void => {
  const wrappers = getWrappers(options);
  list(wrappers, options);
};
