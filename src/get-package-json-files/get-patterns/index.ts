import { pipe } from '@effect/data/Function';
import * as O from '@effect/data/Option';
import * as Effect from '@effect/io/Effect';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { getSource } from '../../config/get-source';
import { DEFAULT_SOURCES } from '../../constants';
import type { Env } from '../../env/create-env';
import type { Ctx } from '../../get-context';
import { getLernaPatterns } from './get-lerna-patterns';
import { getPnpmPatterns } from './get-pnpm-patterns';
import { getYarnPatterns } from './get-yarn-patterns';

/**
 * Find every glob pattern which should be used to find package.json files for
 * this monorepo.
 *
 * @returns `['./package.json', './packages/* /package.json']`
 */
export function getPatterns(config: Ctx['config']): Effect.Effect<Env, never, string[]> {
  return pipe(
    getCliPatterns(),
    Effect.flatMap((option) => (O.isSome(option) ? Effect.succeed(option) : getYarnPatterns())),
    Effect.flatMap((option) => (O.isSome(option) ? Effect.succeed(option) : getPnpmPatterns())),
    Effect.flatMap((option) => (O.isSome(option) ? Effect.succeed(option) : getLernaPatterns())),
    Effect.map((option) =>
      pipe(
        option,
        O.map(addRootDir),
        O.map(limitToPackageJson),
        O.getOrElse(() => DEFAULT_SOURCES),
      ),
    ),
  );

  function getCliPatterns() {
    return pipe(O.some(getSource(config)), O.filter(isArrayOfStrings), Effect.succeed);
  }

  function addRootDir(patterns: string[]): string[] {
    return ['package.json', ...patterns];
  }

  function limitToPackageJson(patterns: string[]): string[] {
    return patterns.map((pattern) =>
      pattern.includes('package.json') ? pattern : `${pattern}/package.json`,
    );
  }
}
