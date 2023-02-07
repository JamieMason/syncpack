import chalk from 'chalk';
import { isString } from 'expect-more';
import { relative } from 'path';
import { inspect } from 'util';
import { CWD, ICON } from '../constants';

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

export function fileChanged(filePath: string): void {
  console.log(chalk.green(ICON.tick), relative(CWD, filePath));
}

export function fileUnchanged(filePath: string): void {
  console.log(chalk.dim(ICON.skip), chalk.dim(relative(CWD, filePath)));
}

export function logVersionGroupHeader(order: number): void {
  console.log(chalk`{dim = Version Group ${order} ${'='.repeat(63)}}`);
}
