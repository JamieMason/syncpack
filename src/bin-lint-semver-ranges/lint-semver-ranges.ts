import chalk from 'chalk';
import { ICON } from '../constants';
import type { SemverGroup } from '../lib/get-context/get-groups/semver-group';
import type { Instance } from '../lib/get-context/get-package-json-files/package-json-file/instance';
import * as log from '../lib/log';
import { setSemverRange } from '../lib/set-semver-range';
import type { Syncpack } from '../types';

export function lintSemverRanges(ctx: Syncpack.Ctx): Syncpack.Ctx {
  ctx.semverGroups.reverse().forEach((semverGroup, i) => {
    // Nothing to do if there are no mismatches
    if (!semverGroup.hasMismatches()) return;

    // Record that this project has mismatches, so that eg. the CLI can exit
    // with the correct status code.
    ctx.isInvalid = true;

    // Log each group which has mismatches
    semverGroup.getMismatches().forEach(([name, mismatches]) => {
      // Annotate user-defined version groups
      if (!semverGroup.isDefault) log.semverGroupHeader(i);

      // Log the dependency name
      log.invalid(name);

      // Log each of the dependencies mismatches
      mismatches.forEach((instance) => {
        if (!instance.hasRange(semverGroup.range)) {
          logSemverRangeMismatch(instance, semverGroup);
        }
      });
    });
  });

  return ctx;
}

function logSemverRangeMismatch(instance: Instance, semverGroup: SemverGroup) {
  const type = instance.dependencyType;
  const shortPath = instance.packageJsonFile.shortPath;
  const actual = instance.version;
  const expected = setSemverRange(semverGroup.range, actual);
  console.log(
    chalk`  {red ${actual}} ${ICON.rightArrow} {green ${expected}} {dim in ${type} of ${shortPath}}`,
  );
}
