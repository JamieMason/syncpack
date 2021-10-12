import type {
  IndexedSemverGroup,
  Instance,
} from '../lib/get-input/get-instances';
import { setSemverRange } from '../lib/set-semver-range';
import { sortByName } from '../lib/sort-by-name';

export function listSemverGroupMismatches(
  semverGroup: IndexedSemverGroup,
): Instance[] {
  return semverGroup.instances.sort(sortByName).filter(({ version }) => {
    return version !== setSemverRange(semverGroup.range, version);
  });
}
