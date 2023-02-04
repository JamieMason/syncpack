import type { Disk } from '../lib/disk';
import { getContext } from '../lib/get-context';
import type { Config } from '../lib/get-context/get-config/config';
import { writeIfChanged } from '../lib/write-if-changed';
import { setSemverRanges } from './set-semver-ranges';

export function setSemverRangesCli(
  input: Partial<Config.All>,
  disk: Disk,
): void {
  writeIfChanged(setSemverRanges(getContext(input, disk)));
}
