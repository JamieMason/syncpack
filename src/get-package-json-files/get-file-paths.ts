import { flat } from 'tightrope/array/flat';
import { uniq } from 'tightrope/array/uniq';
import { pipe } from 'tightrope/fn/pipe';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import type { Result } from 'tightrope/result';
import { andThen } from 'tightrope/result/and-then';
import { filter } from 'tightrope/result/filter';
import { fromTry } from 'tightrope/result/from-try';
import { map } from 'tightrope/result/map';
import { mapErr } from 'tightrope/result/map-err';
import type { Context } from '../get-context';
import { $R } from '../lib/$R';
import type { Disk } from '../lib/disk';
import { printStrings } from '../lib/print-strings';
import { getPatterns } from './get-patterns';

type SafeFilePaths = Result<string[]>;

/**
 * Using --source options and/or config files on disk from npm/pnpm/yarn/lerna,
 * return an array of absolute paths to every package.json file the user is
 * working with.
 *
 * @returns Array of absolute file paths to package.json files
 */
export function getFilePaths(
  disk: Disk,
  config: Context['config'],
): SafeFilePaths {
  return pipe(
    config,
    getPatterns(disk),
    andThen(function resolvePatterns(patterns) {
      return pipe(
        patterns,
        $R.onlyOk(function resolvePattern(pattern) {
          return pipe(
            fromTry(() => disk.globSync(pattern)),
            filter(isArrayOfStrings, `"glob" did not match "${pattern}"`),
            map(flat),
            map(uniq),
            $R.tapErrVerbose,
          );
        }),
        map(flat),
        map(uniq),
        mapErr(() => new Error(`No files matched ${printStrings(patterns)}`)),
      );
    }),
  );
}
