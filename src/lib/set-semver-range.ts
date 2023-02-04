import { RANGE } from '../constants';
import type { ValidRange } from './get-context/get-config/config';
import { isLooseSemver, isSemver, isValidSemverRange } from './is-semver';

export function setSemverRange(
  semverRange: ValidRange,
  version: string,
): string {
  if (!isSemver(version) || !isValidSemverRange(semverRange)) return version;
  if (semverRange === '*') return semverRange;
  const nextVersion = isLooseSemver(version)
    ? version.replace(/\.x/g, '.0')
    : version;
  const from1stNumber = nextVersion.search(/[0-9]/);
  const from1stDot = nextVersion.indexOf('.');
  return semverRange === RANGE.LOOSE
    ? `${nextVersion.slice(from1stNumber, from1stDot)}.x.x`
    : `${semverRange}${nextVersion.slice(from1stNumber)}`;
}
