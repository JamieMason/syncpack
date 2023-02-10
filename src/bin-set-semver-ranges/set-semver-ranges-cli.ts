import type { Disk } from '../lib/disk';
import { getContext } from '../lib/get-context';
import { writeIfChanged } from '../lib/write-if-changed';
import type { TConfig } from '../types';
import { setSemverRanges } from './set-semver-ranges';

export function setSemverRangesCli(
  input: Partial<TConfig.Cli>,
  disk: Disk,
): void {
  writeIfChanged(setSemverRanges(getContext(input, disk)));
}
