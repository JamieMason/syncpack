import type { ProgramInput } from '../../lib/get-input';
import type { IndexedVersionGroup } from '../../lib/get-input/get-instances';
import { getHighestVersion } from './get-highest-version';
import { getPinnedVersion } from './get-pinned-version';
import { getWorkspaceVersion } from './get-workspace-version';

export function getExpectedVersion(
  name: string,
  versionGroup: Pick<IndexedVersionGroup, 'instances' | 'pinVersion'>,
  input: Pick<ProgramInput, 'wrappers'>,
): string {
  return (
    getPinnedVersion(versionGroup) ||
    getWorkspaceVersion(name, input.wrappers) ||
    getHighestVersion(versionGroup.instances.map(({ version }) => version))
  );
}
