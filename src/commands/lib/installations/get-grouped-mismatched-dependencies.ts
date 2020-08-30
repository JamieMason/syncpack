import { SyncpackConfig } from '../../../constants';
import { SourceWrapper } from '../get-wrappers';
import { getDependencies, InstalledPackage } from './get-dependencies';

export function* getGroupedMismatchedDependencies(
  wrappers: SourceWrapper[],
  options: Pick<SyncpackConfig, 'dev' | 'peer' | 'prod' | 'versionGroups'>,
): Generator<InstalledPackage> {
  const iterator = getDependencies(wrappers, options);
  for (const installedPackage of iterator) {
    if (installedPackage.installations.length > 1) {
      const ungroupedInstallations = [];
      for (const versionGroup of options.versionGroups) {
        let hasMismatchesInThisGroup = false;
        const groupInstallations = [];
        const dependencyIsInThisGroup = versionGroup.dependencies.includes(installedPackage.name);
        if (dependencyIsInThisGroup) {
          for (const installation of installedPackage.installations) {
            const packageIsInThisGroup = versionGroup.packages.includes(String(installation.source.contents.name));
            if (packageIsInThisGroup) {
              if (!hasMismatchesInThisGroup) {
                const [lastItem] = groupInstallations.slice(-1);
                if (lastItem && lastItem.version !== installation.version) {
                  hasMismatchesInThisGroup = true;
                }
              }
              groupInstallations.push(installation);
            } else {
              ungroupedInstallations.push(installation);
            }
          }
        } else {
          ungroupedInstallations.push(...installedPackage.installations);
        }
        if (hasMismatchesInThisGroup) {
          yield {
            installations: groupInstallations,
            name: installedPackage.name,
          };
        }
      }
      const len = ungroupedInstallations.length;
      if (len > 1) {
        for (let i = 1; i < len; i++) {
          if (ungroupedInstallations[i].version !== ungroupedInstallations[i - 1].version) {
            yield {
              installations: ungroupedInstallations,
              name: installedPackage.name,
            };
            break;
          }
        }
      }
    }
  }
}
