import gt from 'semver/functions/gt';
import lt from 'semver/functions/lt';
import { isSemver } from '../../../../../lib/is-semver';
import { clean } from './clean';

/** Is this next version to be inspected higher than the current highest? */
export function compareGt(
  next: string,
  highest: string | undefined,
): '*' | 'invalid' | 'gt' | 'lt' | 'eq' {
  if (next === '*') return '*';
  if (!isSemver(next)) return 'invalid';
  if (!highest || gt(clean(next), highest)) return 'gt';
  if (lt(clean(next), highest)) return 'lt';
  return 'eq';
}

/** Is this next version to be inspected lower than the current lowest? */
export function compareLt(
  next: string,
  lowest: string | undefined,
): '*' | 'invalid' | 'gt' | 'lt' | 'eq' {
  if (next === '*') return '*';
  if (!isSemver(next)) return 'invalid';
  if (lowest === '*') return 'lt';
  if (!lowest || lt(clean(next), lowest)) return 'lt';
  if (gt(clean(next), lowest)) return 'gt';
  return 'eq';
}
