import type { Ctx } from '../get-context';
import { isValidSemverRange } from '../guards/is-valid-semver-range';
import type { SemverRange } from './types';

export function getSemverRange({ cli, rcFile }: Ctx['config']): SemverRange {
  return isValidSemverRange(cli.semverRange)
    ? cli.semverRange
    : isValidSemverRange(rcFile.semverRange)
    ? rcFile.semverRange
    : '';
}
