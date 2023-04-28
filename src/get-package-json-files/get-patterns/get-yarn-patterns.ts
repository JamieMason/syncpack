import { join } from 'path';
import { get } from 'tightrope/fn/get';
import { pipe } from 'tightrope/fn/pipe';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import type { Result } from 'tightrope/result';
import { andThen } from 'tightrope/result/and-then';
import { filter } from 'tightrope/result/filter';
import { orElse } from 'tightrope/result/or-else';
import { CWD } from '../../constants';
import type { Disk } from '../../lib/disk';
import type { PackageJson } from '../package-json-file';
import { readJsonSafe } from './read-json-safe';

export function getYarnPatterns(disk: Disk): () => Result<string[]> {
  return function getYarnPatterns() {
    return pipe(
      join(CWD, 'package.json'),
      readJsonSafe<PackageJson>(disk),
      andThen((file) =>
        pipe(
          get(file, 'contents', 'workspaces', 'packages'),
          orElse(() => get(file, 'contents', 'workspaces')),
        ),
      ),
      filter(isArrayOfStrings, 'no yarn patterns found'),
    );
  };
}
