import type { Context } from '../lib/get-context';

export const setSemverRanges = (ctx: Context): Context => {
  ctx.semverGroups.reverse().forEach((semverGroup) => {
    semverGroup.instances.forEach((instance) => {
      instance.setRange(semverGroup.range);
    });
  });

  return ctx;
};
