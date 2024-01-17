import { Effect } from 'effect';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings.js';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array.js';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string.js';
import { isObject } from 'tightrope/guard/is-object.js';
import type { Ctx } from '../get-context/index.js';
import { BannedVersionGroup } from './banned.js';
import { FilteredOutVersionGroup } from './filtered-out.js';
import { IgnoredVersionGroup } from './ignored.js';
import { VersionGroup } from './index.js';
import { PinnedVersionGroup } from './pinned.js';
import { SameRangeVersionGroup } from './same-range.js';
import { SnappedToVersionGroup } from './snapped-to.js';
import { StandardVersionGroup } from './standard.js';

export function createVersionGroups(
  ctx: Ctx,
): Effect.Effect<never, VersionGroup.ConfigError, VersionGroup.Any[]> {
  const { rcFile } = ctx.config;
  const versionGroups: Effect.Effect<never, VersionGroup.ConfigError, VersionGroup.Any>[] = [
    Effect.succeed(new FilteredOutVersionGroup(ctx)),
  ];

  if (isNonEmptyArray(rcFile.versionGroups)) {
    rcFile.versionGroups.forEach((config: unknown) => {
      if (!isObject(config)) {
        return versionGroups.push(
          Effect.fail(
            new VersionGroup.ConfigError({
              config,
              error: 'config is not an object',
            }),
          ),
        );
      }

      const mutuallyExclusiveProps = (
        ['isBanned', 'isIgnored', 'pinVersion', 'snapTo', 'policy'] as const
      ).filter((prop) => Boolean(config[prop]));

      if (mutuallyExclusiveProps.length > 1) {
        return versionGroups.push(
          Effect.fail(
            new VersionGroup.ConfigError({
              config,
              error: `it's unclear what kind of version group you want, as it contains both ${mutuallyExclusiveProps.join(
                ' and ',
              )}`,
            }),
          ),
        );
      }

      const label = isNonEmptyString(config.label) ? config.label : '';
      const dependencyTypes = isArrayOfStrings(config.dependencyTypes)
        ? config.dependencyTypes
        : ['**'];
      const dependencies = isArrayOfStrings(config.dependencies) ? config.dependencies : ['**'];
      const packages = isArrayOfStrings(config.packages) ? config.packages : ['**'];
      const specifierTypes = isArrayOfStrings(config.specifierTypes)
        ? config.specifierTypes
        : ['**'];

      if (config.isBanned === true) {
        versionGroups.push(
          Effect.succeed(
            new BannedVersionGroup({
              dependencies,
              dependencyTypes,
              specifierTypes,
              isBanned: true,
              label,
              packages,
            }),
          ),
        );
      } else if (config.isIgnored === true) {
        versionGroups.push(
          Effect.succeed(
            new IgnoredVersionGroup({
              dependencies,
              dependencyTypes,
              specifierTypes,
              isIgnored: true,
              label,
              packages,
            }),
          ),
        );
      } else if (isNonEmptyString(config.pinVersion)) {
        versionGroups.push(
          Effect.succeed(
            new PinnedVersionGroup({
              dependencies,
              dependencyTypes,
              specifierTypes,
              label,
              packages,
              pinVersion: config.pinVersion,
            }),
          ),
        );
      } else if (isArrayOfStrings(config.snapTo)) {
        versionGroups.push(
          Effect.succeed(
            new SnappedToVersionGroup({
              dependencies,
              dependencyTypes,
              specifierTypes,
              label,
              packages,
              snapTo: config.snapTo,
            }),
          ),
        );
      } else if (config.policy === 'sameRange') {
        versionGroups.push(
          Effect.succeed(
            new SameRangeVersionGroup(ctx, {
              dependencies,
              dependencyTypes,
              specifierTypes,
              label,
              packages,
              policy: config.policy,
            }),
          ),
        );
      } else {
        versionGroups.push(
          Effect.succeed(
            new StandardVersionGroup(false, {
              dependencies,
              dependencyTypes,
              specifierTypes,
              label,
              packages,
              preferVersion:
                config.preferVersion === 'lowestSemver' ? 'lowestSemver' : 'highestSemver',
            }),
          ),
        );
      }
    });
  }

  versionGroups.push(
    Effect.succeed(
      new StandardVersionGroup(true, {
        dependencies: ['**'],
        dependencyTypes: ['**'],
        specifierTypes: ['**'],
        label: 'Default Version Group',
        packages: ['**'],
        preferVersion: 'highestSemver',
      }),
    ),
  );

  return Effect.all(versionGroups);
}
