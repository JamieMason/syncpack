import chalk from 'chalk';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array';
import { ICON } from '../constants';
import type { Context } from '../get-context';
import { getSemverGroups } from '../get-semver-groups';
import * as log from '../lib/log';
import { sortByName } from '../lib/sort-by-name';

export function lintSemverRanges(ctx: Context): Context {
  const semverGroups = getSemverGroups(ctx);
  const hasUserGroups = isNonEmptyArray(ctx.config.rcFile.semverGroups);

  semverGroups.forEach((semverGroup, i) => {
    semverGroup
      .inspect()
      .sort(sortByName)
      .forEach((report, ii) => {
        // Allow eg. CLI to exit with the correct status code.
        if (!report.isValid) ctx.isInvalid = true;

        switch (report.status) {
          case 'WORKSPACE_SEMVER_RANGE_MISMATCH':
          case 'SEMVER_RANGE_MISMATCH': {
            // Annotate each group
            if (ii === 0 && hasUserGroups)
              log.semverGroupHeader(semverGroup, i);

            console.log(
              chalk`{red %s} %s {red %s} %s {green %s} {dim in %s of %s}`,
              ICON.cross,
              report.name,
              report.instance.version,
              ICON.rightArrow,
              report.expectedVersion,
              report.instance.strategy.path,
              report.instance.packageJsonFile.shortPath,
            );
            break;
          }
          case 'IGNORED':
          case 'UNSUPPORTED_VERSION':
          case 'VALID': {
            // no action needed
            break;
          }
        }
      });
  });

  return ctx;
}
