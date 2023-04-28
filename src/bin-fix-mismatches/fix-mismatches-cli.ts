import { pipe } from 'tightrope/fn/pipe';
import type { CliConfig } from '../config/types';
import { getContext } from '../get-context';
import type { Disk } from '../lib/disk';
import { exitIfInvalid } from '../lib/exit-if-invalid';
import { writeIfChanged } from '../lib/write-if-changed';
import { fixMismatches } from './fix-mismatches';

export function fixMismatchesCli(input: Partial<CliConfig>, disk: Disk): void {
  pipe(getContext(input, disk), fixMismatches, writeIfChanged, exitIfInvalid);
}
