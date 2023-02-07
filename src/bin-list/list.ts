import chalk from 'chalk';
import { ICON } from '../constants';
import type { Context } from '../lib/get-context';
import type { InstanceGroup } from '../lib/get-context/get-groups/version-group/instance-group';
import * as log from '../lib/log';

export function list(ctx: Context): Context {
  ctx.versionGroups.reverse().forEach((versionGroup, i) => {
    // Annotate user-defined version groups
    if (!versionGroup.isDefault) log.versionGroupHeader(i);

    versionGroup.instanceGroups.forEach((instanceGroup) => {
      const expected = instanceGroup.getExpectedVersion();
      const uniques = instanceGroup.uniques;

      // Record that this project has mismatches, so that eg. the CLI can exit
      // with the correct status code.
      if (instanceGroup.isInvalid) ctx.isInvalid = true;

      instanceGroup.versionGroup.isBanned
        ? logBanned(instanceGroup)
        : instanceGroup.versionGroup.isIgnored
        ? logIgnored(instanceGroup)
        : instanceGroup.hasMismatches
        ? logVersionMismatch(instanceGroup, uniques, expected)
        : logVersionMatch(instanceGroup, uniques);
    });
  });

  return ctx;

  function logVersionMatch(
    instanceGroup: InstanceGroup,
    uniques: string[],
  ): void {
    console.log(chalk`{dim -} {white ${instanceGroup.name}} {dim ${uniques}}`);
  }

  function logVersionMismatch(
    instanceGroup: InstanceGroup,
    uniques: string[],
    expected: string | undefined,
  ): void {
    console.log(
      chalk`{red ${ICON.cross} ${instanceGroup.name}} ${uniques
        .map((version) =>
          version === expected ? chalk.green(version) : chalk.red(version),
        )
        .join(chalk.dim(', '))}`,
    );
  }

  function logIgnored(instanceGroup: InstanceGroup): void {
    console.log(
      chalk`{dim ${ICON.skip} ${instanceGroup.name}} is ignored in this version group`,
    );
  }

  function logBanned(instanceGroup: InstanceGroup): void {
    console.log(
      chalk`{red ${ICON.cross} ${instanceGroup.name}} {dim.red is banned in this version group}`,
    );
  }
}
