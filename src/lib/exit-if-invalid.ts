import type { Context } from './get-context';

export function exitIfInvalid(ctx: Context): Context {
  if (ctx.isInvalid) {
    ctx.disk.process.exit(1);
  }
  return ctx;
}
