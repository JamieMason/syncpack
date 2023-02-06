import chalk from 'chalk';
import { isUndefined } from 'expect-more';
import type { Context } from '../lib/get-context';

export function fixMismatches(ctx: Context): Context {
  /**
   * Reverse the list so the default/ungrouped version group is rendered first
   * (appears at the top). The actual version groups which the user configured
   * will then start from index 1.
   */
  ctx.versionGroups.reverse().forEach((versionGroup, i) => {
    if (!versionGroup.isDefault) {
      console.log(chalk`{dim = Version Group ${i} ${'='.repeat(63)}}`);
    }

    versionGroup.instanceGroups.forEach((instanceGroup) => {
      if (instanceGroup.hasMismatches && !instanceGroup.isIgnored) {
        const nextVersion = instanceGroup.getExpectedVersion();
        instanceGroup.instances.forEach(
          ({
            dependencyType,
            dependencyCustomPath,
            version,
            packageJsonFile,
          }) => {
            const root: any = packageJsonFile.contents;
            if (version !== nextVersion) {
              if (dependencyType === 'pnpmOverrides') {
                root.pnpm.overrides[instanceGroup.name] = nextVersion;
              } else if (dependencyType === 'customDependencies') {
                updateObjectNestedKeyFromPath(
                  root,
                  dependencyCustomPath as string,
                  nextVersion,
                );
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

function updateObjectNestedKeyFromPath(
  obj: any,
  nestedPropertyPath: string,
  value: string | undefined,
) {
  const properties = nestedPropertyPath.split('.');
  const lastKeyIndex = properties.length - 1;
  for (let i = 0; i < lastKeyIndex; ++i) {
    const key = properties[i];
    if (!(key in obj)) obj[key] = {};
    obj = obj[key];
  }
  obj[properties[lastKeyIndex]] = value;
}
