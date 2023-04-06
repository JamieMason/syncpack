import { pipe } from 'tightrope/fn/pipe';
import { andThen } from 'tightrope/result/and-then';
import { map } from 'tightrope/result/map';
import { unwrap } from 'tightrope/result/unwrap';
import type { Disk } from '../lib/disk';
import { disk as defaultDisk } from '../lib/disk';
import type { Syncpack } from '../types';
import { $R } from './$R';
import { getConfig } from './get-config';
import { getGroups } from './get-groups';
import type { SemverGroup } from './get-groups/semver-group';
import type { VersionGroup } from './get-groups/version-group';
import { getPackageJsonFiles } from './get-package-json-files';
import type { PackageJsonFile } from './get-package-json-files/package-json-file';

export interface Context {
  config: Syncpack.Config.Private;
  disk: Disk;
  isInvalid: boolean;
  packageJsonFiles: PackageJsonFile[];
  semverGroups: SemverGroup[];
  versionGroups: VersionGroup[];
}

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
  return pipe(
    // merge CLI options, .syncpackrc contents, and default config
    getConfig(disk, program),
    andThen((config) =>
      pipe(
        // get the package.json file which match the globs in config
        getPackageJsonFiles(disk, config),
        andThen((packageJsonFiles) =>
          pipe(
            // allocate dependencies into semver and version groups
            getGroups(config, packageJsonFiles),
            // combine everything into the final config
            map(({ semverGroups, versionGroups }) => ({
              config,
              disk,
              isInvalid: false,
              packageJsonFiles,
              semverGroups,
              versionGroups,
            })),
          ),
        ),
      ),
    ),
    // if anything errored at any stage, log it when in verbose mode
    $R.tapErrVerbose,
    // throw if anything errored, can't do anything without this data
    unwrap,
  );
}
