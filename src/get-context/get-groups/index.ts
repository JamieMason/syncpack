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

  instances.forEach((instance) => {
    const { name, pkgName } = instance;
    (Object.keys(groupsByName) as (keyof typeof groupsByName)[]).forEach(
      (key) => {
        for (const group of groupsByName[key]) {
          if (group.canAdd(instance)) {
            group.add(instance);
            return;
          }
        }
        throw new Error(`${name} in ${pkgName} did not match any ${key}`);
      },
    );
  });

  return groupsByName;
}
