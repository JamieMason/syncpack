/**
 * Remove unwanted readonly type added by TaskEither.traverseArray
 */
export function removeReadonlyType<T>(value: readonly T[]): T[] {
  return value as T[];
}
