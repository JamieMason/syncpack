import { verbose } from '../../lib/log';
import type { Syncpack } from '../../types';
import type { Instance } from '../get-package-json-files/package-json-file/instance';
import { SemverGroup } from './semver-group';
import { VersionGroup } from './version-group';

export function getGroups(
  config: Syncpack.Config.Private,
  instances: Instance[],
): {
  semverGroups: SemverGroup[];
  versionGroups: VersionGroup[];
} {
  const groupsByName = {
    semverGroups: config.semverGroups.map(
      (group) => new SemverGroup(config, group),
    ),
    versionGroups: config.versionGroups.map(
      (group) => new VersionGroup(config, group),
    ),
  };
  type Key = keyof typeof groupsByName;
  const groupNames = Object.keys(groupsByName) as Key[];

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
