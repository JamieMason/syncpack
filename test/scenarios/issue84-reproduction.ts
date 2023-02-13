import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * @see https://github.com/JamieMason/syncpack/issues/84#issue-1284878219
 */
export function issue84Reproduction() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('@myscope/a', { deps: ['@myscope/a@1.0.0'] }),
        after: mockPackage('@myscope/a', { deps: ['@myscope/a@^1.0.0'] }),
      },
      {
        path: 'packages/b/package.json',
        before: mockPackage('@myscope/b', {}),
        after: mockPackage('@myscope/b', {}),
      },
    ],
    {
      semverGroups: [
        {
          range: '^',
          dependencies: ['@myscope/**'],
          dependencyTypes: [],
          packages: ['**'],
        },
      ],
      semverRange: '~',
      types: 'dev,overrides,pnpmOverrides,peer,prod,resolutions,workspace',
    },
  );
}
