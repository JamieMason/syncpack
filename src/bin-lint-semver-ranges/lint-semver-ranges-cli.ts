import type { Disk } from '../lib/disk';
import { exitIfInvalid } from '../lib/exit-if-invalid';
import { getContext } from '../lib/get-context';
import type { Config } from '../lib/get-context/get-config/config';
import { lintSemverRanges } from './lint-semver-ranges';

export function lintSemverRangesCli(
  input: Partial<Config.All>,
  disk: Disk,
): void {
  exitIfInvalid(lintSemverRanges(getContext(input, disk)));
}
