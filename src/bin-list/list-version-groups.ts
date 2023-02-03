import { isNonEmptyString } from 'expect-more';
import type { Instance, InstanceIndex } from '../lib/get-input/get-instances';
import { sortByName } from '../lib/sort-by-name';
import type { VersionGroup } from '../types/version-group';

interface ListItem {
  hasMismatches: boolean;
  instances: Instance[];
  isBanned: boolean;
  isIgnored: boolean;
  name: string;
  uniques: string[];
}

export function listVersionGroups(
  versionGroup: InstanceIndex<VersionGroup.Any>,
): ListItem[] {
  const instances = versionGroup.instances;
  const instancesByName = groupBy<Instance>('name', instances.sort(sortByName));
  return Object.entries(instancesByName).map(([name, instances]) => {
    const pinnedVersion =
      'pinVersion' in versionGroup ? versionGroup.pinVersion : '';
    const hasPinnedVersion = isNonEmptyString(pinnedVersion);
    const versions = instances.map(({ version }) => version);
    const uniques = Array.from(new Set(versions));
    const isBanned =
      'isBanned' in versionGroup && versionGroup.isBanned === true;
    const isIgnored =
      'isIgnored' in versionGroup && versionGroup.isIgnored === true;
    const hasMismatches =
      isBanned ||
      versions.some(
        (version, i) =>
          (hasPinnedVersion && version !== pinnedVersion) ||
          (i > 0 && version !== versions[i - 1]),
      );
    return {
      hasMismatches,
      instances,
      isBanned,
      isIgnored,
      name,
      uniques,
    };
  });
}

function groupBy<T>(key: string, array: T[]): Record<string, T[]> {
  return array.reduce((memo: any, obj: any) => {
    const value = obj[key];
    memo[value] = (memo[value] || []).concat(obj);
    return memo;
  }, {});
}
