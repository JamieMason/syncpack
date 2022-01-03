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
    const versions = instances.map(({ version }) => version);
    const uniques = Array.from(new Set(versions));
    const hasMismatches = uniques.length > 1;
    return {
      hasMismatches,
      instances,
      name,
      uniques,
    };
  });
}
