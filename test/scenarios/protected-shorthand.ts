import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/** "repository" contains properties which cannot be shortened */
export function protectedShorthand() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', {
          omitName: true,
          otherProps: {
            repository: {
              url: 'git://gitlab.com/User/repo',
              type: 'git',
              directory: 'packages/foo',
            },
          },
        }),
        after: mockPackage('a', {
          omitName: true,
          otherProps: {
            repository: {
              url: 'git://gitlab.com/User/repo',
              type: 'git',
              directory: 'packages/foo',
            },
          },
        }),
      },
    ],
    {},
  );
}
