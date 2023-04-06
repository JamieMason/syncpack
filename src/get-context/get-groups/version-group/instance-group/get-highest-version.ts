import type { Result } from 'tightrope/result';
import { Err, Ok } from 'tightrope/result';
import { BaseError } from '../../../../lib/error';
import { clean } from './lib/clean';
import { compareGt } from './lib/compare-semver';
import { getRangeScore } from './lib/get-range-score';

interface HighestVersion {
  withRange: string;
  semver: string;
}

export function getHighestVersion(versions: string[]): Result<string> {
  let highest: HighestVersion | undefined;

  for (const withRange of versions) {
    switch (compareGt(withRange, highest?.semver)) {
      // highest possible, quit early
      case '*': {
        return new Ok(withRange);
      }
      // impossible to know how the user wants to resolve unsupported versions
      case 'invalid': {
        return new Err(new BaseError(`"${withRange}" is not supported`));
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
    ? new Ok(highest.withRange)
    : new Err(new BaseError(`getHighestVersion(): did not return a version`));
}

function newHighestVersion(withRange: string): HighestVersion {
  return { withRange, semver: clean(withRange) };
}
