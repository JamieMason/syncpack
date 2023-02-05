import type { Context } from '../get-context';
import type { VersionGroup } from '../get-context/get-groups';
import { getHighestVersion } from './get-highest-version';
import { getPinnedVersion } from './get-pinned-version';
import { getWorkspaceVersion } from './get-workspace-version';

export function getExpectedVersion(
  name: string,
  versionGroup: VersionGroup.Any,
  ctx: Context,
): string | undefined {
  if ('isBanned' in versionGroup && versionGroup.isBanned === true) {
    // remove this dependency
    return undefined;
  }
  if ('pinVersion' in versionGroup && versionGroup.pinVersion) {
    return getPinnedVersion(versionGroup);
  }
  if (ctx.workspace === true) {
    const workspaceVersion = getWorkspaceVersion(name, ctx.packageJsonFiles);
    if (workspaceVersion) return workspaceVersion;
  }
  return getHighestVersion(
    versionGroup.instances
      .filter((instance) => instance.name === name)
      .map(({ version }) => version),
  );
}
