import { getContext } from '../get-context';
import type { Disk } from '../lib/disk';
import { writeIfChanged } from '../lib/write-if-changed';
import type { Syncpack } from '../types';
import { format } from './format';

export function formatCli(
  input: Partial<Syncpack.Config.Cli>,
  disk: Disk,
): void {
  writeIfChanged(format(getContext(input, disk)));
}
