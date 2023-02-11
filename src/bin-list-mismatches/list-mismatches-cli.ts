import { getContext } from '../get-context';
import type { Disk } from '../lib/disk';
import { exitIfInvalid } from '../lib/exit-if-invalid';
import type { Syncpack } from '../types';
import { listMismatches } from './list-mismatches';

export function listMismatchesCli(
  input: Partial<Syncpack.Config.Cli>,
  disk: Disk,
): void {
  exitIfInvalid(listMismatches(getContext(input, disk)));
}
