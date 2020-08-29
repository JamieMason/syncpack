import { SyncpackConfig } from '../../../constants';
import { SourceWrapper } from '../get-wrappers';
import { getDependencies, InstalledPackage } from './get-dependencies';

export function* getMismatchedDependencies(
  wrappers: SourceWrapper[],
  options: Pick<SyncpackConfig, 'dev' | 'peer' | 'prod'>,
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
