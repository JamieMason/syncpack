import { pipe } from 'tightrope/fn/pipe';
import type { Result } from 'tightrope/result';
import { fromTry } from 'tightrope/result/from-try';
import { mapErr } from 'tightrope/result/map-err';
import type { Effects } from '../../lib/effects';

export function readYamlSafe<T = unknown>(
  effects: Effects,
): (filePath: string) => Result<T> {
  return function readYamlSafe(filePath) {
    return pipe(
      fromTry(() => effects.readYamlFileSync<T>(filePath)),
      mapErr(() => new Error(`Failed to read YAML file at ${filePath}`)),
    );
  };
}
