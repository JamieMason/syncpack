import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - C is developed in this monorepo, its version is `0.0.1`
 * - C's version is the single source of truth and should never be changed
 * - A depends on C incorrectly and should be fixed
 * - B depends on C incorrectly and should be fixed
 */
export function dependentDoesNotMatchWorkspaceVersion() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', { deps: ['c@0.1.0'] }),
        after: mockPackage('a', { deps: ['c@0.0.1'] }),
      },
      {
        path: 'packages/b/package.json',
        before: mockPackage('b', { devDeps: ['c@0.2.0'] }),
        after: mockPackage('b', { devDeps: ['c@0.0.1'] }),
      },
      {
        path: 'packages/c/package.json',
        before: mockPackage('c', {
          otherProps: { name: 'c', version: '0.0.1' },
        }),
        after: mockPackage('c', {
          otherProps: { name: 'c', version: '0.0.1' },
        }),
      },
    ],
    {},
  );
}
