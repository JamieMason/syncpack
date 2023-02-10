import type { Syncpack } from '../types';
import * as log from './log';

export function writeIfChanged(ctx: Syncpack.Ctx): Syncpack.Ctx {
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
