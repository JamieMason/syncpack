import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/** "bugs" and "repository" can safely use equivalent shorthands */
export function shorthand() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', {
          omitName: true,
          otherProps: {
            bugs: { url: 'https://github.com/User/repo/issues' },
            repository: { url: 'git://gitlab.com/User/repo', type: 'git' },
          },
        }),
        after: mockPackage('a', {
          omitName: true,
          otherProps: {
            bugs: 'https://github.com/User/repo/issues',
            repository: 'git://gitlab.com/User/repo',
          },
        }),
      },
    ],
    {},
  );
}
