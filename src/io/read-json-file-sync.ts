import { dirname, relative } from 'node:path';
import { Data, Effect, pipe } from 'effect';
import type { Io } from './index.js';
import type { ReadFileError } from './read-file-sync.js';
import { readFileSync } from './read-file-sync.js';

export class JsonParseError extends Data.TaggedClass('JsonParseError')<{
  readonly error: unknown;
  readonly filePath: string;
  readonly json: string;
}> {}

export class JsonFile<T> extends Data.TaggedClass('JsonFile')<{
  /** absolute path on disk to the directory of this file */
  readonly dirPath: string;
  /** absolute path on disk to this file */
  readonly filePath: string;
  /** relative path on disk to this file */
  readonly shortPath: string;
  /** parsed JSON contents of the file */
  contents: T;
  /** raw file contents of the file */
  readonly json: string;
}> {}

export function readJsonFileSync<T>(
  io: Io,
  filePath: string,
): Effect.Effect<JsonFile<T>, ReadFileError | JsonParseError> {
  return pipe(
    Effect.Do,
    Effect.bind('json', () => readFileSync(io, filePath)),
    Effect.bind('contents', ({ json }) =>
      Effect.try({
        try: () => JSON.parse(json),
        catch: error => new JsonParseError({ error, filePath, json }),
      }),
    ),
    Effect.map(
      ({ contents, json }) =>
        new JsonFile<T>({
          contents,
          dirPath: dirname(filePath),
          filePath,
          json,
          shortPath: relative(io.process.cwd(), filePath),
        }),
    ),
  );
}
