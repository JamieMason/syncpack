import * as Data from '@effect/data/Data';
import { isNonEmptyArray } from '@effect/data/ReadonlyArray';
import * as Effect from '@effect/io/Effect';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { isBoolean } from 'tightrope/guard/is-boolean';
import { isEmptyArray } from 'tightrope/guard/is-empty-array';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import { INTERNAL_TYPES } from '../constants';
import type { Ctx } from '../get-context';
import { NameAndVersionPropsStrategy } from '../strategy/name-and-version-props';
import { VersionsByNameStrategy } from '../strategy/versions-by-name';
import type { Strategy } from './get-custom-types';
import { getCustomTypes } from './get-custom-types';

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
  DeprecatedTypesError | RenamedWorkspaceTypeError,
  Strategy.Any[]
> {
  const deprecatedTypeProps = getDeprecatedTypeProps();

  if (deprecatedTypeProps.length > 0) {
    return Effect.fail(new DeprecatedTypesError({ types: deprecatedTypeProps }));
  }

  const allStrategiesByName: Record<string, Strategy.Any> = Object.fromEntries([
    ['dev', new VersionsByNameStrategy('dev', 'devDependencies')],
    ['local', new NameAndVersionPropsStrategy('local', 'version', 'name')],
    ['overrides', new VersionsByNameStrategy('overrides', 'overrides')],
    ['peer', new VersionsByNameStrategy('peer', 'peerDependencies')],
    ['pnpmOverrides', new VersionsByNameStrategy('pnpmOverrides', 'pnpm.overrides')],
    ['prod', new VersionsByNameStrategy('prod', 'dependencies')],
    ['resolutions', new VersionsByNameStrategy('resolutions', 'resolutions')],
    ...getCustomTypes({ cli, rcFile }).map((customType) => [customType.name, customType]),
  ]);
  const allStrategyNames = Object.keys(allStrategiesByName);

  const names: Record<'provided' | 'enabled' | 'positive' | 'negative', string[]> = {
    provided: (isNonEmptyString(cli.types)
      ? cli.types.split(',')
      : isArrayOfStrings(rcFile.dependencyTypes)
      ? rcFile.dependencyTypes
      : []
    ).filter(isNonEmptyString),
    enabled: [],
    positive: [],
    negative: [],
  };

  if (isEmptyArray(names.provided) || names.provided.join('') === '**') {
    return Effect.succeed(allStrategyNames.map(getStrategyByName));
  }

  names.provided.forEach((name) => {
    if (name.startsWith('!')) {
      names.negative.push(name.replace('!', ''));
    } else {
      names.positive.push(name);
    }
  });

  if (isNonEmptyArray(names.negative)) {
    allStrategyNames.forEach((name) => {
      if (!names.negative.includes(name)) {
        names.enabled.push(name);
      }
    });
  }

  if (isNonEmptyArray(names.positive)) {
    names.positive.forEach((name) => {
      if (!names.enabled.includes(name)) {
        names.enabled.push(name);
      }
    });
  }

  if (names.enabled.includes('workspace')) {
    return Effect.fail(new RenamedWorkspaceTypeError({}));
  }

  return Effect.succeed(names.enabled.map(getStrategyByName));

  function getStrategyByName(type: string): Strategy.Any {
    return allStrategiesByName[type] as Strategy.Any;
  }

  /**
   * Look for dependency types defined using the old syntax of `{ prod: true }`
   * which was deprecated in syncpack@9.0.0.
   */
  function getDeprecatedTypeProps() {
    return INTERNAL_TYPES.filter((key) => isBoolean((rcFile as Record<string, boolean>)[key]));
  }
}
