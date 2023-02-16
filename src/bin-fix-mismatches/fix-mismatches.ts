import { isObject, isUndefined } from 'expect-more';
import type { Syncpack } from '../types';

export function fixMismatches(ctx: Syncpack.Ctx): Syncpack.Ctx {
  ctx.versionGroups.reverse().forEach((versionGroup) => {
    const invalidGroups = versionGroup.getInvalidInstanceGroups();

    // Nothing to do if there are no mismatches
    if (invalidGroups.length === 0) return;

    // Set the correct version on each instance.
    invalidGroups.forEach((instanceGroup) => {
      if (!instanceGroup.hasUnsupportedVersion()) {
        const nextVersion = instanceGroup.getExpectedVersion();
        instanceGroup.instances.forEach((instance) =>
          instance.setVersion(nextVersion),
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
