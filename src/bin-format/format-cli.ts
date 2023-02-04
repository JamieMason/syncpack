import type { Disk } from '../lib/disk';
import { getContext } from '../lib/get-context';
import type { Config } from '../lib/get-context/get-config/config';
import { writeIfChanged } from '../lib/write-if-changed';
import { format } from './format';

export function formatCli(input: Partial<Config.All>, disk: Disk): void {
  writeIfChanged(format(getContext(input, disk)));
}
