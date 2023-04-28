import { pipe } from 'tightrope/fn/pipe';
import type { Result } from 'tightrope/result';
import { andThen } from 'tightrope/result/and-then';
import { fromTry } from 'tightrope/result/from-try';
import { map } from 'tightrope/result/map';
import { mapErr } from 'tightrope/result/map-err';
import type { Disk } from '../../lib/disk';

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
): (filePath: string) => Result<JsonFile<T>> {
  return function readJsonSafe(filePath) {
    return pipe(
      fromTry(() => disk.readFileSync(filePath)),
      andThen((json: string) =>
        pipe(
          fromTry(() => JSON.parse(json)),
          map((contents: T) => ({ contents, json })),
        ),
      ),
      map(({ contents, json }) => ({ contents, filePath, json })),
      mapErr(() => new Error(`Failed to read JSON file at ${filePath}`)),
    );
  };
}
