import type { Disk } from '../lib/disk';
import { disk as defaultDisk } from '../lib/disk';
import type { Syncpack } from '../types';
import { getAllInstances } from './get-all-instances';
import { getConfig } from './get-config';
import { getSemverGroups } from './get-groups/get-semver-groups';
import { getVersionGroups } from './get-groups/get-version-groups';
import type { SemverGroup } from './get-groups/semver-group';
import type { VersionGroup } from './get-groups/version-group';
import { getPackageJsonFiles } from './get-package-json-files';
import type { PackageJsonFile } from './get-package-json-files/package-json-file';

export type Context = Omit<
  Syncpack.Config.Private,
  'semverGroups' | 'versionGroups'
> & {
  disk: Disk;
  isInvalid: boolean;
  packageJsonFiles: PackageJsonFile[];
  semverGroups: SemverGroup[];
  versionGroups: VersionGroup[];
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
  program: Partial<Syncpack.Config.Cli>,
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
