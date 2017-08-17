import chalk from 'chalk';

const isVerbose = process.env.NODE_ENV === 'development';

export const addition = value => console.info(chalk.green('+ %s'), value);
export const bug = (value, err) =>
  console.error(
    chalk.red('! %s\n\n! Please raise an issue at %s\n\n%s'),
    value,
    chalk.underline('https://github.com/JamieMason/syncpack/issues'),
    String(err.stack).replace(/^/gm, '    ')
  );
export const error = value => console.error(chalk.red('! %s'), value);
export const info = value => console.info(chalk.blue('i %s'), value);
export const removal = value => console.info(chalk.red('- %s'), value);
export const verbose = value => isVerbose && console.info(chalk.grey('? %s'), value);
