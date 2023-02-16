import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - A, B, C & D depend on foo
 * - The versions mismatch
 * - Some versions are not semver
 * - `0.3.0` is the highest valid semver version
 * - All packages should be fixed to use `0.3.0`
 */
export function mismatchesIncludeNonSemverVersions() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', { deps: ['foo@link:vendor/foo-0.1.0'] }),
        after: mockPackage('a', { deps: ['foo@0.3.0'] }),
      },
      {
        path: 'packages/b/package.json',
        before: mockPackage('b', { deps: ['foo@workspace:*'] }),
        after: mockPackage('b', { deps: ['foo@0.3.0'] }),
      },
      {
        path: 'packages/c/package.json',
        before: mockPackage('c', { deps: ['foo@0.3.0'] }),
        after: mockPackage('c', { deps: ['foo@0.3.0'] }),
      },
      {
        path: 'packages/d/package.json',
        before: mockPackage('d', { deps: ['foo@0.2.0'] }),
        after: mockPackage('d', { deps: ['foo@0.3.0'] }),
      },
    ],
    {},
  );
}
