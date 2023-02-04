import { isNonEmptyString } from 'expect-more';
import type { Instance, VersionGroup } from './get-context/get-groups';
import { sortByName } from './sort-by-name';

interface VersionGroupInstance {
  hasMismatches: boolean;
  instances: Instance[];
  isBanned: boolean;
  isIgnored: boolean;
  isInvalid: boolean;
  name: string;
  uniques: string[];
}

export function getVersionGroupInstances(
  versionGroup: VersionGroup.Any,
): VersionGroupInstance[] {
  const instances = versionGroup.instances.sort(sortByName);
  const instancesByName = groupByName(instances);
  return Object.entries(instancesByName).map(
    ([name, instances]): VersionGroupInstance => {
      const pinnedVersion = (versionGroup as VersionGroup.Pinned).pinVersion;
      const isBanned = (versionGroup as VersionGroup.Banned).isBanned === true;
      const isIgnored =
        (versionGroup as VersionGroup.Ignored).isIgnored === true;
      const hasPinnedVersion = isNonEmptyString(pinnedVersion);
      const versions = instances.map(({ version }) => version);
      const uniques = Array.from(new Set(versions));
      const [version] = uniques;
      const isUnpinned = hasPinnedVersion && version !== pinnedVersion;
      const hasMismatches = isBanned || isUnpinned || uniques.length > 1;
      const isInvalid = !isIgnored && hasMismatches;
      return {
        hasMismatches,
        instances,
        isBanned,
        isIgnored,
        isInvalid,
        name,
        uniques,
      };
    },
  );
}

function groupByName(instances: Instance[]) {
  return instances.reduce<Record<string, Instance[]>>((memo, instance) => {
    const name = instance.name;
    memo[name] = (memo[name] || []).concat(instance);
    return memo;
  }, {});
}
