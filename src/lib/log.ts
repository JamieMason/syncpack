import chalk from 'chalk';

export function verbose(message: string): void {
  if (process.env.SYNCPACK_VERBOSE) {
    console.log(chalk.yellow(`? ${message}`));
  }
}
