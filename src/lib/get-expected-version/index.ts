import type { VersionGroup } from '../../types/version-group';
import type { ProgramInput } from '../get-input';
import type { InstanceIndex } from '../get-input/get-instances';
import { getHighestVersion } from './get-highest-version';
import { getPinnedVersion } from './get-pinned-version';
import { getWorkspaceVersion } from './get-workspace-version';

export function getExpectedVersion(
  name: string,
  versionGroup:
    | Pick<InstanceIndex<VersionGroup.Banned>, 'isBanned' | 'instances'>
    | Pick<InstanceIndex<VersionGroup.Pinned>, 'instances' | 'pinVersion'>
    | Pick<InstanceIndex<VersionGroup.Default>, 'instances'>,
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
