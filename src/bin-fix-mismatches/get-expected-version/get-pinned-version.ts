import type { PinnedVersionGroup } from '../../types/version-group';

export function getPinnedVersion(
  versionGroup: Pick<PinnedVersionGroup, 'pinVersion'>,
): string {
  return versionGroup.pinVersion || '';
}
