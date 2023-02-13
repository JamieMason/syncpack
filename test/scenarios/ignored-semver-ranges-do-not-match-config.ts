import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - All dependencies are checked
 * - The semver range `~` should be used
 * - `one` uses exact versions
 * - `b` and `c` are ignored
 * - All but `b` and `c` should use `~`
 */
export function ignoredSemverRangesDoNotMatchConfig() {
  return createScenario(
    [
      {
        path: 'packages/one/package.json',
        before: mockPackage('one', {
          deps: ['a@0.1.0'],
          devDeps: ['b@0.1.0'],
          overrides: ['c@0.1.0'],
          pnpmOverrides: ['d@0.1.0'],
          peerDeps: ['e@0.1.0'],
          resolutions: ['f@0.1.0'],
        }),
        after: mockPackage('two', {
          deps: ['a@~0.1.0'],
          devDeps: ['b@0.1.0'],
          overrides: ['c@0.1.0'],
          pnpmOverrides: ['d@~0.1.0'],
          peerDeps: ['e@~0.1.0'],
          resolutions: ['f@~0.1.0'],
        }),
      },
    ],
    {
      semverRange: '~',
      semverGroups: [
        {
          dependencies: ['b', 'c'],
          dependencyTypes: [],
          packages: ['**'],
          isIgnored: true,
        },
      ],
      types: 'dev,overrides,pnpmOverrides,peer,prod,resolutions,workspace',
    },
  );
}
