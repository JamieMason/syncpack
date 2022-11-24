import { isArrayOfStrings } from 'expect-more';
import { flow, pipe } from 'fp-ts/lib/function';
import * as O from 'fp-ts/lib/Option';
import type { SyncpackConfig } from '../../../../types';
import { DEFAULT_SOURCES } from '../../../../constants';
import type { Disk } from '../../../../lib/disk';
import { tapNone, tapOption } from '../tap';
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
  return (program: SyncpackConfig): O.Option<string[]> =>
    pipe(
      O.of(program.source),
      O.filter(isArrayOfStrings),
      tapOption('--source patterns found'),
      O.fold(flow(getYarnPatterns(disk), O.map(addRootDir)), O.of),
      tapOption('yarn workspaces found'),
      O.fold(flow(getPnpmPatterns(disk), O.map(addRootDir)), O.of),
      tapOption('pnpm workspaces found'),
      O.fold(flow(getLernaPatterns(disk), O.map(addRootDir)), O.of),
      tapOption('lerna packages found'),
      O.map(limitToPackageJson),
      tapNone('no patterns found, using defaults'),
      O.fold(() => O.some(DEFAULT_SOURCES), O.of),
    );

  function addRootDir(patterns: string[]): string[] {
    return ['.', ...patterns];
  }

  function limitToPackageJson(patterns: string[]): string[] {
    return patterns.map((pattern) =>
      pattern.includes('package.json') ? pattern : `${pattern}/package.json`,
    );
  }
}
