import chalk from 'chalk';
import { isString } from 'expect-more';
import { inspect } from 'util';

export function verbose(...values: any[]): void {
  if (process.env.SYNCPACK_VERBOSE) {
    console.info(
      chalk.yellow('?'),
      ...values.map((value) =>
        isString(value)
          ? chalk.yellow(value)
          : inspect(value, false, null, true),
      ),
    );
  }
}
