import type { Disk } from '../disk';
import { disk as defaultDisk } from '../disk';
import { getAllInstances } from './get-all-instances';
import { getConfig } from './get-config';
import type { Config } from './get-config/config';
import type { InternalConfig } from './get-config/internal-config';
import type { SemverGroup, VersionGroup } from './get-groups';
import { getSemverGroups, getVersionGroups } from './get-groups';
import { getPackageJsonFiles } from './get-package-json-files';
import type { PackageJsonFile } from './get-package-json-files/package-json-file';

export type Context = Omit<InternalConfig, 'semverGroups' | 'versionGroups'> & {
  disk: Disk;
  isInvalid: boolean;
  packageJsonFiles: PackageJsonFile[];
  semverGroups: SemverGroup.Any[];
  versionGroups: VersionGroup.Any[];
};

/**
 * Every command in syncpack should accept the return value of this function as
 * its input.
 *
 * The aim here is to move all disk activity to a single place, so
 * that the majority of syncpack and its tests don't have to deal with the file
 * system and can focus solely on transformation logic.
 */
export function getContext(
  program: Partial<Config.All>,
  disk: Disk = defaultDisk,
): Context {
  const config = getConfig(disk, program);
  const packageJsonFiles = getPackageJsonFiles(disk, config);
  const instances = getAllInstances(packageJsonFiles);
  const semverGroups = getSemverGroups(config, instances);
  const versionGroups = getVersionGroups(config, instances);

  return {
    ...config,
    disk,
    isInvalid: false,
    packageJsonFiles,
    semverGroups,
    versionGroups,
  };
}
