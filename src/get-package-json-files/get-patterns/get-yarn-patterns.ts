import { join } from 'path';
import { get } from 'tightrope/fn/get';
import { pipe } from 'tightrope/fn/pipe';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import type { Result } from 'tightrope/result';
import { andThen } from 'tightrope/result/and-then';
import { filter } from 'tightrope/result/filter';
import { orElse } from 'tightrope/result/or-else';
import { CWD } from '../../constants';
import type { Effects } from '../../lib/effects';
import type { PackageJson } from '../package-json-file';
import { readJsonSafe } from './read-json-safe';

export function getYarnPatterns(effects: Effects): () => Result<string[]> {
  return function getYarnPatterns() {
    return pipe(
      join(CWD, 'package.json'),
      readJsonSafe<PackageJson>(effects),
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
