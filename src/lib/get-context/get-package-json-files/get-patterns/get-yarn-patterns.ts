import { isArrayOfStrings } from 'expect-more';
import * as E from 'fp-ts/lib/Either';
import { pipe } from 'fp-ts/lib/function';
import * as O from 'fp-ts/lib/Option';
import { join } from 'path';
import { CWD } from '../../../../constants';
import type { Disk } from '../../../disk';
import type { PackageJson } from '../package-json-file';
import { props } from './props';
import { readJsonSafe } from './read-json-safe';

export function getYarnPatterns(disk: Disk): () => O.Option<string[]> {
  return () =>
    pipe(
      readJsonSafe<PackageJson>(disk)(join(CWD, 'package.json')),
      E.map((file) => pipe(findPackages(file.contents))),
      O.fromEither,
      O.flatten,
    );

  function findPackages(yarn: PackageJson): O.Option<string[]> {
    return pipe(
      getArrayOfStrings('workspaces', yarn),
      O.fold(() => getArrayOfStrings('workspaces.packages', yarn), O.some),
    );
  }

  function getArrayOfStrings(
    paths: string,
    yarn: PackageJson,
  ): O.Option<string[]> {
    return pipe(yarn, props(paths), O.filter(isArrayOfStrings));
  }
}
