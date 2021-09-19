import { parse } from 'fp-ts/Json';
import * as E from 'fp-ts/lib/Either';
import { pipe } from 'fp-ts/lib/function';
import { readFileSync } from 'fs-extra';
import { SourceWrapper } from '..';
import { getErrorOrElse } from '../try-catch';

export function readJsonSafe(filePath: string): E.Either<Error | SyntaxError, SourceWrapper> {
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
}

function readFileSafe(filePath: string): E.Either<Error, string> {
  return E.tryCatch(
    () => readFileSync(filePath, { encoding: 'utf8' }),
    getErrorOrElse(`Failed to read JSON file at ${filePath}`),
  );
}
