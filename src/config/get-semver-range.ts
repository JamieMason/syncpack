import type { Context } from '../get-context';
import { isValidSemverRange } from '../lib/is-semver';
import type { SemverRange } from './types';

export function getSemverRange({
  cli,
  rcFile,
}: Context['config']): SemverRange {
  return isValidSemverRange(cli.semverRange)
    ? cli.semverRange
    : isValidSemverRange(rcFile.semverRange)
    ? rcFile.semverRange
    : '';
}
