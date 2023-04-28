import { pipe } from 'tightrope/fn/pipe';
import type { CliConfig } from '../config/types';
import { getContext } from '../get-context';
import type { Disk } from '../lib/disk';
import { writeIfChanged } from '../lib/write-if-changed';
import { format } from './format';

export function formatCli(input: Partial<CliConfig>, disk: Disk): void {
  pipe(getContext(input, disk), format, writeIfChanged);
}
