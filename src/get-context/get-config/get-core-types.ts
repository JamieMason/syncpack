import type { Syncpack } from '../../types';

type CoreTypes = typeof coreTypes;
export type CoreTypeName = keyof CoreTypes;

const coreTypes = {
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

export function getCoreTypes(): Syncpack.PathDefinition[] {
  return Object.entries(coreTypes).map(
    ([name, pathDef]): Syncpack.PathDefinition => ({ ...pathDef, name }),
  );
}
