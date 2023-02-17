import chalk from 'chalk';
import { ICON } from '../constants';
import type { SemverGroup } from '../get-context/get-groups/semver-group';
import type { Instance } from '../get-context/get-package-json-files/package-json-file/instance';
import * as log from '../lib/log';
import type { Syncpack } from '../types';

export function lintSemverRanges(ctx: Syncpack.Ctx): Syncpack.Ctx {
  const hasUserGroups = ctx.semverGroups.length > 1;

  ctx.semverGroups.forEach((semverGroup, i) => {
    // Nothing to do if there are no mismatches
    if (!semverGroup.hasMismatches()) return;

    // Record that this project has mismatches, so that eg. the CLI can exit
    // with the correct status code.
    ctx.isInvalid = true;

    // Log each group which has mismatches
    semverGroup.getMismatches().forEach(([name, mismatches]) => {
      // Annotate each group
      hasUserGroups && log.semverGroupHeader(semverGroup, i);

      // Log the dependency name
      log.invalid(name);

      // Log each of the dependencies mismatches
      mismatches.forEach((instance) => {
        logSemverRangeMismatch(instance, semverGroup);
      });
    });
  });

  return ctx;
}

function logSemverRangeMismatch(instance: Instance, semverGroup: SemverGroup) {
  const path = instance.pathDef.path;
  const shortPath = instance.packageJsonFile.shortPath;
  const actual = instance.version;
  const expected = semverGroup.getExpectedVersion(instance);
  console.log(
    chalk`  {red ${actual}} ${ICON.rightArrow} {green ${expected}} {dim in ${path} of ${shortPath}}`,
  );
}
