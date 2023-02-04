import { flow } from 'fp-ts/lib/function';
import * as O from 'fp-ts/lib/Option';
import { verbose } from '../../log';

type Fn<T> = (ma: O.Option<T>) => O.Option<T>;

/**
 * Log a message when a pipeline contains `None` and let it continue unchanged.
 */
export function tapNone<T>(message: string): Fn<T> {
  return O.fold<T, O.Option<T>>(function logNoneValue() {
    verbose(message);
    return O.none;
  }, O.of);
}

/**
 * Log a message when a pipeline contains `Some` and let it continue unchanged.
 */
export function tapSome<T>(message: string): Fn<T> {
  return O.map(function logSomeValue(value) {
    verbose(message, value);
    return value;
  });
}

/**
 * Log both possibilities of an `Option` and let it continue unchanged.
 */
export function tapOption<T>(message: string): Fn<T> {
  return flow(tapSome(message), tapNone(`no ${message}`));
}
