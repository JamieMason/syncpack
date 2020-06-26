import chalk from 'chalk';
import { DependencyType } from '../constants';
import { getDependencyTypes } from './lib/get-dependency-types';
import { getDependencies, sortByName } from './lib/get-installations';
import { getWrappers, SourceWrapper } from './lib/get-wrappers';
import { log } from './lib/log';

interface Options {
  dev: boolean;
  filter: RegExp[];
  peer: boolean;
  prod: boolean;
  sources: string[];
}

export const list = (dependencyTypes: DependencyType[], filter: RegExp[], wrappers: SourceWrapper[]): void => {
  const iterator = getDependencies(dependencyTypes, wrappers);
  const packages = Array.from(iterator).filter(({ name }) =>
    filter.reduce((matchesFilter: boolean, f) => matchesFilter || name.search(f) !== -1, false),
  );

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

export const listFromDisk = ({ dev, filter, peer, prod, sources: sources }: Options): void => {
  const dependencyTypes = getDependencyTypes({ dev, peer, prod });
  const wrappers = getWrappers({ sources });

  list(dependencyTypes, filter, wrappers);
};
