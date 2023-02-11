import type { Syncpack } from '../../types';
import type { Instance } from '../get-package-json-files/package-json-file/instance';
import { SemverGroup } from './semver-group';

export function getSemverGroups(
  input: Syncpack.Config.Private,
  instances: Instance[],
): SemverGroup[] {
  const semverGroups = input.semverGroups.map(
    (semverGroup) => new SemverGroup(input, semverGroup),
  );

  instances.forEach((instance) => {
    const { name, pkgName } = instance;
    for (const semverGroup of semverGroups) {
      if (instance.matchesGroup(semverGroup)) {
        if (!semverGroup.instancesByName[name]) {
          semverGroup.instancesByName[name] = [];
        }
        semverGroup.instancesByName[name]?.push(instance);
        semverGroup.instances.push(instance);
        return;
      }
    }
    throw new Error(`${name} in ${pkgName} did not match any semverGroups`);
  });

  return semverGroups;
}
