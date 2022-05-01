import chalk from 'chalk';
import type { ProgramInput } from '../lib/get-input';
import { listVersionGroups } from './list-version-groups';

export function list(input: ProgramInput): void {
  let isInvalid = false;

  /**
   * Reverse the list so the default/ungrouped version group is rendered first
   * (appears at the top). The actual version groups which the user configured
   * will then start from index 1.
   */
  input.instances.versionGroups.reverse().forEach((versionGroup, i) => {
    const isVersionGroup = i > 0;
    const groups = listVersionGroups(versionGroup);

    if (isVersionGroup) {
      console.log(chalk`{dim = Version Group ${i} ${'='.repeat(63)}}`);
    }

    groups.forEach(({ hasMismatches, name, uniques }) => {
      const versionList = uniques.sort().join(', ');
      console.log(
        hasMismatches
          ? chalk`{red ✕ ${name}} {dim.red ${versionList}}`
          : chalk`{dim -} {white ${name}} {dim ${versionList}}`,
      );
    });

    if (groups.some(({ hasMismatches }) => hasMismatches)) {
      isInvalid = true;
    }
  });

  if (isInvalid) {
    process.exit(1);
  }
}
