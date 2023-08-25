import { Effect, Option as O, pipe } from 'effect';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { getSource } from '../../config/get-source';
import { DEFAULT_CONFIG } from '../../constants';
import type { Ctx } from '../../get-context';
import type { Io } from '../../io';
import { getLernaPatterns } from './get-lerna-patterns';
import { getPnpmPatterns } from './get-pnpm-patterns';
import { getYarnPatterns } from './get-yarn-patterns';

/**
 * Find every glob pattern which should be used to find package.json files for
 * this monorepo.
 */
export function getPatterns(io: Io, config: Ctx['config']): Effect.Effect<never, never, string[]> {
  return pipe(
    getCliPatterns(),
    Effect.flatMap((opt) => (O.isSome(opt) ? Effect.succeed(opt) : getWorkspacePatterns())),
    Effect.map(O.map(limitToPackageJson)),
    Effect.map(O.getOrElse(() => [...DEFAULT_CONFIG.source])),
  );

  function getCliPatterns(): Effect.Effect<never, never, O.Option<string[]>> {
    return pipe(O.some(getSource(config)), O.filter(isArrayOfStrings), Effect.succeed);
  }

  function getWorkspacePatterns(): Effect.Effect<never, never, O.Option<string[]>> {
    return pipe(
      getYarnPatterns(io),
      Effect.flatMap((opt) => (O.isSome(opt) ? Effect.succeed(opt) : getPnpmPatterns(io))),
      Effect.flatMap((opt) => (O.isSome(opt) ? Effect.succeed(opt) : getLernaPatterns(io))),
      Effect.map(O.map((patterns) => ['package.json', ...patterns])),
    );
  }

  function limitToPackageJson(patterns: string[]): string[] {
    return patterns.map((pattern) =>
      pattern.includes('package.json') ? pattern : `${pattern}/package.json`,
    );
  }
}
