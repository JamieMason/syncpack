import { Data, Effect } from 'effect';
import type { Io } from './index.js';

export class GlobError extends Data.TaggedClass('GlobError')<{
  readonly error: string;
}> {}

export function globSync(io: Io, patterns: string[]): Effect.Effect<never, GlobError, string[]> {
  return Effect.try({
    try: () =>
      io.globby.sync(patterns, {
        absolute: true,
        cwd: io.process.cwd(),
        fs: io.fs,
        ignore: ['**/node_modules/**'],
      }),
    catch: (err) => new GlobError({ error: String(err) }),
  });
}
