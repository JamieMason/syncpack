import chalk from 'chalk';

const isVerbose = process.env.NODE_ENV === 'development';

export const addition = (value: string): void => console.log(chalk.green('+ %s'), value);
export const bug = (value: string, err: Error): void => {
  console.log(
    chalk.red('! %s\n\n! Please raise an issue at %s\n\n%s'),
    value,
    chalk.underline('https://github.com/JamieMason/syncpack/issues'),
    String(err.stack).replace(/^/gm, '    ')
  );
  process.exit(1);
};
export const error = (value: string): void => console.log(chalk.red('! %s'), value);
export const info = (value: string): void => console.log(chalk.blue('i %s'), value);
export const removal = (value: string): void => console.log(chalk.red('- %s'), value);
export const resolve = (value: string): void => console.log(chalk.yellow('✓ %s'), value);
export const verbose = isVerbose
  ? (value: string): void => console.info(chalk.grey('? %s'), value)
  : (): void => undefined;
