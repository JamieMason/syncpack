import { pipe } from 'tightrope/fn/pipe';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import type { Result } from 'tightrope/result';
import { Ok } from 'tightrope/result';
import { fromGuard } from 'tightrope/result/from-guard';
import { map } from 'tightrope/result/map';
import { orElse } from 'tightrope/result/or-else';
import { getSource } from '../../config/get-source';
import { DEFAULT_SOURCES } from '../../constants';
import type { Context } from '../../get-context';
import type { Effects } from '../../lib/effects';
import { getLernaPatterns } from './get-lerna-patterns';
import { getPnpmPatterns } from './get-pnpm-patterns';
import { getYarnPatterns } from './get-yarn-patterns';

/**
 * Find every glob pattern which should be used to find package.json files for
 * this monorepo.
 *
 * @returns `['./package.json', './packages/* /package.json']`
 */
export function getPatterns(effects: Effects) {
  return function getPatterns(config: Context['config']): Result<string[]> {
    return pipe(
      getCliPatterns(),
      orElse(getYarnPatterns(effects)),
      orElse(getPnpmPatterns(effects)),
      orElse(getLernaPatterns(effects)),
      map(addRootDir),
      map(limitToPackageJson),
      orElse(() => new Ok(DEFAULT_SOURCES)),
    );

    function getCliPatterns() {
      return fromGuard(
        isArrayOfStrings,
        new Error('No --source options provided'),
        getSource(config),
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
