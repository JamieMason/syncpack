import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/** "repository" contains a github URL which can be shortened further */
export function githubShorthand() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', {
          omitName: true,
          otherProps: {
            repository: {
              url: 'git://github.com/User/repo',
              type: 'git',
            },
          },
        }),
        after: mockPackage('a', {
          omitName: true,
          otherProps: {
            repository: 'User/repo',
          },
        }),
      },
    ],
    {},
  );
}
