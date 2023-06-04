import { pipe } from 'tightrope/fn/pipe';
import type { CliConfig } from '../config/types';
import { getContext } from '../get-context';
import type { Effects } from '../lib/effects';
import { writeIfChanged } from '../lib/write-if-changed';
import { format } from './format';

export function formatCli(input: Partial<CliConfig>, effects: Effects): void {
  pipe(getContext(input, effects), format, writeIfChanged);
}
