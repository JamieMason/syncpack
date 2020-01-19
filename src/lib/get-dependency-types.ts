import { DEPENDENCY_TYPES } from '../constants';
import { CommanderApi, IManifestKey } from '../typings';

export const getDependencyTypes = (program: CommanderApi): IManifestKey[] =>
  program.prod || program.dev || program.peer
    ? DEPENDENCY_TYPES.filter(
        (type) =>
          (type === 'dependencies' && program.prod) ||
          (type === 'devDependencies' && program.dev) ||
          (type === 'peerDependencies' && program.peer),
      )
    : DEPENDENCY_TYPES;
