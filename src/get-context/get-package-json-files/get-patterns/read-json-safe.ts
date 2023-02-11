import { flow, pipe, R } from '@mobily/ts-belt';
import type { Disk } from '../../../lib/disk';
import { BaseError } from '../../../lib/error';

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
): (filePath: string) => R.Result<JsonFile<T>, BaseError> {
  return function readJsonSafe(filePath) {
    return pipe(
      readFileSafe(filePath),
      R.flatMap(
        flow(
          parseJsonSafe<T>,
          R.mapError(BaseError.map(`Failed to parse JSON file at ${filePath}`)),
          R.map(({ contents, json }) => ({ contents, filePath, json })),
        ),
      ),
    );
  };

  function readFileSafe(filePath: string): R.Result<string, BaseError> {
    return pipe(
      R.fromExecution(() => disk.readFileSync(filePath)),
      R.mapError(BaseError.map(`Failed to read JSON file at ${filePath}`)),
    );
  }
}

function parseJsonSafe<T>(
  json: string,
): R.Result<{ contents: T; json: string }, BaseError> {
  return pipe(
    R.fromExecution(() => JSON.parse(json)),
    R.mapError(BaseError.map('Failed to parse JSON')),
    R.map((contents) => ({ contents, json })),
  );
}
