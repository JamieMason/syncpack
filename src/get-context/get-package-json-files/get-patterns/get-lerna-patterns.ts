import { O, pipe, R } from '@mobily/ts-belt';
import { join } from 'path';
import { CWD } from '../../../constants';
import type { Disk } from '../../../lib/disk';
import { BaseError } from '../../../lib/error';
import { getArrayOfStrings } from './lib/get-array-of-strings';
import { readJsonSafe } from './read-json-safe';

export function getLernaPatterns(
  disk: Disk,
): () => R.Result<string[], BaseError> {
  const getPackages = getArrayOfStrings('packages');

  return function getLernaPatterns() {
    return pipe(
      join(CWD, 'lerna.json'),
      readJsonSafe(disk),
      R.flatMap(({ contents }) =>
        pipe(
          getPackages(contents),
          O.toResult(new BaseError('no lerna patterns found')),
        ),
      ),
    );
  };
}
