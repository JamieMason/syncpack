import { Effect } from 'effect';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import { isObject } from 'tightrope/guard/is-object';
import { VersionGroup } from '.';
import type { Ctx } from '../get-context';
import { BannedVersionGroup } from './banned';
import { FilteredOutVersionGroup } from './filtered-out';
import { IgnoredVersionGroup } from './ignored';
import { PinnedVersionGroup } from './pinned';
import { SameRangeVersionGroup } from './same-range';
import { SnappedToVersionGroup } from './snapped-to';
import { StandardVersionGroup } from './standard';

export function createVersionGroups(
  ctx: Ctx,
): Effect.Effect<never, VersionGroup.ConfigError, VersionGroup.Any[]> {
  const { rcFile } = ctx.config;
  const versionGroups: Effect.Effect<never, VersionGroup.ConfigError, VersionGroup.Any>[] = [
    Effect.succeed(new FilteredOutVersionGroup(ctx)),
  ];

  if (isNonEmptyArray(rcFile.versionGroups)) {
    rcFile.versionGroups.forEach((config) => {
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

      if (config.isBanned === true) {
        versionGroups.push(
          Effect.succeed(
            new BannedVersionGroup({
              dependencies,
              dependencyTypes,
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
        label: 'Default Version Group',
        packages: ['**'],
        preferVersion: 'highestSemver',
      }),
    ),
  );

  return Effect.all(versionGroups);
}
