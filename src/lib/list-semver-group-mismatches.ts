import type { SemverGroup } from './get-context/get-groups/get-semver-groups';
import type { Instance } from './get-context/get-package-json-files/package-json-file/instance';
import { setSemverRange } from './set-semver-range';
import { sortByName } from './sort-by-name';

export function listSemverGroupMismatches(
  semverGroup: SemverGroup.WithRange,
): Instance[] {
  return semverGroup.instances.sort(sortByName).filter(({ version }) => {
    return version !== setSemverRange(semverGroup.range, version);
  });
}
