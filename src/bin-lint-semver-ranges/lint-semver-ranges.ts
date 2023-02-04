import chalk from 'chalk';
import type { Disk } from '../lib/disk';
import type { ProgramInput } from '../lib/get-input';
import { isValidSemverRange } from '../lib/is-semver';
import { setSemverRange } from '../lib/set-semver-range';
import { listSemverGroupMismatches } from './list-semver-group-mismatches';

export function lintSemverRanges(input: ProgramInput, disk: Disk): void {
  let isInvalid = false;

  /**
   * Reverse the list so the default/ungrouped semver group is rendered first
   * (appears at the top). The actual semver groups which the user configured
   * will then start from index 1.
   */
  input.instances.semverGroups.reverse().forEach((semverGroup, i) => {
    if ('range' in semverGroup && isValidSemverRange(semverGroup.range)) {
      const mismatches = listSemverGroupMismatches(semverGroup);

      if (!semverGroup.isDefault && mismatches.length > 0) {
        console.log(chalk`{dim = Semver Group ${i} ${'='.repeat(63)}}`);
      }

      mismatches.forEach(({ dependencyType, name, version, wrapper }) => {
        console.log(
          chalk`{red ✕ ${name}} {red.dim ${version} in ${dependencyType} of ${
            wrapper.contents.name
          } should be ${setSemverRange(semverGroup.range, version)}}`,
        );
      });

      if (mismatches.length > 0) {
        isInvalid = true;
      }
    }
  });

  if (isInvalid) {
    disk.process.exit(1);
  }
}
