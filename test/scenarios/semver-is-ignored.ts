import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - A does not depend on `bar`
 * - B does depend on `bar`
 * - `bar` is ignored by syncpack in every package
 * - `foo` is not ignored
 * - `bar` is unprotected so can have mismatching range etc
 * - only `foo` should have its semver range fixed
 */
export function semverIsIgnored() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', { deps: ['foo@0.1.0', 'bar@1.1.1'] }),
        after: mockPackage('a', { deps: ['foo@~0.1.0', 'bar@1.1.1'] }),
      },
      {
        path: 'packages/b/package.json',
        before: mockPackage('b', { deps: ['bar@0.2.0'] }),
        after: mockPackage('b', { deps: ['bar@0.2.0'] }),
      },
    ],
    {
      semverRange: '~',
      semverGroups: [
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
