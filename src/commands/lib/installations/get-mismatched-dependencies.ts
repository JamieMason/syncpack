import { SyncpackConfig } from '../../../constants';
import { SourceWrapper } from '../get-wrappers';
import { getDependencies, InstalledPackage } from './get-dependencies';
import { getGroupedMismatchedDependencies } from './get-grouped-mismatched-dependencies';

function* getUngroupedMismatchedDependencies(
  wrappers: SourceWrapper[],
  options: Pick<SyncpackConfig, 'dev' | 'peer' | 'prod' | 'versionGroups'>,
): Generator<InstalledPackage> {
  const iterator = getDependencies(wrappers, options);
  for (const installedPackage of iterator) {
    const { installations } = installedPackage;
    const len = installations.length;
    if (len > 1) {
      for (let i = 1; i < len; i++) {
        if (installations[i].version !== installations[i - 1].version) {
          yield installedPackage;
          break;
        }
      }
    }
  }
}

export function* getMismatchedDependencies(
  wrappers: SourceWrapper[],
  options: Pick<SyncpackConfig, 'dev' | 'peer' | 'prod' | 'versionGroups'>,
): Generator<InstalledPackage> {
  const iterator = options.versionGroups.length
    ? getGroupedMismatchedDependencies(wrappers, options)
    : getUngroupedMismatchedDependencies(wrappers, options);
  for (const installedPackage of iterator) {
    yield installedPackage;
  }
}
