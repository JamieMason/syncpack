import { join } from 'path';
import { get } from 'tightrope/fn/get';
import { pipe } from 'tightrope/fn/pipe';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import type { Result } from 'tightrope/result';
import { andThen } from 'tightrope/result/and-then';
import { filter } from 'tightrope/result/filter';
import { CWD } from '../../constants';
import type { Effects } from '../../lib/effects';
import { readJsonSafe } from './read-json-safe';

export function getLernaPatterns(effects: Effects): () => Result<string[]> {
  return function getLernaPatterns() {
    return pipe(
      join(CWD, 'lerna.json'),
      readJsonSafe(effects),
      andThen((file) => get(file, 'contents', 'packages')),
      filter(isArrayOfStrings, 'no lerna patterns found'),
    );
  };
}
