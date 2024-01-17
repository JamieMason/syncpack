import { Data, Effect } from 'effect';
import type { Io } from './index.js';

class ReadYamlFileError extends Data.TaggedClass('ReadYamlFileError')<{
  readonly filePath: string;
  readonly error: string;
}> {}

export function readYamlFileSync<T = unknown>(
  io: Io,
  filePath: string,
): Effect.Effect<never, ReadYamlFileError, T> {
  return Effect.try({
    try: () => io.readYamlFile.sync(filePath),
    catch: (err) => new ReadYamlFileError({ filePath, error: String(err) }),
  });
}
