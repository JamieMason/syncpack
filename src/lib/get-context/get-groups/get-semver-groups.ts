import type { Config } from '../get-config/config';
import type { InternalConfig } from '../get-config/internal-config';
import type { Instance } from '../get-package-json-files/package-json-file/instance';
import type { Group } from './types';

export namespace SemverGroup {
  export type Any = Group<Config.SemverGroup.Any>;
  export type Ignored = Group<Config.SemverGroup.Ignored>;
  export type WithRange = Group<Config.SemverGroup.WithRange>;
}

export function getSemverGroups(
  input: InternalConfig,
  instances: Instance[],
): SemverGroup.Any[] {
  const semverGroups = input.semverGroups.map(
    (semverGroup): SemverGroup.Any => ({
      ...semverGroup,
      instances: [],
      instancesByName: {},
      isDefault: semverGroup === input.defaultSemverGroup,
    }),
  );

  instances.forEach((instance) => {
    const { name, pkgName } = instance;
    for (const semverGroup of semverGroups) {
      if (instance.matchesGroup(semverGroup)) {
        if (!semverGroup.instancesByName[name]) {
          semverGroup.instancesByName[name] = [];
        }
        semverGroup.instancesByName[name].push(instance);
        semverGroup.instances.push(instance);
        return;
      }
    }
    throw new Error(`${name} in ${pkgName} did not match any semverGroups`);
  });

  return semverGroups;
}
