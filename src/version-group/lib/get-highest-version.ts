import type { Specifier } from '../../specifier';
import { getPreferredVersion } from './get-preferred-version';

/**
 * From an array of instances where every instance contains a valid semver
 * version, return the highest version number
 */
export function getHighestVersion(specifiers: Specifier.Any[]) {
  return getPreferredVersion('highestSemver', specifiers);
}
