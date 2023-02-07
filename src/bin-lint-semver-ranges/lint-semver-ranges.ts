import chalk from 'chalk';
import type { Context } from '../lib/get-context';
import type { ValidRange } from '../lib/get-context/get-config/config';
import type { Instance } from '../lib/get-context/get-package-json-files/package-json-file/instance';
import { logSemverGroupHeader } from '../lib/log';
import { setSemverRange } from '../lib/set-semver-range';

export function lintSemverRanges(ctx: Context): Context {
  ctx.semverGroups.reverse().forEach((semverGroup, i) => {
    const invalidInstances = semverGroup.getInvalidInstances();

    // Nothing to do if there are no mismatches
    if (invalidInstances.length === 0) return;

    // Record that this project has mismatches, so that eg. the CLI can exit
    // with the correct status code.
    ctx.isInvalid = true;

    // Annotate user-defined version groups
    if (!semverGroup.isDefault) logSemverGroupHeader(i);

    // Log the mismatches
    invalidInstances.forEach((instance) => {
      logSemverRangeMismatch(semverGroup.range, instance);
    });
  });

  return ctx;
}

function logSemverRangeMismatch(range: ValidRange, instance: Instance): void {
  const { dependencyType, name, packageJsonFile, version } = instance;
  console.log(
    chalk`{red ✕ ${name}} {red.dim ${version} in ${dependencyType} of ${
      packageJsonFile.contents.name
    } should be ${setSemverRange(range, version)}}`,
  );
}
