import type { Context } from './get-context';
import * as log from './log';

export function writeIfChanged(ctx: Context): Context {
  ctx.packageJsonFiles.forEach((packageJsonFile) => {
    if (packageJsonFile.hasChanged()) {
      packageJsonFile.write();
      log.fixed(packageJsonFile.shortPath);
    } else {
      log.skip(packageJsonFile.shortPath);
    }
  });
  return ctx;
}
