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
import { getSemverRange } from '../config/get-semver-range';
import type { Ctx } from '../get-context';
import { canAddToGroup } from '../guards/can-add-to-group';
import { isValidSemverRange } from '../guards/is-valid-semver-range';
import type { Instance } from '../instance';
import { sortByName } from '../lib/sort-by-name';
import { FilteredOutSemverGroup } from './filtered-out';
import { IgnoredSemverGroup } from './ignored';
import { WithRangeSemverGroup } from './with-range';

export type AnySemverGroup = Union.Strict<
  FilteredOutSemverGroup | IgnoredSemverGroup | WithRangeSemverGroup
>;

export namespace SemverGroupReport {
  export class FilteredOut extends Data.TaggedClass('FilteredOut')<{
    name: string;
    instance: Instance.Any;
    readonly isValid: true;
  }> {}

  export class Ignored extends Data.TaggedClass('Ignored')<{
    name: string;
    instance: Instance.Any;
    readonly isValid: true;
  }> {}

  export class Valid extends Data.TaggedClass('Valid')<{
    name: string;
    instance: Instance.Any;
    readonly isValid: true;
  }> {}

  export class LocalPackageSemverRangeMismatch extends Data.TaggedClass(
    'LocalPackageSemverRangeMismatch',
  )<{
    name: string;
    instance: Instance.Any;
    readonly isValid: false;
    readonly expectedVersion: string;
  }> {}

  export class SemverRangeMismatch extends Data.TaggedClass('SemverRangeMismatch')<{
    name: string;
    instance: Instance.Any;
    readonly isValid: false;
    readonly expectedVersion: string;
  }> {}

  export class NonSemverVersion extends Data.TaggedClass('NonSemverVersion')<{
    name: string;
    instance: Instance.Any;
    readonly isValid: false;
  }> {}

  export type ValidCases = Union.Strict<FilteredOut | Ignored | Valid>;

  export type InvalidCases = Union.Strict<
    SemverRangeMismatch | NonSemverVersion | LocalPackageSemverRangeMismatch
  >;

  export type FixableCases = Union.Strict<SemverRangeMismatch | LocalPackageSemverRangeMismatch>;

  export type UnfixableCases = Union.Strict<NonSemverVersion>;

  export type Any = Union.Strict<ValidCases | InvalidCases>;
}

export class SemverGroupConfigError extends Data.TaggedClass('SemverGroupConfigError')<{
  readonly config: unknown;
  readonly error: string;
}> {}

export function getSemverGroups(
  ctx: Ctx,
): Effect.Effect<never, SemverGroupConfigError | DeprecatedTypesError, AnySemverGroup[]> {
  return pipe(
    Effect.Do,
    Effect.bind('enabledTypes', () => getEnabledTypes(ctx.config)),
    Effect.bind('semverGroups', () => createSemverGroups(ctx)),
    Effect.flatMap(({ enabledTypes, semverGroups }) => {
      for (const file of ctx.packageJsonFiles) {
        instances: for (const instance of file.getInstances(enabledTypes)) {
          for (const group of semverGroups) {
            if (canAddToGroup(group, instance)) {
              group.instances.push(instance);
              continue instances;
            }
          }
        }
      }
      return Effect.succeed(
        semverGroups.filter((group) => isNonEmptyArray(group.instances.sort(sortByName))),
      );
    }),
  );
}

function createSemverGroups(
  ctx: Ctx,
): Effect.Effect<never, SemverGroupConfigError, AnySemverGroup[]> {
  const { cli, rcFile } = ctx.config;
  const semverGroups: Effect.Effect<never, SemverGroupConfigError, AnySemverGroup>[] = [
    Effect.succeed(new FilteredOutSemverGroup(ctx)),
  ];

  if (isNonEmptyArray(rcFile.semverGroups)) {
    rcFile.semverGroups.forEach((config) => {
      if (!isObject(config)) {
        return semverGroups.push(
          Effect.fail(
            new SemverGroupConfigError({
              config,
              error: 'config is not an object',
            }),
          ),
        );
      }
      if (!isArrayOfStrings(config.dependencies)) {
        return semverGroups.push(
          Effect.fail(
            new SemverGroupConfigError({
              config,
              error: 'config.dependencies is not an array of strings',
            }),
          ),
        );
      }
      if (!isArrayOfStrings(config.packages)) {
        return semverGroups.push(
          Effect.fail(
            new SemverGroupConfigError({
              config,
              error: 'config.packages is not an array of strings',
            }),
          ),
        );
      }

      const { dependencies, packages } = config;
      const label = isNonEmptyString(config.label) ? config.label : '';
      const dependencyTypes = isArrayOfStrings(config.dependencyTypes)
        ? config.dependencyTypes
        : [];

      if (config.isIgnored === true) {
        semverGroups.push(
          Effect.succeed(
            new IgnoredSemverGroup({
              dependencies,
              dependencyTypes,
              isIgnored: true,
              label,
              packages,
            }),
          ),
        );
      } else if (isValidSemverRange(config.range)) {
        semverGroups.push(
          Effect.succeed(
            new WithRangeSemverGroup(false, {
              dependencies,
              dependencyTypes,
              label,
              packages,
              range: config.range,
            }),
          ),
        );
      }
    });
  }

  semverGroups.push(
    Effect.succeed(
      new WithRangeSemverGroup(true, {
        dependencies: ['**'],
        label: '',
        packages: ['**'],
        range: getSemverRange({ cli, rcFile }),
      }),
    ),
  );

  return Effect.all(semverGroups);
}
