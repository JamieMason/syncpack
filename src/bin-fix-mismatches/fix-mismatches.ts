import chalk from 'chalk';
import { isUndefined } from 'expect-more';
import type { Context } from '../lib/get-context';
import { getExpectedVersion } from '../lib/get-expected-version';
import { getVersionGroupInstances } from '../lib/get-version-group-instances';

export function fixMismatches(ctx: Context): Context {
  /**
   * Reverse the list so the default/ungrouped version group is rendered first
   * (appears at the top). The actual version groups which the user configured
   * will then start from index 1.
   */
  ctx.versionGroups.reverse().forEach((versionGroup, i) => {
    const groups = getVersionGroupInstances(versionGroup);

    if (!versionGroup.isDefault) {
      console.log(chalk`{dim = Version Group ${i} ${'='.repeat(63)}}`);
    }

    groups.forEach(({ hasMismatches, instances, isIgnored, name }) => {
      if (hasMismatches && !isIgnored) {
        const nextVersion = getExpectedVersion(name, versionGroup, ctx);
        instances.forEach(({ dependencyType, version, wrapper }) => {
          const root: any = wrapper.contents;
          if (version !== nextVersion) {
            if (dependencyType === 'pnpmOverrides') {
              root.pnpm.overrides[name] = nextVersion;
            } else {
              root[dependencyType][name] = nextVersion;
            }
          }
        });
      }
    });
  });

  /** Remove eg `{"dependencies": {}, "devDependencies": {}}` */
  ctx.wrappers.forEach((wrapper) => {
    ctx.dependencyTypes.forEach((dependencyType) => {
      const deps = wrapper.contents[dependencyType];
      if (deps && Object.values(deps).every(isUndefined)) {
        delete wrapper.contents[dependencyType];
      }
    });
  });

  return ctx;
}
