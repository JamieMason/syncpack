import type { TConfig } from '../../../types';
import type { Instance } from '../get-package-json-files/package-json-file/instance';
import { VersionGroup } from './version-group';
import { InstanceGroup } from './version-group/instance-group';

export function getVersionGroups(
  input: TConfig.Private,
  instances: Instance[],
): VersionGroup[] {
  const versionGroups = input.versionGroups.map(
    (versionGroup): VersionGroup => new VersionGroup(input, versionGroup),
  );

  instances.forEach((instance) => {
    const { name, pkgName } = instance;
    for (const versionGroup of versionGroups) {
      if (instance.matchesGroup(versionGroup)) {
        if (!versionGroup.instancesByName[name]) {
          versionGroup.instancesByName[name] = [];
        }
        versionGroup.instancesByName[name]?.push(instance);
        versionGroup.instances.push(instance);
        return;
      }
    }
    throw new Error(`${name} in ${pkgName} did not match any versionGroups`);
  });

  versionGroups.forEach((versionGroup) => {
    versionGroup.instanceGroups = getInstanceGroups(versionGroup);
  });

  return versionGroups;
}

function getInstanceGroups(versionGroup: VersionGroup): InstanceGroup[] {
  return Object.entries(versionGroup.instancesByName).map(
    ([name, instances]): InstanceGroup =>
      new InstanceGroup(versionGroup, name, instances),
  );
}
