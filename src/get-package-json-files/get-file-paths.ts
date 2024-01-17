import { Data, Effect, pipe } from 'effect';
import { uniq } from 'tightrope/array/uniq.js';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array.js';
import type { Ctx } from '../get-context/index.js';
import type { GlobError } from '../io/glob-sync.js';
import { globSync } from '../io/glob-sync.js';
import type { Io } from '../io/index.js';
import { getPatterns } from './get-patterns/index.js';

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
  io: Io,
  config: Ctx['config'],
): Effect.Effect<never, GlobError | NoSourcesFoundError, string[]> {
  return pipe(
    Effect.Do,
    Effect.bind('patterns', () => getPatterns(io, config)),
    Effect.bind('filePaths', ({ patterns }) => globSync(io, patterns)),
    Effect.bind('flatFilePaths', ({ filePaths }) => Effect.sync(() => uniq(filePaths.flat()))),
    Effect.flatMap(({ flatFilePaths, patterns }) =>
      isNonEmptyArray(flatFilePaths)
        ? Effect.succeed(flatFilePaths)
        : Effect.fail(
            new NoSourcesFoundError({
              CWD: io.process.cwd(),
              patterns,
            }),
          ),
    ),
  );
}
