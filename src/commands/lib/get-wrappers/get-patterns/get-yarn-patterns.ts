import { isArrayOfStrings } from 'expect-more';
import * as E from 'fp-ts/lib/Either';
import { pipe } from 'fp-ts/lib/function';
import * as O from 'fp-ts/lib/Option';
import { join } from 'path';
import type { MaybePatterns } from '.';
import { Source } from '..';
import { CWD } from '../../../../constants';
import { props } from './props';
import { readJsonSafe } from './read-json-safe';

export function getYarnPatterns(): MaybePatterns {
  return pipe(
    readJsonSafe(join(CWD, 'package.json')),
    E.map((file) => pipe(findPackages(file.contents))),
    O.fromEither,
    O.flatten,
  );

  function findPackages(yarn: Source): MaybePatterns {
    return pipe(
      getArrayOfStrings('workspaces', yarn),
      O.fold(() => getArrayOfStrings('workspaces.packages', yarn), O.some),
    );
  }

  function getArrayOfStrings(paths: string, yarn: Source): MaybePatterns {
    return pipe(yarn, props(paths), O.filter(isArrayOfStrings));
  }
}
