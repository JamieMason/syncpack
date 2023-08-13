import { toBeSemverRangeMismatch } from '../../matchers/semver-group';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

/**
 * @see https://github.com/JamieMason/syncpack/issues/84#issue-1284878219
 */
test('Issue 84 reproduction', () => {
  expect(getScenario().report.semverGroups).toEqual([
    [
      toBeSemverRangeMismatch({
        expectedVersion: '^1.0.0',
        name: '@myscope/a',
      } as any),
    ],
  ]);

  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('@myscope/a', { deps: ['@myscope/a@1.0.0'] }),
          after: mockPackage('@myscope/a', { deps: ['@myscope/a@^1.0.0'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('@myscope/b', {}),
          after: mockPackage('@myscope/b', {}),
        },
      ],
      {
        cli: {},
        rcFile: {
          semverGroups: [
            {
              range: '^',
              dependencies: ['@myscope/**'],
              dependencyTypes: ['**'],
              packages: ['**'],
            },
          ],
          semverRange: '~',
          dependencyTypes: [
            'dev',
            'overrides',
            'pnpmOverrides',
            'peer',
            'prod',
            'resolutions',
            'local',
          ],
        },
      },
    );
  }
});
