import type { Disk } from '../lib/disk';
import { getContext } from '../lib/get-context';
import { writeIfChanged } from '../lib/write-if-changed';
import type { Syncpack } from '../types';
import { setSemverRanges } from './set-semver-ranges';

export function setSemverRangesCli(
  input: Partial<Syncpack.Config.Cli>,
  disk: Disk,
): void {
  writeIfChanged(setSemverRanges(getContext(input, disk)));
}
