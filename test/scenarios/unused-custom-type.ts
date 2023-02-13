import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - .syncpackrc has a custom type defined to check the "engines" property.
 * - A has node 14
 * - B has node 16
 * - A should normally be fixed to use 16, but .syncpackrc has filtered
 *   "dependencyTypes" to only check the "dependencies" property
 * - Mismatch should be ignored
 */
export function unusedCustomType() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', {
          otherProps: { engines: { node: '14.0.0' } },
        }),
        after: mockPackage('a', {
          otherProps: { engines: { node: '14.0.0' } },
        }),
      },
      {
        path: 'packages/b/package.json',
        before: mockPackage('b', {
          otherProps: { engines: { node: '16.0.0' } },
        }),
        after: mockPackage('b', {
          otherProps: { engines: { node: '14.0.0' } },
        }),
      },
    ],
    {
      types: 'prod',
      customTypes: {
        engines: {
          strategy: 'versionsByName',
          path: 'engines',
        },
      },
    },
  );
}
