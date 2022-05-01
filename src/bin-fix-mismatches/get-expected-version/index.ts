import type { ProgramInput } from '../../lib/get-input';
import type { IndexedVersionGroup } from '../../lib/get-input/get-instances';
import { getHighestVersion } from './get-highest-version';
import { getPinnedVersion } from './get-pinned-version';
import { getWorkspaceVersion } from './get-workspace-version';

export function getExpectedVersion(
  name: string,
  versionGroup: Pick<
    IndexedVersionGroup,
    'isBanned' | 'instances' | 'pinVersion'
  >,
  input: Pick<ProgramInput, 'workspace' | 'wrappers'>,
): string | undefined {
  return versionGroup.isBanned === true
    ? // remove this dependency
      undefined
    : getPinnedVersion(versionGroup) ||
        (input.workspace === true &&
          getWorkspaceVersion(name, input.wrappers)) ||
        getHighestVersion(
          versionGroup.instances
            .filter((instance) => instance.name === name)
            .map(({ version }) => version),
        );
}
