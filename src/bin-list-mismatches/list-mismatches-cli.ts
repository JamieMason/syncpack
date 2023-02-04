import type { Disk } from '../lib/disk';
import { exitIfInvalid } from '../lib/exit-if-invalid';
import { getContext } from '../lib/get-context';
import type { Config } from '../lib/get-context/get-config/config';
import { listMismatches } from './list-mismatches';

export function listMismatchesCli(
  input: Partial<Config.All>,
  disk: Disk,
): void {
  exitIfInvalid(listMismatches(getContext(input, disk)));
}
