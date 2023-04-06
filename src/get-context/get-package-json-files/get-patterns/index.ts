import { pipe } from 'tightrope/fn/pipe';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import type { Result } from 'tightrope/result';
import { Ok } from 'tightrope/result';
import { fromGuard } from 'tightrope/result/from-guard';
import { map } from 'tightrope/result/map';
import { orElse } from 'tightrope/result/or-else';
import { DEFAULT_SOURCES } from '../../../constants';
import type { Disk } from '../../../lib/disk';
import type { Syncpack } from '../../../types';
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
  ): Result<string[]> {
    return pipe(
      getCliPatterns(),
      orElse(getYarnPatterns(disk)),
      orElse(getPnpmPatterns(disk)),
      orElse(getLernaPatterns(disk)),
      map(addRootDir),
      map(limitToPackageJson),
      orElse(() => new Ok(DEFAULT_SOURCES)),
    );

    function getCliPatterns() {
      return fromGuard(
        isArrayOfStrings,
        new Error('No --source options provided'),
        program.source,
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
