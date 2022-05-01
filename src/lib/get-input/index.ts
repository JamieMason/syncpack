import type { SyncpackConfig } from '../../constants';
import type { Disk } from '../../lib/disk';
import { getConfig } from './get-config';
import type { Instances } from './get-instances';
import { getInstances } from './get-instances';
import type { SourceWrapper } from './get-wrappers';
import { getWrappers } from './get-wrappers';

export type ProgramInput = SyncpackConfig & {
  disk: Disk;
  instances: Instances;
  wrappers: SourceWrapper[];
};

/**
 * Every command in syncpack should accept the return value of this function as
 * its input. The aim here is to move all disk activity to a single place, so
 * that the majority of syncpack and its tests don't have to deal with the file
 * system and can focus solely on transformation logic.
 *
 * @param  program     Options received from CLI arguments
 */
export function getInput(
  disk: Disk,
  program: Partial<SyncpackConfig & { configPath: string }>,
): ProgramInput {
  const config = getConfig(disk, program);
  const wrappers = getWrappers(disk, config);
  const instances = getInstances(config, wrappers);
  return { ...config, disk, instances, wrappers };
}
