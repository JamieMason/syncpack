import type { ValidRange } from '../constants';
import { RANGE_LOOSE } from '../constants';
import { isLooseSemver, isSemver, isValidSemverRange } from './is-semver';

export function setSemverRange(
  semverRange: ValidRange,
  version: string,
): string {
  if (!isSemver(version) || !isValidSemverRange(semverRange)) {
    return version;
  }
  const nextVersion = isLooseSemver(version)
    ? version.replace(/\.x/g, '.0')
    : version;
  const from1stNumber = nextVersion.search(/[0-9]/);
  const from1stDot = nextVersion.indexOf('.');
  return semverRange === RANGE_LOOSE
    ? `${nextVersion.slice(from1stNumber, from1stDot)}.x.x`
    : `${semverRange}${nextVersion.slice(from1stNumber)}`;
}
