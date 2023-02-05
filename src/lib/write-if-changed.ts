import type { Context } from './get-context';
import * as log from './log';

export function writeIfChanged(ctx: Context): Context {
  ctx.packageJsonFiles.forEach((packageJsonFile) => {
    if (packageJsonFile.hasChanged()) {
      packageJsonFile.write();
      log.fileChanged(packageJsonFile.filePath);
    } else {
      log.fileUnchanged(packageJsonFile.filePath);
    }
  });
  return ctx;
}
