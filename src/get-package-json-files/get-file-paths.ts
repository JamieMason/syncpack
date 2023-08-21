import * as Data from '@effect/data/Data';
import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import { uniq } from 'tightrope/array/uniq';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array';
import { type Env } from '../env/create-env';
import type { GlobError } from '../env/tags';
import { EnvTag } from '../env/tags';
import type { Ctx } from '../get-context';
import { getPatterns } from './get-patterns';

export class NoSourcesFoundError extends Data.TaggedClass('NoSourcesFoundError')<{
  readonly CWD: string;
  readonly patterns: string[];
}> {}

/**
 * Using --source options and/or config files on disk from
 * npm/pnpm/yarn/lerna, return an array of absolute paths to every package.json
 * file the user is working with.
 *
 * @returns Array of absolute file paths to package.json files
 */
export function getFilePaths(
  config: Ctx['config'],
): Effect.Effect<Env, GlobError | NoSourcesFoundError, string[]> {
  return pipe(
    Effect.Do,
    Effect.bind('env', () => EnvTag),
    Effect.bind('patterns', () => getPatterns(config)),
    Effect.bind('filePaths', ({ env, patterns }) => env.globSync(patterns)),
    Effect.bind('flatFilePaths', ({ filePaths }) => Effect.sync(() => uniq(filePaths.flat()))),
    Effect.flatMap(({ env, flatFilePaths, patterns }) =>
      isNonEmptyArray(flatFilePaths)
        ? Effect.succeed(flatFilePaths)
        : Effect.fail(
            new NoSourcesFoundError({
              CWD: env.CWD,
              patterns,
            }),
          ),
    ),
  );
}
