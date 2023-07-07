import * as Data from '@effect/data/Data';
import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import { isObject } from 'tightrope/guard/is-object';
import type { Union } from 'ts-toolbelt';
import type { DeprecatedTypesError } from '../config/get-enabled-types';
import { getEnabledTypes } from '../config/get-enabled-types';
import type { Ctx } from '../get-context';
import type { Instance } from '../get-package-json-files/instance';
import { canAddToGroup } from '../guards/can-add-to-group';
import { sortByName } from '../lib/sort-by-name';
import { BannedVersionGroup } from './banned';
import { FilteredOutVersionGroup } from './filtered-out';
import { IgnoredVersionGroup } from './ignored';
import { PinnedVersionGroup } from './pinned';
import { SameRangeVersionGroup } from './same-range';
import { SnappedToVersionGroup } from './snapped-to';
import { StandardVersionGroup } from './standard';
import type{ VersionGroupConfig } from "../config/types";

export type AnyVersionGroup = Union.Strict<
  | BannedVersionGroup
  | FilteredOutVersionGroup
  | IgnoredVersionGroup
  | PinnedVersionGroup
  | SameRangeVersionGroup
  | SnappedToVersionGroup
  | StandardVersionGroup
>;

export namespace VersionGroupReport {
  export class Banned extends Data.TaggedClass('Banned')<{
    name: string;
    instances: Instance[];
    readonly isValid: false;
  }> {}

  export class FilteredOut extends Data.TaggedClass('FilteredOut')<{
    name: string;
    instances: Instance[];
    readonly isValid: true;
  }> {}

  export class HighestSemverMismatch extends Data.TaggedClass('HighestSemverMismatch')<{
    name: string;
    instances: Instance[];
    readonly isValid: false;
    readonly expectedVersion: string;
  }> {}

  export class Ignored extends Data.TaggedClass('Ignored')<{
    name: string;
    instances: Instance[];
    readonly isValid: true;
  }> {}

  export class LowestSemverMismatch extends Data.TaggedClass('LowestSemverMismatch')<{
    name: string;
    instances: Instance[];
    readonly isValid: false;
    readonly expectedVersion: string;
  }> {}

  export class PinnedMismatch extends Data.TaggedClass('PinnedMismatch')<{
    name: string;
    instances: Instance[];
    readonly isValid: false;
    readonly expectedVersion: string;
  }> {}

  export class SameRangeMismatch extends Data.TaggedClass('SameRangeMismatch')<{
    name: string;
    instances: Instance[];
    readonly isValid: false;
  }> {}

  export class SnappedToMismatch extends Data.TaggedClass('SnappedToMismatch')<{
    name: string;
    instances: Instance[];
    readonly isValid: false;
    readonly expectedVersion: string;
    readonly snapTo: string[];
  }> {}

  export class UnsupportedMismatch extends Data.TaggedClass('UnsupportedMismatch')<{
    name: string;
    instances: Instance[];
    readonly isValid: false;
  }> {}

  export class Valid extends Data.TaggedClass('Valid')<{
    name: string;
    instances: Instance[];
    readonly isValid: true;
  }> {}

  export class WorkspaceMismatch extends Data.TaggedClass('WorkspaceMismatch')<{
    name: string;
    instances: Instance[];
    readonly isValid: false;
    readonly expectedVersion: string;
    readonly workspaceInstance: Instance;
  }> {}

  export type ValidCases = Union.Strict<FilteredOut | Ignored | Valid>;

  export type InvalidCases = Union.Strict<
    | Banned
    | HighestSemverMismatch
    | LowestSemverMismatch
    | PinnedMismatch
    | SameRangeMismatch
    | SnappedToMismatch
    | UnsupportedMismatch
    | WorkspaceMismatch
  >;

  export type FixableCases = Union.Strict<
    | HighestSemverMismatch
    | LowestSemverMismatch
    | PinnedMismatch
    | SnappedToMismatch
    | WorkspaceMismatch
  >;

  export type UnfixableCases = Union.Strict<
    SameRangeMismatch | UnsupportedMismatch | WorkspaceMismatch
  >;

  export type HighLowSemverMismatch =
    | VersionGroupReport.HighestSemverMismatch
    | VersionGroupReport.LowestSemverMismatch;

  export type Any = Union.Strict<ValidCases | InvalidCases>;
}

export class VersionGroupConfigError extends Data.TaggedClass('VersionGroupConfigError')<{
  readonly config: unknown;
  readonly error: string;
}> {}

export function getVersionGroups(
  ctx: Ctx,
): Effect.Effect<never, VersionGroupConfigError | DeprecatedTypesError, AnyVersionGroup[]> {
  return pipe(
    Effect.Do(),
    Effect.bind('enabledTypes', () => getEnabledTypes(ctx.config)),
    Effect.bind('versionGroups', () => createVersionGroups(ctx)),
    Effect.flatMap(({ enabledTypes, versionGroups }) => {
      for (const file of ctx.packageJsonFiles) {
        instances: for (const instance of file.getInstances(enabledTypes)) {
          for (const group of versionGroups) {
            if (canAddToGroup(group, instance)) {
              group.instances.push(instance);
              continue instances;
            }
          }
        }
      }
      return Effect.succeed(
        versionGroups.filter((group) => isNonEmptyArray(group.instances.sort(sortByName))),
      );
    }),
  );
}

function createVersionGroups(
  ctx: Ctx,
): Effect.Effect<never, VersionGroupConfigError, AnyVersionGroup[]> {
  const { rcFile } = ctx.config;
  const versionGroups: Effect.Effect<never, VersionGroupConfigError, AnyVersionGroup>[] = [
    Effect.succeed(new FilteredOutVersionGroup(ctx)),
  ];

  if (isNonEmptyArray(rcFile.versionGroups)) {
    // @ts-expect-error TODO: fix this
    rcFile.versionGroups.forEach((config: VersionGroupConfig.Any) => {
      if (!isObject(config)) {
        return versionGroups.push(
          Effect.fail(
            new VersionGroupConfigError({
              config,
              error: 'config is not an object',
            }),
          ),
        );
      }
      if (!isArrayOfStrings(config.dependencies)) {
        return versionGroups.push(
          Effect.fail(
            new VersionGroupConfigError({
              config,
              error: 'config.dependencies is not an array of strings',
            }),
          ),
        );
      }
      if (!isArrayOfStrings(config.packages)) {
        return versionGroups.push(
          Effect.fail(
            new VersionGroupConfigError({
              config,
              error: 'config.packages is not an array of strings',
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
            new VersionGroupConfigError({
              config,
              error: `it's unclear what kind of version group you want, as it contains both ${mutuallyExclusiveProps.join(
                ' and ',
              )}`,
            }),
          ),
        );
      }

      const { dependencies, packages } = config;
      const label = isNonEmptyString(config.label) ? config.label : '';
      const dependencyTypes = isArrayOfStrings(config.dependencyTypes)
        ? config.dependencyTypes
        : [];

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
            new SameRangeVersionGroup({
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
        packages: ['**'],
        preferVersion: 'highestSemver',
      }),
    ),
  );

  return Effect.all(versionGroups);
}
