import { O } from '@mobily/ts-belt';

const isWalkable = (value: unknown): value is Record<string, unknown> =>
  value !== null && typeof value !== 'undefined';

/**
 * Safely read nested properties of any value.
 * @param keys 'child.grandChild.greatGrandChild'
 */
export function props<T>(
  keys: string,
  predicate: (value: unknown) => value is T,
) {
  return function getNestedProp(obj: unknown): O.Option<T> {
    let next = obj;
    for (const key of keys.split('.')) {
      if (isWalkable(next) && key in next) {
        next = next[key];
      } else {
        return O.None;
      }
    }
    return O.fromPredicate(next as any, predicate);
  };
}
