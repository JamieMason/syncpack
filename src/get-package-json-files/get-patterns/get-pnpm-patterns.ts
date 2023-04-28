import { join } from 'path';
import { get } from 'tightrope/fn/get';
import { pipe } from 'tightrope/fn/pipe';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import type { Result } from 'tightrope/result';
import { andThen } from 'tightrope/result/and-then';
import { filter } from 'tightrope/result/filter';
import { CWD } from '../../constants';
import type { Disk } from '../../lib/disk';
import { readYamlSafe } from './read-yaml-safe';

interface PnpmWorkspace {
  packages?: string[];
}

export function getPnpmPatterns(disk: Disk): () => Result<string[]> {
  return function getPnpmPatterns() {
    return pipe(
      // packages:
      //   - "packages/**"
      //   - "components/**"
      //   - "!**/test/**"
      join(CWD, 'pnpm-workspace.yaml'),
      readYamlSafe<PnpmWorkspace>(disk),
      andThen((file) => get(file, 'packages')),
      filter(isArrayOfStrings, 'no pnpm patterns found'),
    );
  };
}
