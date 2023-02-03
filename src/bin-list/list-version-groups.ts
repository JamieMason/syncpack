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
  const instances = versionGroup.instances.sort(sortByName);
  const instancesByName = groupByName(instances);
  return Object.entries(instancesByName).map(([name, instances]) => {
    const pinnedVersion = (versionGroup as VersionGroup.Pinned).pinVersion;
    const isBanned = (versionGroup as VersionGroup.Banned).isBanned === true;
    const isIgnored = (versionGroup as VersionGroup.Ignored).isIgnored === true;
    const hasPinnedVersion = isNonEmptyString(pinnedVersion);
    const versions = instances.map(({ version }) => version);
    const uniques = Array.from(new Set(versions));
    const [version] = uniques;
    const isUnpinned = hasPinnedVersion && version !== pinnedVersion;
    const hasMismatches = isBanned || isUnpinned || uniques.length > 1;
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

function groupByName(instances: Instance[]) {
  return instances.reduce<Record<string, Instance[]>>((memo, instance) => {
    const name = instance.name;
    memo[name] = (memo[name] || []).concat(instance);
    return memo;
  }, {});
}
