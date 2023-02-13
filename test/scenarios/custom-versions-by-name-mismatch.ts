import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - .syncpackrc has a custom type defined to check the "engines" property.
 * - A has node 14
 * - B has node 16
 * - A should be fixed to use 16
 */
export function customVersionsByNameMismatch() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', {
          otherProps: { engines: { node: '14.0.0' } },
        }),
        after: mockPackage('a', {
          otherProps: { engines: { node: '16.0.0' } },
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
      customTypes: {
        engines: {
          strategy: 'versionsByName',
          path: 'engines',
        },
      },
    },
  );
}
