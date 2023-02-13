import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - A has an npm override of C
 * - B has an npm override of C
 * - The versions do not match
 * - The highest semver version wins
 */
export function dependentDoesNotMatchNpmOverrideVersion() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', { pnpmOverrides: ['c@0.1.0'] }),
        after: mockPackage('a', { pnpmOverrides: ['c@0.2.0'] }),
      },
      {
        path: 'packages/b/package.json',
        before: mockPackage('b', { pnpmOverrides: ['c@0.2.0'] }),
        after: mockPackage('b', { pnpmOverrides: ['c@0.2.0'] }),
      },
    ],
    {
      types: 'pnpmOverrides',
    },
  );
}
