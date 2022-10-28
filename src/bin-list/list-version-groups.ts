import { isNonEmptyString } from 'expect-more';
import type {
  AnyIndexedVersionGroup,
  Instance,
} from '../lib/get-input/get-instances';
import { groupBy } from '../lib/group-by';
import { sortByName } from '../lib/sort-by-name';

export interface ListItem {
  hasMismatches: boolean;
  instances: Instance[];
  isBanned: boolean;
  isIgnored: boolean;
  name: string;
  uniques: string[];
}

export function listVersionGroups(
  versionGroup: AnyIndexedVersionGroup,
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
