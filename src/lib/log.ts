import chalk from 'chalk';
import { isString } from 'expect-more';
import { inspect } from 'util';
import { ICON } from '../constants';

export function verbose(...values: unknown[]): void {
  if (process.env.SYNCPACK_VERBOSE) {
    console.info(
      chalk.yellow(ICON.debug),
      ...values.map((value) =>
        isString(value)
          ? chalk.yellow(value)
          : inspect(value, false, null, true),
      ),
    );
  }
}
