import { pipe } from 'tightrope/fn/pipe';
import type { Result } from 'tightrope/result';
import { fromTry } from 'tightrope/result/from-try';
import { mapErr } from 'tightrope/result/map-err';
import { BaseError } from '../../lib/error';
import { verbose } from '../../lib/log';
import { sortByName } from '../../lib/sort-by-name';
import type { Syncpack } from '../../types';
import type { PackageJsonFile } from '../get-package-json-files/package-json-file';
import { SemverGroup } from './semver-group';
import { VersionGroup } from './version-group';

interface GroupsByPropName {
  semverGroups: SemverGroup[];
  versionGroups: VersionGroup[];
}

export function getGroups(
  config: Syncpack.Config.Private,
  packageJsonFiles: PackageJsonFile[],
): Result<GroupsByPropName> {
  const ERR_CREATING_GROUPS = 'Error creating semver and version groups';
  return pipe(
    fromTry(() => unsafeGetGroups(config, packageJsonFiles)),
    mapErr(BaseError.map(ERR_CREATING_GROUPS)),
  );
}

function unsafeGetGroups(
  config: Syncpack.Config.Private,
  packageJsonFiles: PackageJsonFile[],
): GroupsByPropName {
  type Key = keyof typeof groupsByName;
  const groupsByName = {
    semverGroups: config.semverGroups.map(
      (group) => new SemverGroup(config, group, packageJsonFiles),
    ),
    versionGroups: config.versionGroups.map(
      (group) => new VersionGroup(config, group, packageJsonFiles),
    ),
  };
  const groupNames = Object.keys(groupsByName) as Key[];
  const instances = packageJsonFiles
    .flatMap((pkg) => pkg.getInstances())
    .sort(sortByName);

  instances.forEach((instance) => {
    const { name, pkgName } = instance;
    groupNames.forEach((key) => {
      for (const group of groupsByName[key]) {
        if (group.canAdd(instance)) {
          group.add(instance);
          return;
        }
      }
      throw new Error(`${name} in ${pkgName} did not match any ${key}`);
    });
  });

  /* istanbul ignore if */
  if (process.env.SYNCPACK_VERBOSE) {
    groupNames.forEach((key) => {
      groupsByName[key].forEach((group, i) => {
        const size = group.instances.length;
        const ref = `${key}[${group.isDefault ? 'default' : i}]`;
        verbose(`${ref} has ${size} instances`);
        group.instances.forEach(
          ({ name, pathDef, version, packageJsonFile }) => {
            const shortPath = packageJsonFile.shortPath;
            verbose(
              `${ref} ← ${name}@${version} in ${pathDef.path} of ${shortPath}`,
            );
          },
        );
      });
    });
  }

  return groupsByName;
}
