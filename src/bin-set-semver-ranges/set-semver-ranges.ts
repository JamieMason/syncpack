import type { Syncpack } from '../types';

export const setSemverRanges = (ctx: Syncpack.Ctx): Syncpack.Ctx => {
  ctx.semverGroups.forEach((semverGroup) => {
    semverGroup.instances.forEach((instance) => {
      instance.setVersion(semverGroup.getExpectedVersion(instance));
    });
  });

  return ctx;
};
