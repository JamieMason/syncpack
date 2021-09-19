import { isArrayOfStrings } from 'expect-more';
import { flow, pipe } from 'fp-ts/lib/function';
import * as O from 'fp-ts/lib/Option';
import { join } from 'path';
import { ALL_PATTERNS, CWD, SyncpackConfig } from '../../../../constants';
import { tapNone } from '../tap';
import { getLernaPatterns } from './get-lerna-patterns';
import { getPnpmPatterns } from './get-pnpm-patterns';
import { getYarnPatterns } from './get-yarn-patterns';

type Patterns = string[];

export type Options = Pick<SyncpackConfig, 'source'>;
export type MaybePatterns = O.Option<Patterns>;

/**
 * Find every glob pattern which should be used to find package.json files for
 * this monorepo.
 *
 * @returns `['./package.json', './packages/* /package.json']`
 */
export function getPatterns(program: Options): Patterns {
  return pipe(
    O.of(program.source),
    O.filter(isArrayOfStrings),
    tapNone<Patterns>('no --source patterns found'),
    O.fold(
      flow(
        getYarnPatterns,
        tapNone<Patterns>('no yarn workspaces found'),
        O.fold(getPnpmPatterns, O.of),
        tapNone<Patterns>('no pnpm workspaces found'),
        O.fold(getLernaPatterns, O.of),
        tapNone<Patterns>('no lerna packages found'),
        O.map(flow(addRootDir, limitToPackageJson)),
      ),
      O.of,
    ),
    tapNone<Patterns>('no patterns found, using defaults'),
    O.getOrElse(() => ALL_PATTERNS),
  );

  function addRootDir(patterns: Patterns): Patterns {
    return [CWD, ...patterns];
  }

  function limitToPackageJson(patterns: Patterns): Patterns {
    return patterns.map((pattern) => join(pattern, 'package.json'));
  }
}
