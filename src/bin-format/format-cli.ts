import type { Disk } from '../lib/disk';
import { getContext } from '../lib/get-context';
import { writeIfChanged } from '../lib/write-if-changed';
import type { Syncpack } from '../types';
import { format } from './format';

export function formatCli(
  input: Partial<Syncpack.Config.Cli>,
  disk: Disk,
): void {
  writeIfChanged(format(getContext(input, disk)));
}
