import type { Disk } from '../lib/disk';
import { exitIfInvalid } from '../lib/exit-if-invalid';
import { getContext } from '../lib/get-context';
import type { Syncpack } from '../types';
import { listMismatches } from './list-mismatches';

export function listMismatchesCli(
  input: Partial<Syncpack.Config.Cli>,
  disk: Disk,
): void {
  exitIfInvalid(listMismatches(getContext(input, disk)));
}
