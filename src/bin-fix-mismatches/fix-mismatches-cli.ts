import type { Disk } from '../lib/disk';
import { getContext } from '../lib/get-context';
import { writeIfChanged } from '../lib/write-if-changed';
import type { TConfig } from '../types';
import { fixMismatches } from './fix-mismatches';

export function fixMismatchesCli(
  input: Partial<TConfig.Cli>,
  disk: Disk,
): void {
  writeIfChanged(fixMismatches(getContext(input, disk)));
}
