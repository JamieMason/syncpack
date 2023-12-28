import { Effect } from 'effect';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import { isObject } from 'tightrope/guard/is-object';
import { SemverGroup } from '.';
import type { Ctx } from '../get-context';
import { isValidSemverRange } from '../guards/is-valid-semver-range';
import { DisabledSemverGroup } from './disabled';
import { FilteredOutSemverGroup } from './filtered-out';
import { IgnoredSemverGroup } from './ignored';
import { WithRangeSemverGroup } from './with-range';

export function createSemverGroups(
  ctx: Ctx,
): Effect.Effect<never, SemverGroup.ConfigError, SemverGroup.Any[]> {
  const { rcFile } = ctx.config;
  const semverGroups: Effect.Effect<never, SemverGroup.ConfigError, SemverGroup.Any>[] = [
    Effect.succeed(new FilteredOutSemverGroup(ctx)),
    Effect.succeed(
      new WithRangeSemverGroup(false, {
        dependencies: ['**'],
        dependencyTypes: ['local'],
        label: 'the version property of package.json files must always be exact',
        packages: ['**'],
        range: '',
      }),
    ),
  ];

  if (isNonEmptyArray(rcFile.semverGroups)) {
    rcFile.semverGroups.forEach((config: unknown) => {
      if (!isObject(config)) {
        return semverGroups.push(
          Effect.fail(
            new SemverGroup.ConfigError({
              config,
              error: 'config is not an object',
            }),
          ),
        );
      }

      const mutuallyExclusiveProps = (['isIgnored', 'range'] as const).filter((prop) =>
        Boolean(config[prop]),
      );

      if (mutuallyExclusiveProps.length > 1) {
        return semverGroups.push(
          Effect.fail(
            new SemverGroup.ConfigError({
              config,
              error: `it's unclear what kind of semver group you want, as it contains both ${mutuallyExclusiveProps.join(
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
      new DisabledSemverGroup(true, {
        dependencies: ['**'],
        dependencyTypes: ['**'],
        label: 'Default Semver Group',
        packages: ['**'],
        isDisabled: true,
      }),
    ),
  );

  return Effect.all(semverGroups);
}
