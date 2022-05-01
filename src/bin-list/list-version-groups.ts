import { isNonEmptyString } from 'expect-more';
import type {
  IndexedVersionGroup,
  Instance,
} from '../lib/get-input/get-instances';
import { groupBy } from '../lib/group-by';
import { sortByName } from '../lib/sort-by-name';

export interface ListItem {
  hasMismatches: boolean;
  instances: Instance[];
  isBanned: boolean;
  name: string;
  uniques: string[];
}

export function listVersionGroups(
  versionGroup: IndexedVersionGroup,
): ListItem[] {
  const instances = versionGroup.instances;
  const instancesByName = groupBy<Instance>('name', instances.sort(sortByName));
  return Object.entries(instancesByName).map(([name, instances]) => {
    const pinnedVersion = versionGroup.pinVersion;
    const hasPinnedVersion = isNonEmptyString(pinnedVersion);
    const versions = instances.map(({ version }) => version);
    const uniques = Array.from(new Set(versions));
    const isBanned = versionGroup.isBanned === true;
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
      name,
      uniques,
    };
  });
}
