import * as E from 'fp-ts/lib/Either';
import type { Disk } from '../../../disk';
import { getErrorOrElse } from '../try-catch';

export function readYamlSafe<T = unknown>(
  disk: Disk,
): (filePath: string) => E.Either<Error, T> {
  return function readYamlSafe(filePath) {
    return E.tryCatch(
      () => disk.readYamlFileSync<T>(filePath),
      getErrorOrElse(`Failed to read YAML file at ${filePath}`),
    );
  };
}
