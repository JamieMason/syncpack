import { pipe } from 'tightrope/fn/pipe';
import type { Result } from 'tightrope/result';
import { fromTry } from 'tightrope/result/from-try';
import { mapErr } from 'tightrope/result/map-err';
import type { Disk } from '../../../lib/disk';

export function readYamlSafe<T = unknown>(
  disk: Disk,
): (filePath: string) => Result<T> {
  return function readYamlSafe(filePath) {
    return pipe(
      fromTry(() => disk.readYamlFileSync<T>(filePath)),
      mapErr(() => new Error(`Failed to read YAML file at ${filePath}`)),
    );
  };
}
