import type { Instance, InstanceIndex } from '../lib/get-input/get-instances';
import { setSemverRange } from '../lib/set-semver-range';
import { sortByName } from '../lib/sort-by-name';
import type { SemverGroup } from '../types/semver-group';

export function listSemverGroupMismatches(
  semverGroup: InstanceIndex<SemverGroup.WithRange>,
): Instance[] {
  return semverGroup.instances.sort(sortByName).filter(({ version }) => {
    return version !== setSemverRange(semverGroup.range, version);
  });
}
