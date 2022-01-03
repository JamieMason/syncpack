import type { IndexedVersionGroup } from '../../lib/get-input/get-instances';

export function getPinnedVersion(
  versionGroup: Pick<IndexedVersionGroup, 'pinVersion'>,
): string {
  return versionGroup.pinVersion || '';
}
