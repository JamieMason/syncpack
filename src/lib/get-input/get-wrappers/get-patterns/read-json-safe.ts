import { parse } from 'fp-ts/Json';
import * as E from 'fp-ts/lib/Either';
import { pipe } from 'fp-ts/lib/function';
import type { SourceWrapper } from '..';
import type { Disk } from '../../../../lib/disk';
import { getErrorOrElse } from '../try-catch';

export function readJsonSafe(
  disk: Disk,
): (filePath: string) => E.Either<Error | SyntaxError, SourceWrapper> {
  return function (filePath) {
    return pipe(
      readFileSafe(filePath),
      E.chain((json) =>
        pipe(
          parse(json),
          E.mapLeft(getErrorOrElse(`Failed to parse JSON file at ${filePath}`)),
          E.map((contents) => ({ contents, filePath, json } as SourceWrapper)),
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
