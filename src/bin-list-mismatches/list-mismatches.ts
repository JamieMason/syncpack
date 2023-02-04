import chalk from 'chalk';
import { relative } from 'path';
import { listVersionGroups } from '../bin-list/list-version-groups';
import { CWD, ICON } from '../constants';
import type { Disk } from '../lib/disk';
import { getExpectedVersion } from '../lib/get-expected-version';
import type { ProgramInput } from '../lib/get-input';
import type { Instance } from '../lib/get-input/get-instances';

export function listMismatches(input: ProgramInput, disk: Disk): void {
  let isInvalid = false;

  /**
   * Reverse the list so the default/ungrouped version group is rendered first
   * (appears at the top). The actual version groups which the user configured
   * will then start from index 1.
   */
  input.instances.versionGroups.reverse().forEach((versionGroup, i) => {
    const groups = listVersionGroups(versionGroup).filter(
      (group) => !group.isIgnored && group.hasMismatches,
    );

    if (groups.length > 0) {
      isInvalid = true;

      if (!versionGroup.isDefault) {
        console.log(chalk`{dim = Version Group ${i} ${'='.repeat(63)}}`);
      }
    }

    groups.forEach(({ instances, isBanned, name }) => {
      let workspaceMatch: Instance | null = null;
      const expected = getExpectedVersion(name, versionGroup, input);

      for (const instance of instances) {
        const isMatch = instance.version === expected;
        const isWorkspace = instance.dependencyType === 'workspace';
        if (isMatch && isWorkspace) {
          workspaceMatch = instance;
        }
      }

      if (isBanned) {
        console.log(
          chalk`{red ${ICON.cross} ${name}} {dim.red is defined in this version group as banned from use}`,
        );
      } else if (workspaceMatch) {
        const shortPath = relative(CWD, workspaceMatch.wrapper.filePath);
        const reason = chalk`{dim : ${expected} is developed in this repo at ${shortPath}}`;
        console.log(chalk`{dim -} ${name}${reason}`);
      } else {
        const reason = chalk`{dim : ${expected} is the highest valid semver version in use}`;
        console.log(chalk`{dim -} ${name}${reason}`);
      }

      instances.forEach(({ dependencyType, version, wrapper }) => {
        const isMatch = version === expected;
        const isLocal = dependencyType === 'workspace';
        const shortPath = relative(CWD, wrapper.filePath);
        const loc = isLocal ? 'version' : dependencyType;
        if (isMatch) {
          console.log(chalk`{green   ${version} in ${loc} of ${shortPath}}`);
        } else {
          console.log(chalk`{red   ${version} in ${loc} of ${shortPath}}`);
        }
      });
    });
  });

  if (isInvalid) {
    disk.process.exit(1);
  }
}
