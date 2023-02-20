import { pipe, R } from '@mobily/ts-belt';
import { isObject } from 'expect-more/dist/is-object';
import { isUndefined } from 'expect-more/dist/is-undefined';
import { $R } from '../get-context/$R';
import type { Syncpack } from '../types';

export function fixMismatches(ctx: Syncpack.Ctx): Syncpack.Ctx {
  ctx.versionGroups.forEach((versionGroup) => {
    const invalidGroups = versionGroup.getInvalidInstanceGroups();

    // Nothing to do if there are no mismatches
    if (invalidGroups.length === 0) return;

    // Set the correct version on each instance.
    invalidGroups.forEach((instanceGroup) => {
      if (!instanceGroup.hasUnsupportedVersion()) {
        pipe(
          instanceGroup.getExpectedVersion(),
          R.tap((nextVersion) => {
            instanceGroup.instances.forEach((instance) =>
              instance.setVersion(nextVersion),
            );
          }),
          $R.tapErrVerbose,
        );
      }
    });
  });

  /** Remove eg `{"dependencies": {}, "devDependencies": {}}` */
  ctx.packageJsonFiles.forEach((packageJsonFile) => {
    const contents = packageJsonFile.contents;
    Object.keys(contents).forEach((key) => {
      const value = contents[key];
      if (
        isObject<Record<string, unknown>>(value) &&
        Object.values(value).every(isUndefined)
      ) {
        delete contents[key];
      }
    });
  });

  return ctx;
}
