import { pipe } from 'fp-ts/lib/function';
import * as O from 'fp-ts/lib/Option';
import * as C from 'fp-ts/lib/ReadonlyRecord';
import * as S from 'fp-ts/lib/State';

/**
 * Safely read nested properties of any value.
 * @param keys 'child.grandChild.greatGrandChild'
 * @see https://gist.github.com/JamieMason/c0a3b21184cf8c43f76c77878c7c9198
 */
export function props(keys: string) {
  return function getNestedProp(obj: unknown): O.Option<unknown> {
    return pipe(
      keys.split('.'),
      S.traverseArray((key: string) => S.modify(O.chain(C.lookup(key) as never))),
      S.execute(O.fromNullable<unknown>(obj)),
    );
  };
}
