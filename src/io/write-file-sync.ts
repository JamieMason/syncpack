import { Data, Effect, pipe } from 'effect';
import type { Io } from './index.js';
import { IoTag } from './index.js';

export class WriteFileError extends Data.TaggedClass('WriteFileError')<{
  readonly filePath: string;
  readonly error: string;
}> {}

export function writeFileSync(
  filePath: string,
  contents: string,
): Effect.Effect<Io, WriteFileError, void> {
  return pipe(
    IoTag,
    Effect.flatMap((io) =>
      Effect.try({
        try: () => io.fs.writeFileSync(filePath, contents),
        catch: (err) => new WriteFileError({ filePath, error: String(err) }),
      }),
    ),
  );
}
