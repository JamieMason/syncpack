import chalk from 'chalk';
import { version } from 'os';
import { getExpectedVersion } from '../bin-fix-mismatches/get-expected-version';
import { listVersionGroups } from '../bin-list/list-version-groups';
import type { ProgramInput } from '../lib/get-input';

export function listMismatches(input: ProgramInput): void {
  let isInvalid = false;

  /**
   * Reverse the list so the default/ungrouped version group is rendered first
   * (appears at the top). The actual version groups which the user configured
   * will then start from index 1.
   */
  input.instances.versionGroups.reverse().forEach((versionGroup, i) => {
    const isVersionGroup = i > 0;
    const groups = listVersionGroups(versionGroup).filter(
      ({ hasMismatches }) => hasMismatches,
    );

    if (groups.length > 0) {
      isInvalid = true;

      if (isVersionGroup) {
        console.log(chalk`{dim = Version Group ${i} ${'='.repeat(63)}}`);
      }
    }

    groups.forEach(({ instances, name }) => {
      const expectedVersion = getExpectedVersion(name, versionGroup, input);
      console.log(chalk`{dim -} ${name} {green.dim ${expectedVersion}}`);
      instances.forEach(({ dependencyType, version, wrapper }) => {
        console.log(
          chalk`{red   ${version} {dim in ${dependencyType} of ${wrapper.contents.name}}}`,
        );
      });
    });
  });

  if (isInvalid) {
    process.exit(1);
  }
}
