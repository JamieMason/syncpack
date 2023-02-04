import type { Context } from '../lib/get-context';
import { isValidSemverRange } from '../lib/is-semver';
import { listSemverGroupMismatches } from '../lib/list-semver-group-mismatches';
import { setSemverRange } from '../lib/set-semver-range';

export const setSemverRanges = (ctx: Context): Context => {
  ctx.semverGroups.reverse().forEach((semverGroup) => {
    if ('range' in semverGroup && isValidSemverRange(semverGroup.range)) {
      const mismatches = listSemverGroupMismatches(semverGroup);
      mismatches.forEach(({ dependencyType, name, version, wrapper }) => {
        if (dependencyType === 'workspace') return;
        const root: any = wrapper.contents;
        const nextVersion = setSemverRange(semverGroup.range, version);
        if (dependencyType === 'pnpmOverrides') {
          root.pnpm.overrides[name] = nextVersion;
        } else {
          root[dependencyType][name] = nextVersion;
        }
      });
    }
  });

  return ctx;
};
