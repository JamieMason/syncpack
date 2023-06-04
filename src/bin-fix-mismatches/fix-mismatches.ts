import { isObject } from 'tightrope/guard/is-object';
import { isUndefined } from 'tightrope/guard/is-undefined';
import type { Context } from '../get-context';
import { getVersionGroups } from '../get-version-groups';
import { DELETE } from '../get-version-groups/lib/delete';

export function fixMismatches(ctx: Context): Context {
  const versionGroups = getVersionGroups(ctx);
  let shouldPruneEmpty = false;

  versionGroups.forEach((versionGroup) => {
    versionGroup.inspect().forEach((report) => {
      if (!report.isValid) {
        report.instances.forEach((instance) => {
          switch (report.status) {
            case 'HIGHEST_SEMVER_MISMATCH':
            case 'LOWEST_SEMVER_MISMATCH':
            case 'PINNED_MISMATCH':
            case 'SNAPPED_TO_MISMATCH':
            case 'WORKSPACE_MISMATCH': {
              instance.setVersion(report.expectedVersion);
              break;
            }
            case 'BANNED': {
              shouldPruneEmpty = true;
              instance.setVersion(DELETE);
              break;
            }
            case 'SAME_RANGE_MISMATCH':
            case 'UNSUPPORTED_MISMATCH': {
              // @TODO Output something when fix-mismatches faces an unsupported mismatch
              ctx.isInvalid = true;
              break;
            }
          }
        });
      }
    });
  });

  /** Remove empty objects such as `{"dependencies": {}}` left after deleting */
  if (shouldPruneEmpty) {
    ctx.packageJsonFiles.forEach((packageJsonFile) => {
      const contents = packageJsonFile.contents;
      Object.keys(contents).forEach((key) => {
        const value = contents[key];
        if (isObject(value) && Object.values(value).every(isUndefined)) {
          delete contents[key];
        }
      });
    });
  }

  return ctx;
}
