import type { Context } from './get-context';
import * as log from './log';

export function writeIfChanged(ctx: Context): Context {
  ctx.wrappers.forEach((wrapper) => {
    if (wrapper.hasChanged()) {
      wrapper.write();
      log.fileChanged(wrapper.filePath);
    } else {
      log.fileUnchanged(wrapper.filePath);
    }
  });
  return ctx;
}
