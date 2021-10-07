import { getDependencies, Installation } from './get-dependencies';
import { SyncpackConfig } from '../../../constants';
import { SourceWrapper } from '../get-wrappers';
import { matchesFilter as createMatchesFilter } from '../matches-filter';

type Options = Pick<SyncpackConfig, 'dev' | 'peer' | 'prod' | 'resolutions' | 'overrides' | 'filter'>;

export function* getInstallations(wrappers: SourceWrapper[], options: Options): Generator<Installation> {
  const dependenciesIterator = getDependencies(wrappers, options);
  const matchesFilter = createMatchesFilter(options);

  for (const installedPackage of dependenciesIterator) {
    if (matchesFilter(installedPackage)) {
      for (const installation of installedPackage.installations) {
        yield installation;
      }
    }
  }
}
