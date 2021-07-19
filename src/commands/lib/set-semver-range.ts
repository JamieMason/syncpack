import { RANGE_LOOSE, SyncpackConfig } from '../../constants';
import { isLooseSemver, isSemver, isValidSemverRange } from './is-semver';

type Options = Pick<SyncpackConfig, 'semverRange'>;

export const setSemverRange = ({ semverRange }: Options) => (version: string): string => {
  if (!isSemver(version) || !isValidSemverRange(semverRange)) {
    return version;
  }
  const nextVersion = isLooseSemver(version) ? version.replace(/\.x/g, '.0') : version;
  const from1stNumber = nextVersion.search(/[0-9]/);
  const from1stDot = nextVersion.indexOf('.');
  return semverRange === RANGE_LOOSE
    ? `${nextVersion.slice(from1stNumber, from1stDot)}.x.x`
    : `${semverRange}${nextVersion.slice(from1stNumber)}`;
};
