import type { Syncpack } from '../types';

export const setSemverRanges = (ctx: Syncpack.Ctx): Syncpack.Ctx => {
  ctx.semverGroups.reverse().forEach((semverGroup) => {
    semverGroup.instances.forEach((instance) => {
      instance.setRange(semverGroup.range);
    });
  });

  return ctx;
};
