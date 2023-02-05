import chalk from 'chalk';
import { relative } from 'path';
import { CWD, ICON } from '../constants';
import type { Context } from '../lib/get-context';
import type { Instance } from '../lib/get-context/get-groups';
import { getExpectedVersion } from '../lib/get-expected-version';
import { getVersionGroupInstances } from '../lib/get-version-group-instances';

export function listMismatches(ctx: Context): Context {
  /**
   * Reverse the list so the default/ungrouped version group is rendered first
   * (appears at the top). The actual version groups which the user configured
   * will then start from index 1.
   */
  ctx.versionGroups.reverse().forEach((versionGroup, i) => {
    const groups = getVersionGroupInstances(versionGroup);
    const invalidGroups = groups.filter((group) => group.isInvalid);

    if (invalidGroups.length > 0) {
      ctx.isInvalid = true;

      if (!versionGroup.isDefault) {
        console.log(chalk`{dim = Version Group ${i} ${'='.repeat(63)}}`);
      }
    }

    invalidGroups.forEach(({ instances, isBanned, name }) => {
      let workspaceMatch: Instance | null = null;
      const expected = getExpectedVersion(name, versionGroup, ctx);

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
        const shortPath = relative(
          CWD,
          workspaceMatch.packageJsonFile.filePath,
        );
        const reason = chalk`{dim : ${expected} is developed in this repo at ${shortPath}}`;
        console.log(chalk`{dim -} ${name}${reason}`);
      } else {
        const reason = chalk`{dim : ${expected} is the highest valid semver version in use}`;
        console.log(chalk`{dim -} ${name}${reason}`);
      }

      instances.forEach(({ dependencyType, version, packageJsonFile }) => {
        const isMatch = version === expected;
        const isLocal = dependencyType === 'workspace';
        const shortPath = relative(CWD, packageJsonFile.filePath);
        const loc = isLocal ? 'version' : dependencyType;
        if (isMatch) {
          console.log(chalk`{green   ${version} in ${loc} of ${shortPath}}`);
        } else {
          console.log(chalk`{red   ${version} in ${loc} of ${shortPath}}`);
        }
      });
    });
  });

  return ctx;
}
