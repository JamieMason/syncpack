import { isNonEmptyString } from 'expect-more';
import type {
  IndexedVersionGroup,
  Instance,
} from '../lib/get-input/get-instances';
import { groupBy } from '../lib/group-by';
import { sortByName } from '../lib/sort-by-name';

interface ListItem {
  hasMismatches: boolean;
  instances: Instance[];
  name: string;
  uniques: string[];
}

export function listVersionGroups(
  versionGroup: IndexedVersionGroup,
): ListItem[] {
  const instancesByName = groupBy<Instance>(
    'name',
    versionGroup.instances.sort(sortByName),
  );
  return Object.entries(instancesByName).map(([name, instances]) => {
    const pinnedVersion = versionGroup.pinVersion;
    const hasPinnedVersion = isNonEmptyString(pinnedVersion);
    const versions = instances.map(({ version }) => version);
    const uniques = Array.from(new Set(versions));
    const hasMismatches = versions.some(
      (version, i) =>
        (hasPinnedVersion && version !== pinnedVersion) ||
        (i > 0 && version !== versions[i - 1]),
    );
    return {
      hasMismatches,
      instances,
      name,
      uniques,
    };
  });
}
