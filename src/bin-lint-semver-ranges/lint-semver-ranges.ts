import chalk from 'chalk';
import type { ProgramInput } from '../lib/get-input';
import { setSemverRange } from '../lib/set-semver-range';
import { listSemverGroupMismatches } from './list-semver-group-mismatches';

export function lintSemverRanges(input: ProgramInput): void {
  let isInvalid = false;

  /**
   * Reverse the list so the default/ungrouped semver group is rendered first
   * (appears at the top). The actual semver groups which the user configured
   * will then start from index 1.
   */
  input.instances.semverGroups.reverse().forEach((semverGroup, i) => {
    const isSemverGroup = i > 0;
    const mismatches = listSemverGroupMismatches(semverGroup);

    if (isSemverGroup) {
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
  });

  if (isInvalid) {
    process.exit(1);
  }
}
