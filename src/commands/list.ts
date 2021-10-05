import chalk from 'chalk';
import { SyncpackConfig } from '../constants';
import { getWrappers, SourceWrapper } from './lib/get-wrappers';
import { getDependencies } from './lib/installations/get-dependencies';
import { getMismatchedDependencies } from './lib/installations/get-mismatched-dependencies';
import { sortByName } from './lib/installations/sort-by-name';
import { log } from './lib/log';
import { matchesFilter } from './lib/matches-filter';

type Options = Pick<SyncpackConfig, 'dev' | 'filter' | 'peer' | 'prod' | 'matchRanges' | 'source' | 'versionGroups'>;

export const list = (wrappers: SourceWrapper[], options: Options): void => {
  const packages = Array.from(getDependencies(wrappers, options)).filter(matchesFilter(options));
  const mismatches = Array.from(getMismatchedDependencies(wrappers, options)).filter(matchesFilter(options));

  packages.sort(sortByName).forEach(({ name, installations }) => {
    const versions = installations.map(({ version }) => version);
    const uniques = Array.from(new Set(versions));
    const mismatch = mismatches.find((mismatch) => mismatch.name === name);
    const hasMismatches = mismatch !== undefined;
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
