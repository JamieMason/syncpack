import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - A does not depend on `bar`
 * - B does depend on `bar`
 * - `bar` is ignored by syncpack in every package
 * - `bar` is unprotected so can mismatch etc
 */
export function versionIsIgnored() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', { deps: ['foo@0.1.0', 'bar@1.1.1'] }),
        after: mockPackage('a', { deps: ['foo@0.1.0', 'bar@1.1.1'] }),
      },
      {
        path: 'packages/b/package.json',
        before: mockPackage('b', { deps: ['bar@0.2.0'] }),
        after: mockPackage('b', { deps: ['bar@0.2.0'] }),
      },
    ],
    {
      versionGroups: [
        {
          dependencies: ['bar'],
          dependencyTypes: [],
          packages: ['**'],
          isIgnored: true,
        },
      ],
    },
  );
}
