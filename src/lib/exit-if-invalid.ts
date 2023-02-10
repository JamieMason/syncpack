import type { Syncpack } from '../types';

export function exitIfInvalid(ctx: Syncpack.Ctx): Syncpack.Ctx {
  if (ctx.isInvalid) {
    ctx.disk.process.exit(1);
  }
  return ctx;
}
