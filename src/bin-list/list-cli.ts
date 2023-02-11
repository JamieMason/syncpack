import { getContext } from '../get-context';
import type { Disk } from '../lib/disk';
import { exitIfInvalid } from '../lib/exit-if-invalid';
import type { Syncpack } from '../types';
import { list } from './list';

export function listCli(input: Partial<Syncpack.Config.Cli>, disk: Disk): void {
  exitIfInvalid(list(getContext(input, disk)));
}
