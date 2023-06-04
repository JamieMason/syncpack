import { pipe } from 'tightrope/fn/pipe';
import type { CliConfig } from '../config/types';
import { getContext } from '../get-context';
import type { Effects } from '../lib/effects';
import { exitIfInvalid } from '../lib/exit-if-invalid';
import { writeIfChanged } from '../lib/write-if-changed';
import { fixMismatches } from './fix-mismatches';

export function fixMismatchesCli(
  input: Partial<CliConfig>,
  effects: Effects,
): void {
  pipe(
    getContext(input, effects),
    fixMismatches,
    writeIfChanged,
    exitIfInvalid,
  );
}
