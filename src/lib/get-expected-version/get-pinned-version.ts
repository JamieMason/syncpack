import type { Config } from '../get-context/get-config/config';

export function getPinnedVersion(
  versionGroup: Pick<Config.VersionGroup.Pinned, 'pinVersion'>,
): string {
  return versionGroup.pinVersion || '';
}
