import { RANGE_LOOSE, SyncpackConfig } from '../constants';
import { getWrappers, SourceWrapper } from './lib/get-wrappers';
import { getDependencies } from './lib/installations/get-dependencies';
import { isLooseSemver, isSemver, isValidSemverRange } from './lib/is-semver';
import { writeIfChanged } from './lib/write-if-changed';

type Options = Pick<SyncpackConfig, 'dev' | 'filter' | 'indent' | 'peer' | 'prod' | 'semverRange' | 'source'>;

export const setSemverRange = (range: string, version: string): string => {
  if (!isSemver(version) || !isValidSemverRange(range)) {
    return version;
  }
  const nextVersion = isLooseSemver(version) ? version.replace(/\.x/g, '.0') : version;
  const from1stNumber = nextVersion.search(/[0-9]/);
  const from1stDot = nextVersion.indexOf('.');
  return range === RANGE_LOOSE
    ? `${nextVersion.slice(from1stNumber, from1stDot)}.x.x`
    : `${range}${nextVersion.slice(from1stNumber)}`;
};

export const setSemverRanges = (wrapper: SourceWrapper, options: Options): void => {
  const iterator = getDependencies([wrapper], options);
  for (const installedPackage of iterator) {
    if (installedPackage.name.search(new RegExp(options.filter)) !== -1) {
      for (const installation of installedPackage.installations) {
        const { name, type, version } = installation;
        const dependencies = installation.source.contents[type];
        if (dependencies) {
          dependencies[name] = setSemverRange(options.semverRange, version);
        }
      }
    }
  }
};

export const setSemverRangesToDisk = (options: Options): void => {
  const { indent, source } = options;
  getWrappers({ source }).forEach((wrapper) => {
    writeIfChanged(indent, wrapper, () => {
      setSemverRanges(wrapper, options);
    });
  });
};
