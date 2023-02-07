import { F, O, pipe, R } from '@mobily/ts-belt';
import { join } from 'path';
import { CWD } from '../../../../constants';
import type { Disk } from '../../../disk';
import { BaseError } from '../../../error';
import type { PackageJson } from '../package-json-file';
import { getArrayOfStrings } from './lib/get-array-of-strings';
import { readJsonSafe } from './read-json-safe';

export function getYarnPatterns(
  disk: Disk,
): () => R.Result<string[], BaseError> {
  const getPackages = getArrayOfStrings('workspaces');
  const getPackagesNested = getArrayOfStrings('workspaces.packages');

  return function getYarnPatterns() {
    return pipe(
      join(CWD, 'package.json'),
      readJsonSafe<PackageJson>(disk),
      R.flatMap(({ contents }) =>
        pipe(
          getPackages(contents),
          O.match(F.identity, () => getPackagesNested(contents)),
          O.toResult(new BaseError('no yarn patterns found')),
        ),
      ),
    );
  };
}
