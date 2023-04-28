import chalk from 'chalk';
import { ICON } from '../constants';
import type { Context } from '../get-context';

export function writeIfChanged(ctx: Context): Context {
  ctx.packageJsonFiles.forEach((packageJsonFile) => {
    if (packageJsonFile.hasChanged()) {
      packageJsonFile.write();
      console.log(chalk`{green ${ICON.tick}}`, packageJsonFile.shortPath);
    } else {
      console.log(chalk.dim(ICON.skip), chalk.dim(packageJsonFile.shortPath));
    }
  });
  return ctx;
}
