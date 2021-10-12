import { isArrayOfStrings } from 'expect-more';
import * as E from 'fp-ts/lib/Either';
import { flow, pipe } from 'fp-ts/lib/function';
import * as O from 'fp-ts/lib/Option';
import type { SyncpackConfig } from '../../../constants';
import type { Disk } from '../../../lib/disk';
import { getPatterns } from './get-patterns';
import { removeReadonlyType } from './readonly';
import { tapNone } from './tap';
import { getErrorOrElse } from './try-catch';

type MaybeFilePaths = O.Option<string[]>;
type EitherMaybeFilePaths = E.Either<Error, MaybeFilePaths>;

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
): EitherMaybeFilePaths {
  return pipe(
    getPatterns(disk, program),
    E.traverseArray(resolvePattern),
    E.map(
      flow(
        removeReadonlyType,
        mergeArrayOfOptionsIntoOne,
        O.filter(isArrayOfStrings),
      ),
    ),
  );

  function resolvePattern(pattern: string): EitherMaybeFilePaths {
    return pipe(
      E.tryCatch(
        () => disk.globSync(pattern),
        getErrorOrElse(`npm package "glob" threw on pattern "${pattern}"`),
      ),
      E.map(
        flow(
          O.of,
          O.filter(isArrayOfStrings),
          tapNone<string[]>(`found 0 files matching pattern "${pattern}"`),
        ),
      ),
    );
  }

  function mergeArrayOfOptionsIntoOne(
    options: MaybeFilePaths[],
  ): MaybeFilePaths {
    const unwrap = O.getOrElse<string[]>(() => []);
    return O.of(
      options.reduce<string[]>(
        (values, option) => values.concat(unwrap(option)),
        [],
      ),
    );
  }
}
