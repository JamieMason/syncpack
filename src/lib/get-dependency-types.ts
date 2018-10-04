import { CommanderStatic } from 'commander';
import { DEPENDENCY_TYPES } from '../constants';
import { IManifestKey } from '../typings';

export type GetDependencyTypes = (program: CommanderStatic) => IManifestKey[];

export const getDependencyTypes: GetDependencyTypes = (program) =>
  program.prod || program.dev || program.peer
    ? DEPENDENCY_TYPES.filter(
        (type) =>
          (type === 'dependencies' && program.prod) ||
          (type === 'devDependencies' && program.dev) ||
          (type === 'peerDependencies' && program.peer)
      )
    : DEPENDENCY_TYPES;
