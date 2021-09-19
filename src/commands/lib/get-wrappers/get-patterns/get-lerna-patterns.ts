import { isArrayOfStrings } from 'expect-more';
import * as E from 'fp-ts/lib/Either';
import { flow, pipe } from 'fp-ts/lib/function';
import * as O from 'fp-ts/lib/Option';
import { join } from 'path';
import { MaybePatterns } from '.';
import { CWD } from '../../../../constants';
import { props } from './props';
import { readJsonSafe } from './read-json-safe';

export function getLernaPatterns(): MaybePatterns {
  return pipe(
    readJsonSafe(join(CWD, 'lerna.json')),
    E.map(flow(props('contents.packages'), O.filter(isArrayOfStrings))),
    E.match(
      (): MaybePatterns => O.none,
      (value) => value,
    ),
  );
}
