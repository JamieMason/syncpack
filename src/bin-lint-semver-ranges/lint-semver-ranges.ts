import chalk from 'chalk';
import type { Context } from '../lib/get-context';
import { isValidSemverRange } from '../lib/is-semver';
import { listSemverGroupMismatches } from '../lib/list-semver-group-mismatches';
import { setSemverRange } from '../lib/set-semver-range';

export function lintSemverRanges(ctx: Context): Context {
  /**
   * Reverse the list so the default/ungrouped semver group is rendered first
   * (appears at the top). The actual semver groups which the user configured
   * will then start from index 1.
   */
  ctx.semverGroups.reverse().forEach((semverGroup, i) => {
    if ('range' in semverGroup && isValidSemverRange(semverGroup.range)) {
      const mismatches = listSemverGroupMismatches(semverGroup);

      if (!semverGroup.isDefault && mismatches.length > 0) {
        console.log(chalk`{dim = Semver Group ${i} ${'='.repeat(63)}}`);
      }

      mismatches.forEach(
        ({
          dependencyType,
          dependencyCustomPath,
          name,
          version,
          packageJsonFile,
        }) => {
          const loc =
            dependencyType === 'customDependencies'
              ? `"${dependencyCustomPath}"`
              : dependencyType;
          console.log(
            chalk`{red ✕ ${name}} {red.dim ${version} in ${loc} of ${
              packageJsonFile.contents.name
            } should be ${setSemverRange(semverGroup.range, version)}}`,
          );
        },
      );

      if (mismatches.length > 0) {
        ctx.isInvalid = true;
      }
    }
  });

  return ctx;
}
