import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - .syncpackrc has a custom type defined to check the "packageManager" property.
 * - A has yarn@2
 * - B has yarn@3
 * - A should be fixed to use yarn@3
 */
export function customNameAndVersionMismatch() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', {
          otherProps: { packageManager: 'yarn@2.0.0' },
        }),
        after: mockPackage('a', {
          otherProps: { packageManager: 'yarn@3.0.0' },
        }),
      },
      {
        path: 'packages/b/package.json',
        before: mockPackage('b', {
          otherProps: { packageManager: 'yarn@3.0.0' },
        }),
        after: mockPackage('b', {
          otherProps: { packageManager: 'yarn@3.0.0' },
        }),
      },
    ],
    {
      customTypes: {
        engines: {
          strategy: 'name@version',
          path: 'packageManager',
        },
      },
    },
  );
}
