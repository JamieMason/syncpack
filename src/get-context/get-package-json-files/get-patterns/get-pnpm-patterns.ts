import { F, O, pipe, R } from '@mobily/ts-belt';
import { join } from 'path';
import { CWD } from '../../../constants';
import type { Disk } from '../../../lib/disk';
import { BaseError } from '../../../lib/error';
import { getArrayOfStrings } from './lib/get-array-of-strings';
import { readYamlSafe } from './read-yaml-safe';

interface PnpmWorkspace {
  packages?: string[];
}

const getPackages = getArrayOfStrings('packages');

export function getPnpmPatterns(
  disk: Disk,
): () => R.Result<string[], BaseError> {
  return function getPnpmPatterns() {
    return pipe(
      // packages:
      //   - "packages/**"
      //   - "components/**"
      //   - "!**/test/**"
      join(CWD, 'pnpm-workspace.yaml'),
      readYamlSafe<PnpmWorkspace>(disk),
      R.flatMap((packageJson) =>
        pipe(
          getPackages(packageJson),
          O.match(F.identity, () => getPackages(packageJson)),
          O.toResult(new BaseError('no pnpm patterns found')),
        ),
      ),
    );
  };
}
