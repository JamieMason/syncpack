import { isEmptyObject, isObject, isUndefined } from 'expect-more';
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

  ctx.packageJsonFiles.forEach((file) => {
    removeEmptyObjects(file.contents);
  });

  return ctx;

  /** Remove eg { "dependencies": {}, "devDependencies": {} }` */
  function removeEmptyObjects(parent: unknown) {
    if (isObject(parent)) {
      Object.entries(parent).forEach(([key, child]) => {
        if (
          isObject(child) &&
          (isEmptyObject(child) || Object.values(child).every(isUndefined))
        ) {
          delete parent[key];
        } else {
          removeEmptyObjects(child);
        }
      });
    }
  }
}
