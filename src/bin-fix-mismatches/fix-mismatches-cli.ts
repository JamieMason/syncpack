import { getContext } from '../get-context';
import type { Disk } from '../lib/disk';
import { writeIfChanged } from '../lib/write-if-changed';
import type { Syncpack } from '../types';
import { fixMismatches } from './fix-mismatches';

export function fixMismatchesCli(
  input: Partial<Syncpack.Config.Cli>,
  disk: Disk,
): void {
  writeIfChanged(fixMismatches(getContext(input, disk)));
}
