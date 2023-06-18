import * as Data from '@effect/data/Data';
import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import type { Env } from '../../env/create-env';
import type { ReadFileError } from '../../env/tags';
import { EnvTag } from '../../env/tags';

export interface JsonFile<T> {
  /** absolute path on disk to this file */
  readonly filePath: string;
  /** parsed JSON contents of the file */
  contents: T;
  /** raw file contents of the file */
  readonly json: string;
}

export class JsonParseError extends Data.TaggedClass('JsonParseError')<{
  readonly error: string;
  readonly filePath: string;
  readonly json: string;
}> {}

// @TODO: move to env.readJsonFileSync
export function readJsonSafe<T>(
  filePath: string,
): Effect.Effect<Env, ReadFileError | JsonParseError, JsonFile<T>> {
  return pipe(
    Effect.Do(),
    Effect.bind('env', () => EnvTag),
    Effect.bind('json', ({ env }) => env.readFileSync(filePath)),
    Effect.bind('contents', ({ json }) =>
      // @TODO: move to env.parseJson
      Effect.tryCatch(
        () => JSON.parse(json),
        (err) => new JsonParseError({ error: String(err), filePath, json }),
      ),
    ),
    Effect.map(({ contents, json }) => ({ contents, filePath, json })),
  );
}
