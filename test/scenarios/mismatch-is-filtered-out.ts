import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - A depends on C
 * - B depends on C
 * - The versions do not match
 * - A filter is set to only look at dependency D
 * - The mismatches should be ignored
 */
export function mismatchIsFilteredOut() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', { deps: ['c@0.1.0'] }),
        after: mockPackage('a', { deps: ['c@0.1.0'] }),
      },
      {
        path: 'packages/b/package.json',
        before: mockPackage('b', { deps: ['c@0.2.0', 'd@1.1.1'] }),
        after: mockPackage('b', { deps: ['c@0.2.0', 'd@1.1.1'] }),
      },
    ],
    {
      filter: 'd',
    },
  );
}
