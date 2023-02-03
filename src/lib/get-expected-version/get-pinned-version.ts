import type { VersionGroup } from '../../types/version-group';

export function getPinnedVersion(
  versionGroup: Pick<VersionGroup.Pinned, 'pinVersion'>,
): string {
  return versionGroup.pinVersion || '';
}
