import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - A and B have versions syncpack does not support
 * - The versions match
 * - All packages should be left unchanged
 */
export function matchingUnsupportedVersions() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', { deps: ['foo@workspace:*'] }),
        after: mockPackage('a', { deps: ['foo@workspace:*'] }),
      },
      {
        path: 'packages/b/package.json',
        before: mockPackage('b', { deps: ['foo@workspace:*'] }),
        after: mockPackage('b', { deps: ['foo@workspace:*'] }),
      },
    ],
    {},
  );
}
