import { isArrayOfStrings } from 'expect-more';
import * as E from 'fp-ts/lib/Either';
import { flow, pipe } from 'fp-ts/lib/function';
import * as O from 'fp-ts/lib/Option';
import { join } from 'path';
import type { MaybePatterns } from '.';
import { CWD } from '../../../../constants';
import { props } from './props';
import { readYamlSafe } from './read-yaml-safe';

interface PnpmWorkspace {
  packages?: string[];
}

export function getPnpmPatterns(): MaybePatterns {
  return pipe(
    // packages:
    //   - "packages/**"
    //   - "components/**"
    //   - "!**/test/**"
    readYamlSafe<PnpmWorkspace>(join(CWD, 'pnpm-workspace.yaml')),
    E.map(flow(props('packages'), O.filter(isArrayOfStrings))),
    E.match(
      (): MaybePatterns => O.none,
      (value) => value,
    ),
  );
}
