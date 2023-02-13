import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/** "bar" should always be 2.2.2 but is not */
export function dependencyIsPinned() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', { deps: ['bar@0.2.0'] }),
        after: mockPackage('a', { deps: ['bar@2.2.2'] }),
      },
    ],
    {
      versionGroups: [
        {
          dependencies: ['bar'],
          dependencyTypes: [],
          packages: ['**'],
          pinVersion: '2.2.2',
        },
      ],
    },
  );
}
