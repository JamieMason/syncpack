import { DependencyType, RANGE_LOOSE } from '../constants';
import { getDependencyTypes } from './lib/get-dependency-types';
import { getDependencies } from './lib/get-installations';
import { getWrappers, SourceWrapper } from './lib/get-wrappers';
import { isLooseSemver, isSemver, isValidSemverRange } from './lib/is-semver';
import { writeIfChanged } from './lib/write-if-changed';

interface Options {
  dev: boolean;
  filter: RegExp;
  indent: string;
  peer: boolean;
  prod: boolean;
  semverRange: string;
  sources: string[];
}

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

export const setSemverRanges = (
  semverRange: string,
  dependencyTypes: DependencyType[],
  filter: RegExp,
  wrapper: SourceWrapper,
): void => {
  const iterator = getDependencies(dependencyTypes, [wrapper]);
  for (const installedPackage of iterator) {
    if (installedPackage.name.search(filter) !== -1) {
      for (const installation of installedPackage.installations) {
        const { name, type, version } = installation;
        const dependencies = installation.source.contents[type];
        if (dependencies) {
          dependencies[name] = setSemverRange(semverRange, version);
        }
      }
    }
  }
};

export const setSemverRangesToDisk = ({
  dev,
  filter,
  indent,
  peer,
  prod,
  semverRange,
  sources: sources,
}: Options): void => {
  const dependencyTypes = getDependencyTypes({ dev, peer, prod });
  getWrappers({ sources }).forEach((wrapper) => {
    writeIfChanged(indent, wrapper, () => {
      setSemverRanges(semverRange, dependencyTypes, filter, wrapper);
    });
  });
};
