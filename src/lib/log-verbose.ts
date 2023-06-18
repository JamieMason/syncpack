import chalk from 'chalk';
import { isString } from 'tightrope/guard/is-string';
import { inspect } from 'util';
import { ICON } from '../constants';

export function logVerbose(...values: unknown[]): void {
  /* istanbul ignore if */
  if (process.env.SYNCPACK_VERBOSE) {
    console.info(
      chalk.yellow(ICON.debug),
      ...values.map((value) =>
        isString(value)
          ? chalk.yellow(value)
          : inspect(value, {
              colors: true,
              compact: true,
              depth: 20,
              showHidden: false,
            }),
      ),
    );
  }
}
