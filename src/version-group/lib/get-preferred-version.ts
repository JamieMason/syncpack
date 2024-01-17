import { Effect, pipe } from 'effect';
import gt from 'semver/functions/gt.js';
import lt from 'semver/functions/lt.js';
import type { VersionGroupConfig } from '../../config/types.js';
import type { Specifier } from '../../specifier/index.js';
import type { NonSemverError } from '../../specifier/lib/non-semver-error.js';
import { clean } from './clean.js';
import { getRangeScore } from './get-range-score.js';

export function getPreferredVersion(
  preferVersion: VersionGroupConfig.Standard['preferVersion'],
  specifiers: Specifier.Any[],
): Effect.Effect<never, NonSemverError, Specifier.Any> {
  return pipe(
    // every instance must have a semver version
    Effect.all(
      specifiers.map((specifier) =>
        pipe(
          specifier.getSemver(),
          Effect.map((semver) => ({ semver, specifier })),
        ),
      ),
    ),
    // comparing semver can error on some loose ranges, all must succeed
    Effect.map((semvers) => semvers.sort((a, b) => compareSemver(a.semver, b.semver))),
    // get the preferred value from the list
    Effect.map((sorted) => sorted[preferVersion === 'lowestSemver' ? 0 : sorted.length - 1]),
    // return just the specifier
    Effect.map((preferred) => preferred?.specifier as Specifier.Any),
  );
}

const EQ = 0;
const LT = -1;
const GT = 1;

function compareSemver(a: string, b: string): -1 | 0 | 1 {
  if (a.startsWith('workspace:')) return LT;
  if (b.startsWith('workspace:')) return GT;
  if (a === b) return EQ;
  if (a === '*') return GT;
  if (b === '*') return LT;
  const cleanA = clean(a);
  const cleanB = clean(b);
  if (gt(cleanA, cleanB)) return GT;
  if (lt(cleanA, cleanB)) return LT;
  const scoreA = getRangeScore(a);
  const scoreB = getRangeScore(b);
  if (scoreA < scoreB) return LT;
  if (scoreA > scoreB) return GT;
  return EQ;
}
