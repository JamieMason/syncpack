import { isArrayOfStrings } from 'expect-more';
import * as A from 'fp-ts/lib/Array';
import * as E from 'fp-ts/lib/Either';
import { flow, pipe } from 'fp-ts/lib/function';
import * as O from 'fp-ts/lib/Option';
import * as S from 'fp-ts/lib/string';
import type { SyncpackConfig } from '../../../constants';
import type { Disk } from '../../../lib/disk';
import { getPatterns } from './get-patterns';
import { removeReadonlyType } from './readonly';
import { tapOption } from './tap';
import { getErrorOrElse } from './try-catch';

/**
 * Using --source options and/or config files on disk from npm/pnpm/yarn/lerna,
 * return an array of absolute paths to every package.json file the user is
 * working with.
 *
 * @returns Array of absolute file paths to package.json files
 */
export function getFilePaths(
  disk: Disk,
  program: SyncpackConfig,
): E.Either<Error, O.Option<string[]>> {
  return pipe(
    program,
    getPatterns(disk),
    O.getOrElse<string[]>(() => []),
    E.traverseArray(resolvePattern),
    E.map(removeReadonlyType),
    E.map(flow(A.flatten, A.uniq(S.Eq))),
    E.map(O.fromPredicate(isArrayOfStrings)),
    E.map(tapOption<string[]>('package.json files found')),
  );

  function resolvePattern(pattern: string): E.Either<Error, string[]> {
    return pipe(
      E.tryCatch(
        () => disk.globSync(pattern),
        getErrorOrElse(`npm package "glob" threw on pattern "${pattern}"`),
      ),
      E.map(
        flow(
          O.fromPredicate(isArrayOfStrings),
          tapOption(`files found matching pattern "${pattern}"`),
          O.getOrElse<string[]>(() => []),
        ),
      ),
    );
  }
}
