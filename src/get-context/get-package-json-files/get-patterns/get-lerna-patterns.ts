import { join } from 'path';
import { get } from 'tightrope/fn/get';
import { pipe } from 'tightrope/fn/pipe';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import type { Result } from 'tightrope/result';
import { andThen } from 'tightrope/result/and-then';
import { filter } from 'tightrope/result/filter';
import { CWD } from '../../../constants';
import type { Disk } from '../../../lib/disk';
import { readJsonSafe } from './read-json-safe';

export function getLernaPatterns(disk: Disk): () => Result<string[]> {
  return function getLernaPatterns() {
    return pipe(
      join(CWD, 'lerna.json'),
      readJsonSafe(disk),
      andThen((file) => get(file, 'contents', 'packages')),
      filter(isArrayOfStrings, 'no lerna patterns found'),
    );
  };
}
