import type { Disk } from '../lib/disk';
import { exitIfInvalid } from '../lib/exit-if-invalid';
import { getContext } from '../lib/get-context';
import type { TConfig } from '../types';
import { lintSemverRanges } from './lint-semver-ranges';

export function lintSemverRangesCli(
  input: Partial<TConfig.Cli>,
  disk: Disk,
): void {
  exitIfInvalid(lintSemverRanges(getContext(input, disk)));
}
