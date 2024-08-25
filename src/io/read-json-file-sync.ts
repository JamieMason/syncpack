import { dirname, relative } from 'node:path';
import { Data, Effect, pipe } from 'effect';
import { type ParseError, parse } from 'jsonc-parser';
import type { Io } from './index.js';
import type { ReadFileError } from './read-file-sync.js';
import { readFileSync } from './read-file-sync.js';

export class JsonParseError extends Data.TaggedClass('JsonParseError')<{
  readonly errors: ParseError[];
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
    Effect.bind('contents', ({ json }) => {
      const errors: ParseError[] = [];
      const data = parse(json, errors);
      return errors.length === 0
        ? Effect.succeed(data)
        : Effect.fail(new JsonParseError({ errors, filePath, json }));
    }),
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
