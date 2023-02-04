import type { Instance, SemverGroup } from './get-context/get-groups';
import { setSemverRange } from './set-semver-range';
import { sortByName } from './sort-by-name';

export function listSemverGroupMismatches(
  semverGroup: SemverGroup.WithRange,
): Instance[] {
  return semverGroup.instances.sort(sortByName).filter(({ version }) => {
    return version !== setSemverRange(semverGroup.range, version);
  });
}
