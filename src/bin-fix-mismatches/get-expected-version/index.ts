import type { ProgramInput } from '../../lib/get-input';
import type {
  IndexedBannedVersionGroup,
  IndexedPinnedVersionGroup,
  IndexedVersionGroup,
} from '../../lib/get-input/get-instances';
import { getHighestVersion } from './get-highest-version';
import { getPinnedVersion } from './get-pinned-version';
import { getWorkspaceVersion } from './get-workspace-version';

export function getExpectedVersion(
  name: string,
  versionGroup:
    | Pick<IndexedBannedVersionGroup, 'isBanned' | 'instances'>
    | Pick<IndexedPinnedVersionGroup, 'instances' | 'pinVersion'>
    | Pick<IndexedVersionGroup, 'instances'>,
  input: Pick<ProgramInput, 'workspace' | 'wrappers'>,
): string | undefined {
  if ('isBanned' in versionGroup && versionGroup.isBanned === true) {
    // remove this dependency
    return undefined;
  }
  if ('pinVersion' in versionGroup && versionGroup.pinVersion) {
    return getPinnedVersion(versionGroup);
  }
  if (input.workspace === true) {
    const workspaceVersion = getWorkspaceVersion(name, input.wrappers);
    if (workspaceVersion) return workspaceVersion;
  }
  return getHighestVersion(
    versionGroup.instances
      .filter((instance) => instance.name === name)
      .map(({ version }) => version),
  );
}
