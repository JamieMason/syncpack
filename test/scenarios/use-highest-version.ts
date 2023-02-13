import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/** "bar" should be 0.3.0, which is the highest installed version */
export function useHighestVersion() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', { deps: ['bar@0.2.0'] }),
        after: mockPackage('a', { deps: ['bar@0.3.0'] }),
      },
      {
        path: 'packages/b/package.json',
        before: mockPackage('b', { deps: ['bar@0.3.0'] }),
        after: mockPackage('b', { deps: ['bar@0.3.0'] }),
      },
      {
        path: 'packages/c/package.json',
        before: mockPackage('c', { deps: ['bar@0.1.0'] }),
        after: mockPackage('c', { deps: ['bar@0.3.0'] }),
      },
    ],
    {},
  );
}
