import chalk from 'chalk';
import * as O from 'fp-ts/lib/Option';

/**
 * Log a message when a pipeline contains `None` and let it continue unchanged.
 */
export function tapNone<T>(message: string): (ma: O.Option<T>) => O.Option<T> {
  return O.fold<T, O.Option<T>>(function logNoneValue() {
    if (process.env.SYNCPACK_VERBOSE) {
      console.log(chalk.yellow(`? ${message}`));
    }
    return O.none;
  }, O.of);
}
