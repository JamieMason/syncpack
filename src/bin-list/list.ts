import chalk from 'chalk';
import { ICON } from '../constants';
import type { Context } from '../lib/get-context';
import { logVersionGroupHeader } from '../lib/log';

export function list(ctx: Context): Context {
  ctx.versionGroups.reverse().forEach((versionGroup, i) => {
    if (versionGroup.instanceGroups.some((group) => group.isInvalid)) {
      ctx.isInvalid = true;
    }

    if (!versionGroup.isDefault) {
      logVersionGroupHeader(i);
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
