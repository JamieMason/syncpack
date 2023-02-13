import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - Only `dependencies` is unchecked
 * - The semver range `*` should be used
 * - A uses exact versions for `a`
 * - A should be fixed to use `*` in all other cases
 */
export function semverRangesDoNotMatchConfigWildcard() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', {
          deps: ['a@0.1.0'],
          devDeps: ['b@0.1.0'],
          overrides: ['c@0.1.0'],
          pnpmOverrides: ['d@0.1.0'],
          peerDeps: ['e@0.1.0'],
          resolutions: ['f@0.1.0'],
        }),
        after: mockPackage('a', {
          deps: ['a@0.1.0'],
          devDeps: ['*'],
          overrides: ['*'],
          pnpmOverrides: ['*'],
          peerDeps: ['*'],
          resolutions: ['*'],
        }),
      },
    ],
    {
      semverRange: '*',
      types: 'dev,overrides,pnpmOverrides,peer,resolutions,workspace',
    },
  );
}
