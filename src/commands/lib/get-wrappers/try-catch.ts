/**
 * When using `Either.tryCatch` or `TaskEither.tryCatch`, prefer using the more
 * relevant upstream `Error` when available, otherwise create our own.
 */
export function getErrorOrElse(message: string): (reason: unknown) => Error {
  return function onThrow(err) {
    return err instanceof Error ? err : new Error(message);
  };
}
