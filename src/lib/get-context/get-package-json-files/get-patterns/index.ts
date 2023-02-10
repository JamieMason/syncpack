import { A, F, pipe, R } from '@mobily/ts-belt';
import { isArrayOfStrings } from 'expect-more';
import { DEFAULT_SOURCES } from '../../../../constants';
import type { Syncpack } from '../../../../types';
import type { Disk } from '../../../disk';
import { BaseError } from '../../../error';
import { getLernaPatterns } from './get-lerna-patterns';
import { getPnpmPatterns } from './get-pnpm-patterns';
import { getYarnPatterns } from './get-yarn-patterns';

/**
 * Find every glob pattern which should be used to find package.json files for
 * this monorepo.
 *
 * @returns `['./package.json', './packages/* /package.json']`
 */
export function getPatterns(disk: Disk) {
  return function getPatterns(
    program: Syncpack.Config.SyncpackRc,
  ): R.Result<string[], BaseError> {
    type PatternResult = R.Result<string[], BaseError>;
    type SafeGetPatterns = () => PatternResult;

    const getters: SafeGetPatterns[] = [
      getCliPatterns,
      getYarnPatterns(disk),
      getPnpmPatterns(disk),
      getLernaPatterns(disk),
    ];

    const initialResult = R.Error(
      new BaseError('getPatterns did not try any sources'),
    ) as PatternResult;

    const res = A.reduce(
      getters,
      initialResult,
      (previousResult, getNextResult) => {
        if (R.isOk(previousResult)) return previousResult;
        return getNextResult();
      },
    );

    return pipe(
      res,
      R.map(addRootDir),
      R.map(limitToPackageJson),
      R.handleError(() => DEFAULT_SOURCES),
      R.mapError(F.identity as () => BaseError),
    );

    function getCliPatterns() {
      return R.fromPredicate(
        program.source,
        isArrayOfStrings,
        new BaseError('No --source options provided'),
      );
    }

    function addRootDir(patterns: string[]): string[] {
      return ['package.json', ...patterns];
    }

    function limitToPackageJson(patterns: string[]): string[] {
      return patterns.map((pattern) =>
        pattern.includes('package.json') ? pattern : `${pattern}/package.json`,
      );
    }
  };
}
