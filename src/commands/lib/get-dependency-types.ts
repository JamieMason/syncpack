import { DependencyType, DEPENDENCY_TYPES } from '../../constants';

interface Options {
  prod: boolean;
  dev: boolean;
  peer: boolean;
  resolutions: boolean;
  overrides: boolean;
}

export const getDependencyTypes = (program: Options): DependencyType[] =>
  program.prod || program.dev || program.peer
    ? DEPENDENCY_TYPES.filter(
        (type) =>
          (type === 'dependencies' && program.prod) ||
          (type === 'devDependencies' && program.dev) ||
          (type === 'resolutions' && program.resolutions) ||
          (type === 'overrides' && program.overrides) ||
          (type === 'peerDependencies' && program.peer),
      )
    : DEPENDENCY_TYPES;
