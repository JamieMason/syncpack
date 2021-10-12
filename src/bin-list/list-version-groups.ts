import type { Instance } from '../lib/get-input/get-instances';
import { groupBy } from '../lib/group-by';
import { sortByName } from '../lib/sort-by-name';
import type { ListItem } from './list';

export function listVersionGroups(instances: Instance[]): ListItem[] {
  const instancesByName = groupBy<Instance>('name', instances.sort(sortByName));
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
