import { isUndefined } from 'expect-more';
import type { Context } from '../lib/get-context';
import { logVersionGroupHeader } from '../lib/log';

export function fixMismatches(ctx: Context): Context {
  /**
   * Reverse the list so the default/ungrouped version group is rendered first
   * (appears at the top). The actual version groups which the user configured
   * will then start from index 1.
   */
  ctx.versionGroups.reverse().forEach((versionGroup, i) => {
    if (!versionGroup.isDefault) {
      logVersionGroupHeader(i);
    }

    versionGroup.instanceGroups.forEach((instanceGroup) => {
      if (
        instanceGroup.hasMismatches &&
        !instanceGroup.versionGroup.isIgnored
      ) {
        const nextVersion = instanceGroup.getExpectedVersion();
        instanceGroup.instances.forEach(
          ({ dependencyType, version, packageJsonFile }) => {
            const root: any = packageJsonFile.contents;
            if (version !== nextVersion) {
              if (dependencyType === 'pnpmOverrides') {
                root.pnpm.overrides[instanceGroup.name] = nextVersion;
              } else {
                root[dependencyType][instanceGroup.name] = nextVersion;
              }
            }
          },
        );
      }
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
