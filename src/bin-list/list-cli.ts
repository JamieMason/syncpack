import type { Disk } from '../lib/disk';
import { exitIfInvalid } from '../lib/exit-if-invalid';
import { getContext } from '../lib/get-context';
import type { TConfig } from '../types';
import { list } from './list';

export function listCli(input: Partial<TConfig.Cli>, disk: Disk): void {
  exitIfInvalid(list(getContext(input, disk)));
}
