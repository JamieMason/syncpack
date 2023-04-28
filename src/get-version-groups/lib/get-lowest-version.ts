import { pipe } from 'tightrope/fn/pipe';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import type { Result } from 'tightrope/result';
import { filter } from 'tightrope/result/filter';
import { fromTry } from 'tightrope/result/from-try';
import { map } from 'tightrope/result/map';
import { compareSemver } from './sort';

export function getLowestVersion(versions: string[]): Result<string> {
  return pipe(
    fromTry(() => [...versions].sort(compareSemver)),
    map(([lowest]) => lowest),
    filter(isNonEmptyString, 'getLowestVersion(): did not return a version'),
  );
}
