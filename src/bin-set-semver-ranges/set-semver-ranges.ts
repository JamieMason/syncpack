import type { Context } from '../get-context';
import { getSemverGroups } from '../get-semver-groups';

export const setSemverRanges = (ctx: Context): Context => {
  getSemverGroups(ctx).forEach((semverGroup) => {
    semverGroup.inspect().forEach((report) => {
      switch (report.status) {
        case 'WORKSPACE_SEMVER_RANGE_MISMATCH':
        case 'SEMVER_RANGE_MISMATCH': {
          report.instance.setVersion(report.expectedVersion);
          break;
        }
        case 'FILTERED_OUT':
        case 'IGNORED':
        case 'UNSUPPORTED_VERSION':
        case 'VALID': {
          // no action needed
          break;
        }
      }
    });
  });

  return ctx;
};
