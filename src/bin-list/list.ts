import chalk from 'chalk';
import { ICON } from '../constants';
import type { Context } from '../lib/get-context';

export function list(ctx: Context): Context {
  /**
   * Reverse the list so the default/ungrouped version group is rendered first
   * (appears at the top). The actual version groups which the user configured
   * will then start from index 1.
   */
  ctx.versionGroups.reverse().forEach((versionGroup, i) => {
    if (versionGroup.instanceGroups.some((group) => group.isInvalid)) {
      ctx.isInvalid = true;
    }

    if (!versionGroup.isDefault) {
      console.log(chalk`{dim = Version Group ${i} ${'='.repeat(63)}}`);
    }

    versionGroup.instanceGroups.forEach((instanceGroup) => {
      const versionList = instanceGroup.uniques.sort();
      const expected = instanceGroup.getExpectedVersion();
      console.log(
        instanceGroup.versionGroup.isBanned
          ? chalk`{red ${ICON.cross} ${instanceGroup.name}} {dim.red is defined in this version group as banned from use}`
          : instanceGroup.versionGroup.isIgnored
          ? chalk`{dim ${ICON.skip} ${instanceGroup.name}} is ignored in this version group`
          : instanceGroup.hasMismatches
          ? chalk`{red ${ICON.cross} ${instanceGroup.name}} ${versionList
              .map((version) =>
                version === expected
                  ? chalk.green(version)
                  : chalk.red(version),
              )
              .join(chalk.dim(', '))}`
          : chalk`{dim -} {white ${instanceGroup.name}} {dim ${versionList}}`,
      );
    });
  });

  return ctx;
}
