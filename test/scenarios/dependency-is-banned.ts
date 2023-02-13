import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - A does not depend on `bar`
 * - B does depend on `bar`
 * - `bar` is banned in every package from being installed
 * - `bar` should be removed from B
 */
export function dependencyIsBanned() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', { deps: ['foo@0.1.0'] }),
        after: mockPackage('a', { deps: ['foo@0.1.0'] }),
      },
      {
        path: 'packages/b/package.json',
        before: mockPackage('b', { deps: ['bar@0.2.0'] }),
        after: mockPackage('b'),
      },
    ],
    {
      versionGroups: [
        {
          dependencies: ['bar'],
          dependencyTypes: [],
          packages: ['**'],
          isBanned: true,
        },
      ],
    },
  );
}
