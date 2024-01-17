import type { Specifier } from '../../specifier/index.js';
import { getPreferredVersion } from './get-preferred-version.js';

/**
 * From an array of instances where every instance contains a valid semver
 * version, return the highest version number
 */
export function getHighestVersion(specifiers: Specifier.Any[]) {
  return getPreferredVersion('highestSemver', specifiers);
}
