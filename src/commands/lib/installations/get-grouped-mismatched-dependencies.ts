import { SyncpackConfig, VersionGroup } from '../../../constants';
import { SourceWrapper } from '../get-wrappers';
import { getDependencies, Installation, InstalledPackage } from './get-dependencies';

const groupContainsDependency = (versionGroup: VersionGroup, installedPackage: InstalledPackage) =>
  versionGroup.dependencies.includes(installedPackage.name);

const groupContainsPackage = (versionGroup: VersionGroup, installation: Installation) =>
  versionGroup.packages.includes(`${installation.source.contents.name}`);

const hasDifferentVersionToPreviousSibling = (installation: Installation, i: number, all: Installation[]) =>
  i > 0 && installation.version !== all[i - 1].version;

export function* getGroupedMismatchedDependencies(
  wrappers: SourceWrapper[],
  options: Pick<SyncpackConfig, 'dev' | 'peer' | 'prod' | 'resolutions' | 'overrides' | 'versionGroups'>,
): Generator<InstalledPackage> {
  const iterator = getDependencies(wrappers, options);
  const installedPackages = Array.from(iterator);

  const groupedDependenciesByGroup = options.versionGroups.map((versionGroup) =>
    installedPackages
      .filter((installedPackage) => groupContainsDependency(versionGroup, installedPackage))
      .map(({ installations, name }) => ({
        installations: installations.filter((installation) => groupContainsPackage(versionGroup, installation)),
        name,
      }))
      .filter(({ installations }) => installations.length > 0),
  );

  const ungroupedDependencies = installedPackages
    .map((installedPackage) => {
      const { installations, name } = installedPackage;
      return {
        installations: installations.filter((installation) =>
          options.versionGroups.every(
            (versionGroup) =>
              !groupContainsDependency(versionGroup, installedPackage) ||
              !groupContainsPackage(versionGroup, installation),
          ),
        ),
        name,
      };
    })
    .filter(({ installations }) => installations.length > 0);

  const groupedMismatches = groupedDependenciesByGroup
    .map((groupedDependencies) =>
      groupedDependencies.filter((installedPackage) =>
        installedPackage.installations.some(hasDifferentVersionToPreviousSibling),
      ),
    )
    .reduce((flat, next) => flat.concat(next), []);

  const ungroupedMismatches = ungroupedDependencies.filter((installedPackage) =>
    installedPackage.installations.some(hasDifferentVersionToPreviousSibling),
  );

  const allMismatches = groupedMismatches.concat(ungroupedMismatches);

  for (const installedPackage of allMismatches) {
    yield installedPackage;
  }
}
