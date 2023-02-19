import { R } from '@mobily/ts-belt';
import { BaseError } from '../../../../lib/error';
import { clean } from './lib/clean';
import { compareSemver } from './lib/compare-semver';
import { getRangeScore } from './lib/get-range-score';

interface HighestVersion {
  withRange: string;
  semver: string;
}

export function getHighestVersion(
  versions: string[],
): R.Result<string, BaseError> {
  let highest: HighestVersion | undefined;

  for (const withRange of versions) {
    switch (compareSemver(withRange, highest?.semver)) {
      // highest possible, quit early
      case '*': {
        return R.Ok(withRange);
      }
      // impossible to know how the user wants to resolve unsupported versions
      case 'invalid': {
        return R.Error(new BaseError(`"${withRange}" is not supported`));
      }
      // we found a new highest version
      case 'gt': {
        highest = newHighestVersion(withRange);
        continue;
      }
      // versions are the same, but one range might be greedier than another
      case 'eq': {
        const score = getRangeScore(withRange);
        const highestScore = getRangeScore(`${highest?.withRange}`);
        if (score > highestScore) highest = newHighestVersion(withRange);
      }
    }
  }

  return highest && highest.withRange
    ? R.Ok(highest.withRange)
    : R.Error(new BaseError(`getHighestVersion(): did not return a version`));
}

function newHighestVersion(withRange: string): HighestVersion {
  return { withRange, semver: clean(withRange) };
}
