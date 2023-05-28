import { pipe } from 'tightrope/fn/pipe';
import type { CliConfig } from '../config/types';
import { getContext } from '../get-context';
import type { Disk } from '../lib/disk';
import { exitIfInvalid } from '../lib/exit-if-invalid';
import { lint } from './lint';

export function lintCli(input: Partial<CliConfig>, disk: Disk): void {
  pipe(getContext(input, disk), lint, exitIfInvalid);
}
