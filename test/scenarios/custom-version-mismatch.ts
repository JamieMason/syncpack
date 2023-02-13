import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - .syncpackrc has a custom type defined to check the "somePlugin.version" property.
 * - A has 2.0.0
 * - B has 3.0.0
 * - A should be fixed to use 3.0.0
 */
export function customVersionMismatch() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', {
          otherProps: { somePlugin: { version: '2.0.0' } },
        }),
        after: mockPackage('a', {
          otherProps: { somePlugin: { version: '3.0.0' } },
        }),
      },
      {
        path: 'packages/b/package.json',
        before: mockPackage('b', {
          otherProps: { somePlugin: { version: '3.0.0' } },
        }),
        after: mockPackage('b', {
          otherProps: { somePlugin: { version: '3.0.0' } },
        }),
      },
    ],
    {
      customTypes: {
        engines: {
          strategy: 'version',
          path: 'somePlugin.version',
        },
      },
    },
  );
}
