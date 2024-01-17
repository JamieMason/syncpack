import { Data, Effect } from 'effect';
import type { Io } from './index.js';

export class ReadFileError extends Data.TaggedClass('ReadFileError')<{
  readonly filePath: string;
  readonly error: string;
}> {}

export function readFileSync(
  io: Io,
  filePath: string,
): Effect.Effect<never, ReadFileError, string> {
  return Effect.try({
    try: () => io.fs.readFileSync(filePath, { encoding: 'utf8' }),
    catch: (err) => new ReadFileError({ filePath, error: String(err) }),
  });
}
