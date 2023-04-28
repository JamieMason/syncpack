import gt from 'semver/functions/gt';
import lt from 'semver/functions/lt';
import { isSupported } from '../../lib/is-semver';
import { clean } from './clean';
import { getRangeScore } from './get-range-score';

const EQ = 0;
const LT = -1;
const GT = 1;

export function compareSemver(a: string, b: string): -1 | 0 | 1 {
  if (!isSupported(a)) throw new Error(`"${a}" is not supported`);
  if (!isSupported(b)) throw new Error(`"${b}" is not supported`);
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
