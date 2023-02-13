import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * Variation of `dependentDoesNotMatchWorkspaceVersion` in a nested workspace.
 *
 * - C is developed in this monorepo, its version is `0.0.1`
 * - C's version is the single source of truth and should never be changed
 * - A and B depend on C incorrectly and should be fixed
 * - A, B, and C are in nested workspaces
 *
 * @see https://github.com/goldstack/goldstack/pull/170/files#diff-7ae45ad102eab3b6d7e7896acd08c427a9b25b346470d7bc6507b6481575d519R19
 * @see https://github.com/JamieMason/syncpack/pull/74
 * @see https://github.com/JamieMason/syncpack/issues/66
 */
export function dependentDoesNotMatchNestedWorkspaceVersion() {
  return createScenario(
    [
      {
        path: 'workspaces/a/packages/a/package.json',
        before: mockPackage('a', { deps: ['c@0.1.0'] }),
        after: mockPackage('a', { deps: ['c@0.0.1'] }),
      },
      {
        path: 'workspaces/b/packages/b/package.json',
        before: mockPackage('b', { devDeps: ['c@0.2.0'] }),
        after: mockPackage('b', { devDeps: ['c@0.0.1'] }),
      },
      {
        path: 'workspaces/b/packages/c/package.json',
        before: mockPackage('c', {
          otherProps: { name: 'c', version: '0.0.1' },
        }),
        after: mockPackage('c', {
          otherProps: { name: 'c', version: '0.0.1' },
        }),
      },
    ],
    {
      types: 'dev,prod,workspace',
      source: [
        'package.json',
        'workspaces/*/package.json',
        'workspaces/*/packages/*/package.json',
      ],
    },
  );
}
