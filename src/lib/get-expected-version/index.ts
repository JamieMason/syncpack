import type { VersionGroup } from '../get-context/get-groups';
import { getHighestVersion } from './get-highest-version';
import { getPinnedVersion } from './get-pinned-version';
import { getWorkspaceVersion } from './get-workspace-version';

export function getExpectedVersion(
  name: string,
  versionGroup:
    | Pick<VersionGroup.Banned, 'isBanned' | 'instances'>
    | Pick<VersionGroup.Pinned, 'instances' | 'pinVersion'>
    | Pick<VersionGroup.Standard, 'instances'>,
  ctx: {
    workspace: boolean;
    packageJsonFiles: { contents: { name?: string; version?: string } }[];
  },
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
