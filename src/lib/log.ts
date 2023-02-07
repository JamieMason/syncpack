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

export function fixed(message: string): void {
  console.log(chalk`{green ${ICON.tick}}`, message);
}

export function skip(message: string): void {
  console.log(chalk.dim(ICON.skip), chalk.dim(message));
}

export function semverGroupHeader(order: number): void {
  console.log(chalk`{dim = Semver Group ${order} ${'='.repeat(63)}}`);
}

export function versionGroupHeader(order: number): void {
  console.log(chalk`{dim = Version Group ${order} ${'='.repeat(63)}}`);
}

export function valid(message: string, comment?: string): void {
  if (comment) {
    console.log(chalk`{dim ${ICON.skip}} ${message} {dim ${comment}}`);
  } else {
    console.log(chalk`{dim ${ICON.skip}} ${message}`);
  }
}

export function invalid(message: string, comment?: string): void {
  if (comment) {
    console.log(chalk`{red ${ICON.cross}} ${message} {dim ${comment}}`);
  } else {
    console.log(chalk`{red ${ICON.cross}} ${message}`);
  }
}
