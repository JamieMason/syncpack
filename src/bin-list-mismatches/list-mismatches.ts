import chalk from 'chalk';
import { listVersionGroups } from '../bin-list/list-version-groups';
import type { ProgramInput } from '../lib/get-input';

export function listMismatches(input: ProgramInput): void {
  const isInvalid = false;

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

    groups.forEach(({ hasMismatches, instances, name }) => {
      if (hasMismatches) {
        console.log(chalk`{red ✕ ${name}}`);
        instances.forEach(({ dependencyType, version, wrapper }) => {
          console.log(
            chalk`{dim -} ${version} {dim in ${dependencyType} of} ${wrapper.contents.name}`,
          );
        });
      }
    });
  });

  if (isInvalid) {
    process.exit(1);
  }
}
