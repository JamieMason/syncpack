import type { SemverRange } from '../config/types';
import { RANGE } from '../constants';
import { isLooseSemver } from '../guards/is-loose-semver';
import { isSemver } from '../guards/is-semver';
import { isValidSemverRange } from '../guards/is-valid-semver-range';

export function getSemverRange(version: string): SemverRange {
  if (version === '*') return version;
  const from1stNumber = version.search(/[0-9]/);
  const semverRange = version.slice(0, from1stNumber);
  return isValidSemverRange(semverRange) ? semverRange : '';
}

export function setSemverRange(semverRange: SemverRange, version: string): string {
  if (!isSemver(version) || !isValidSemverRange(semverRange)) return version;
  if (semverRange === '*') return semverRange;
  const nextVersion = isLooseSemver(version) ? version.replace(/\.x/g, '.0') : version;
  const from1stNumber = nextVersion.search(/[0-9]/);
  const from1stDot = nextVersion.indexOf('.');
  return semverRange === RANGE.LOOSE
    ? `${nextVersion.slice(from1stNumber, from1stDot)}.x.x`
    : `${semverRange}${nextVersion.slice(from1stNumber)}`;
}
