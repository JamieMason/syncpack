import * as E from 'fp-ts/lib/Either';
import { sync as readYamlSync } from 'read-yaml-file';
import { getErrorOrElse } from '../try-catch';

export function readYamlSafe<T = unknown>(filePath: string): E.Either<Error, T> {
  return E.tryCatch(() => readYamlSync<T>(filePath), getErrorOrElse(`Failed to read YAML file at ${filePath}`));
}
