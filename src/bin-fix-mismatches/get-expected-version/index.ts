import type { ProgramInput } from '../../lib/get-input';
import type { Instance } from '../../lib/get-input/get-instances';
import { getHighestVersion } from './get-highest-version';
import { getWorkspaceVersion } from './get-workspace-version';

export function getExpectedVersion(
  input: ProgramInput,
  name: string,
  instances: Instance[],
): string {
  return (
    getWorkspaceVersion(name, input.wrappers) ||
    getHighestVersion(instances.map(({ version }) => version))
  );
}
