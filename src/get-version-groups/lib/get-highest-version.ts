import { pipe } from '@effect/data/Function';
import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import type { Result } from 'tightrope/result';
import { filter } from 'tightrope/result/filter';
import { fromTry } from 'tightrope/result/from-try';
import { map } from 'tightrope/result/map';
import { compareSemver } from './compare-semver';

export function getHighestVersion(versions: string[]): Result<string> {
  return pipe(
    fromTry(() => [...versions].sort(compareSemver)),
    map((sorted) => sorted[sorted.length - 1]),
    filter(isNonEmptyString, 'getHighestVersion(): did not return a version'),
  );
}
