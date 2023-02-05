import { parse } from 'fp-ts/Json';
import * as E from 'fp-ts/lib/Either';
import { pipe } from 'fp-ts/lib/function';
import type { Disk } from '../../../disk';
import { getErrorOrElse } from '../try-catch';

export interface JsonFile<T> {
  /** absolute path on disk to this file */
  readonly filePath: string;
  /** parsed JSON contents of the file */
  contents: T;
  /** raw file contents of the file */
  readonly json: string;
}

export function readJsonSafe<T>(
  disk: Disk,
): (filePath: string) => E.Either<Error | SyntaxError, JsonFile<T>> {
  return function (filePath) {
    return pipe(
      readFileSafe(filePath),
      E.chain((json) =>
        pipe(
          parse(json),
          E.mapLeft(getErrorOrElse(`Failed to parse JSON file at ${filePath}`)),
          E.map((contents) => ({ contents, filePath, json } as JsonFile<T>)),
        ),
      ),
    );
  };

  function readFileSafe(filePath: string): E.Either<Error, string> {
    return E.tryCatch(
      () => disk.readFileSync(filePath),
      getErrorOrElse(`Failed to read JSON file at ${filePath}`),
    );
  }
}
