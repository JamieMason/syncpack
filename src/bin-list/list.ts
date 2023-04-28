import chalk from 'chalk';
import { uniq } from 'tightrope/array/uniq';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array';
import { ICON } from '../constants';
import type { Context } from '../get-context';
import type { Instance } from '../get-package-json-files/instance';
import { getVersionGroups } from '../get-version-groups';
import * as log from '../lib/log';
import { sortByName } from '../lib/sort-by-name';

export function list(ctx: Context): Context {
  const versionGroups = getVersionGroups(ctx);
  const hasUserGroups = isNonEmptyArray(ctx.config.rcFile.versionGroups);

  versionGroups.forEach((versionGroup, i) => {
    // Annotate each group
    hasUserGroups && log.versionGroupHeader(versionGroup, i);

    versionGroup
      .inspect()
      .sort(sortByName)
      .forEach((report) => {
        // Allow eg. CLI to exit with the correct status code.
        if (!report.isValid) ctx.isInvalid = true;

        switch (report.status) {
          case 'HIGHEST_SEMVER_MISMATCH':
          case 'LOWEST_SEMVER_MISMATCH':
          case 'PINNED_MISMATCH':
          case 'SNAPPED_TO_MISMATCH':
          case 'WORKSPACE_MISMATCH': {
            console.log(
              chalk`{red %s %s} %s`,
              ICON.cross,
              report.name,
              listColouredVersions(report.expectedVersion, report.instances),
            );
            break;
          }
          case 'BANNED': {
            console.log(
              chalk`{red %s %s} {dim.red is banned in this version group}`,
              ICON.cross,
              report.name,
            );
            break;
          }
          // ignored completely
          case 'FILTERED_OUT':
            break;

          case 'IGNORED': {
            console.log(
              chalk`{dim -} {dim %s} {white is ignored in this version group}`,
              report.name,
            );
            break;
          }
          case 'VALID': {
            console.log(
              chalk`{dim -} {white %s} {dim %s}`,
              report.name,
              report.instances?.[0]?.version,
            );
            break;
          }
          case 'UNSUPPORTED_MISMATCH': {
            console.log(
              chalk`{red %s %s} %s`,
              ICON.cross,
              report.name,
              report.instances
                .map((instance) => chalk.red(instance.version))
                .join(chalk.dim(', ')),
            );
            break;
          }
          // @TODO case 'SEMVER_UNSATISFIED': break;
          // @TODO case 'WORKSPACE_UNSATISFIED': break;
        }

        function listColouredVersions(
          pinVersion: string,
          instances: Instance[],
        ) {
          return getAllVersions(pinVersion, instances)
            .map((version) => withColour(pinVersion, version))
            .join(chalk.dim(', '));
        }

        function withColour(pinVersion: string, version: string) {
          return version === pinVersion
            ? chalk.green(version)
            : chalk.red(version);
        }

        function getAllVersions(pinVersion: string, instances: Instance[]) {
          return uniq([pinVersion].concat(instances.map((i) => i.version)));
        }
      });
  });

  return ctx;
}
