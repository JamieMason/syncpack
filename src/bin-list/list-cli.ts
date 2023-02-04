import type { Disk } from '../lib/disk';
import { exitIfInvalid } from '../lib/exit-if-invalid';
import { getContext } from '../lib/get-context';
import type { Config } from '../lib/get-context/get-config/config';
import { list } from './list';

export function listCli(input: Partial<Config.All>, disk: Disk): void {
  exitIfInvalid(list(getContext(input, disk)));
}
