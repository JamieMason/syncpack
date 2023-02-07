import { pipe, R } from '@mobily/ts-belt';
import type { Disk } from '../../../disk';
import { BaseError } from '../../../error';

export function readYamlSafe<T = unknown>(
  disk: Disk,
): (filePath: string) => R.Result<T, BaseError> {
  return function readYamlSafe(filePath) {
    return pipe(
      R.fromExecution(() => disk.readYamlFileSync<T>(filePath)),
      R.mapError(BaseError.map(`Failed to read YAML file at ${filePath}`)),
    );
  };
}
