import { isBoolean, isObject } from 'expect-more';
import type { Syncpack } from '../../types';

type CorePaths = typeof corePaths;
export type CorePathName = keyof CorePaths;

const corePaths = {
  dev: {
    path: 'devDependencies',
    strategy: 'versionsByName',
  },
  overrides: {
    path: 'overrides',
    strategy: 'versionsByName',
  },
  peer: {
    path: 'peerDependencies',
    strategy: 'versionsByName',
  },
  pnpmOverrides: {
    path: 'pnpm.overrides',
    strategy: 'versionsByName',
  },
  prod: {
    path: 'dependencies',
    strategy: 'versionsByName',
  },
  resolutions: {
    path: 'resolutions',
    strategy: 'versionsByName',
  },
  workspace: {
    namePath: 'name',
    path: 'version',
    strategy: 'name~version',
  },
} as const;

export function getCorePaths(
  fromCli: Pick<Partial<Syncpack.Config.Cli>, CorePathName>,
): Syncpack.PathDefinition[] {
  const corePathNames = Object.keys(corePaths) as CorePathName[];
  const hasOverride = corePathNames.some((name) => isBoolean(fromCli[name]));

  return corePathNames
    .filter((name) => !hasOverride || fromCli[name] === true)
    .map(getByName)
    .filter(isObject<Syncpack.PathDefinition>);

  function getByName(key: CorePathName): Syncpack.PathDefinition | undefined {
    const obj = corePaths[key];
    if (obj) return { ...obj, name: key };
  }
}
