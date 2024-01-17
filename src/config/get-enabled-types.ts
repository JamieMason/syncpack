import { Data, Effect, pipe } from 'effect';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings.js';
import { isBoolean } from 'tightrope/guard/is-boolean.js';
import { isEmptyArray } from 'tightrope/guard/is-empty-array.js';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array.js';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string.js';
import { INTERNAL_TYPES } from '../constants.js';
import type { Ctx } from '../get-context/index.js';
import { NameAndVersionPropsStrategy } from '../strategy/name-and-version-props.js';
import { VersionsByNameStrategy } from '../strategy/versions-by-name.js';
import type { InvalidCustomTypeError, Strategy } from './get-custom-types.js';
import { getCustomTypes } from './get-custom-types.js';

export class DeprecatedTypesError extends Data.TaggedClass('DeprecatedTypesError')<{
  readonly types: string[];
}> {}

export class RenamedWorkspaceTypeError extends Data.TaggedClass('RenamedWorkspaceTypeError')<
  Record<string, never>
> {}

export function getEnabledTypes({
  cli,
  rcFile,
}: Ctx['config']): Effect.Effect<
  never,
  DeprecatedTypesError | InvalidCustomTypeError | RenamedWorkspaceTypeError,
  Strategy.Any[]
> {
  return pipe(
    // Look for dependency types defined using the old `{ prod: true }` syntax
    // deprecated in syncpack@9.0.0
    Effect.succeed(
      INTERNAL_TYPES.filter((key) => isBoolean((rcFile as Record<string, boolean>)[key])),
    ),
    // Short-circuit and quit if deprecated config is used
    Effect.flatMap((deprecatedTypeProps) =>
      deprecatedTypeProps.length > 0
        ? Effect.fail(new DeprecatedTypesError({ types: deprecatedTypeProps }))
        : Effect.unit,
    ),
    Effect.flatMap(() =>
      pipe(
        Effect.Do,
        // Get index of every available strategy, keyed by their names as
        // they're referred to in config
        Effect.bind('allStrategiesByName', () =>
          pipe(
            // Get custom types if any are defined, short-circuit and quit if
            // any are invalid
            getCustomTypes({ cli, rcFile }),
            // Combine them with the default/internal dependency types
            Effect.map(
              (customTypes): Record<string, Strategy.Any> =>
                Object.fromEntries([
                  ['dev', new VersionsByNameStrategy('dev', 'devDependencies')],
                  ['local', new NameAndVersionPropsStrategy('local', 'version', 'name')],
                  ['overrides', new VersionsByNameStrategy('overrides', 'overrides')],
                  ['peer', new VersionsByNameStrategy('peer', 'peerDependencies')],
                  ['pnpmOverrides', new VersionsByNameStrategy('pnpmOverrides', 'pnpm.overrides')],
                  ['prod', new VersionsByNameStrategy('prod', 'dependencies')],
                  ['resolutions', new VersionsByNameStrategy('resolutions', 'resolutions')],
                  ...customTypes.map((type) => [type.name, type]),
                ]),
            ),
          ),
        ),
        // The names of every available strategy
        Effect.bind('allStrategyNames', ({ allStrategiesByName }) =>
          Effect.succeed(Object.keys(allStrategiesByName)),
        ),
        // Create groupings to assign each provided dependencyType to
        Effect.bind('strategyNamesByStatus', () =>
          Effect.succeed<Record<'provided' | 'enabled' | 'positive' | 'negative', string[]>>({
            provided: (isNonEmptyString(cli.types)
              ? cli.types.split(',')
              : isArrayOfStrings(rcFile.dependencyTypes)
                ? rcFile.dependencyTypes
                : []
            ).filter(isNonEmptyString),
            enabled: [],
            positive: [],
            negative: [],
          }),
        ),
      ),
    ),
    Effect.tap(({ strategyNamesByStatus }) =>
      Effect.logDebug(
        `dependency types provided by user: ${JSON.stringify(strategyNamesByStatus.provided)}`,
      ),
    ),
    // Determine which dependencyTypes should be enabled based on:
    // * which are defined
    // * which were listed to be enabled
    // * which were listed but !negated
    // * etc.
    Effect.flatMap(({ allStrategiesByName, allStrategyNames, strategyNamesByStatus }) => {
      if (
        isEmptyArray(strategyNamesByStatus.provided) ||
        strategyNamesByStatus.provided.join('') === '**'
      ) {
        return Effect.succeed(allStrategyNames.map(getStrategyByName));
      }

      strategyNamesByStatus.provided.forEach((name) => {
        if (name.startsWith('!')) {
          strategyNamesByStatus.negative.push(name.replace('!', ''));
        } else {
          strategyNamesByStatus.positive.push(name);
        }
      });

      if (isNonEmptyArray(strategyNamesByStatus.negative)) {
        allStrategyNames.forEach((name) => {
          if (!strategyNamesByStatus.negative.includes(name)) {
            strategyNamesByStatus.enabled.push(name);
          }
        });
      }

      if (isNonEmptyArray(strategyNamesByStatus.positive)) {
        strategyNamesByStatus.positive.forEach((name) => {
          if (!strategyNamesByStatus.enabled.includes(name)) {
            strategyNamesByStatus.enabled.push(name);
          }
        });
      }

      if (strategyNamesByStatus.enabled.includes('workspace')) {
        return Effect.fail(new RenamedWorkspaceTypeError({}));
      }

      return Effect.succeed(strategyNamesByStatus.enabled.map(getStrategyByName));

      function getStrategyByName(type: string): Strategy.Any {
        return allStrategiesByName[type] as Strategy.Any;
      }
    }),
    Effect.tap((enabledTypes) =>
      Effect.logDebug(`enabled dependency types determined to be: ${JSON.stringify(enabledTypes)}`),
    ),
  );
}
