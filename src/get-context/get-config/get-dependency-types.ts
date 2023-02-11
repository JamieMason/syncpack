import { isBoolean } from 'expect-more';
import { ALL_DEPENDENCY_TYPES } from '../../constants';
import type { Syncpack } from '../../types';

export function getDependencyTypes(
  fromCli: Partial<Syncpack.Config.Cli>,
  resolved: Syncpack.Config.Public,
): Syncpack.Config.Private['dependencyTypes'] {
  const dependencyTypes: Syncpack.Config.Private['dependencyTypes'] = [];
  const hasTypeOverride =
    isBoolean(fromCli.dev) ||
    isBoolean(fromCli.overrides) ||
    isBoolean(fromCli.peer) ||
    isBoolean(fromCli.pnpmOverrides) ||
    isBoolean(fromCli.prod) ||
    isBoolean(fromCli.resolutions) ||
    isBoolean(fromCli.workspace);

  if (hasTypeOverride) {
    resolved.dev = Boolean(fromCli.dev);
    resolved.overrides = Boolean(fromCli.overrides);
    resolved.peer = Boolean(fromCli.peer);
    resolved.pnpmOverrides = Boolean(fromCli.pnpmOverrides);
    resolved.prod = Boolean(fromCli.prod);
    resolved.resolutions = Boolean(fromCli.resolutions);
    resolved.workspace = Boolean(fromCli.workspace);
  }

  resolved.dev && dependencyTypes.push('devDependencies');
  resolved.overrides && dependencyTypes.push('overrides');
  resolved.peer && dependencyTypes.push('peerDependencies');
  resolved.pnpmOverrides && dependencyTypes.push('pnpmOverrides');
  resolved.prod && dependencyTypes.push('dependencies');
  resolved.resolutions && dependencyTypes.push('resolutions');
  resolved.workspace && dependencyTypes.push('workspace');

  if (dependencyTypes.length === 0) {
    dependencyTypes.push(...ALL_DEPENDENCY_TYPES);
  }

  return dependencyTypes;
}
