import { isUndefined } from 'expect-more';
import type { Syncpack } from '../types';

export function fixMismatches(ctx: Syncpack.Ctx): Syncpack.Ctx {
  ctx.versionGroups.reverse().forEach((versionGroup) => {
    const invalidGroups = versionGroup.getInvalidInstanceGroups();

    // Nothing to do if there are no mismatches
    if (invalidGroups.length === 0) return;

    // Set the correct version on each instance.
    invalidGroups.forEach((instanceGroup) => {
      const nextVersion = instanceGroup.getExpectedVersion();
      instanceGroup.instances.forEach((instance) =>
        instance.setVersion(nextVersion),
      );
    });
  });

  /** Remove eg `{"dependencies": {}, "devDependencies": {}}` */
  ctx.packageJsonFiles.forEach((packageJsonFile) => {
    ctx.dependencyTypes.forEach((dependencyType) => {
      const deps = packageJsonFile.contents[dependencyType];
      if (deps && Object.values(deps).every(isUndefined)) {
        delete packageJsonFile.contents[dependencyType];
      }
    });
  });

  return ctx;
}
