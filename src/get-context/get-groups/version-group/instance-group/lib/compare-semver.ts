import gt from 'semver/functions/gt';
import { isSemver } from '../../../../../lib/is-semver';
import { clean } from './clean';

/** Is this next version to be inspected higher than the current highest? */
export function compareSemver(
  next: string,
  highest: string | undefined,
): '*' | 'invalid' | 'gt' | 'lt' | 'eq' {
  if (next === '*') return '*';
  if (!isSemver(next)) return 'invalid';
  if (!highest || gt(clean(next), highest)) return 'gt';
  if (gt(clean(next), highest)) return 'lt';
  return 'eq';
}
