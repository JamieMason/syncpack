import { normalize } from 'path';
import { lintSemverRangesCli } from '../../src/bin-lint-semver-ranges/lint-semver-ranges-cli';

import { setSemverRangesCli } from '../../src/bin-set-semver-ranges/set-semver-ranges-cli';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - .syncpackrc has multiple custom types defined to run on every package
 * - Each semver group applies to one custom type
 * - All of the semver groups should run and fix
 */
describe('Custom types and semver groups', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', {
            otherProps: {
              packageManager: 'yarn@~2.0.0',
              engines: {
                node: '16.16.0',
                npm: '7.24.2',
              },
            },
          }),
          after: mockPackage('a', {
            otherProps: {
              packageManager: 'yarn@2.0.0',
              engines: {
                node: '>=16.16.0',
                npm: '^7.24.2',
              },
            },
          }),
        },
      ],
      {
        customTypes: {
          enginesNpm: {
            path: 'engines.npm',
            strategy: 'version',
          },
          enginesNode: {
            path: 'engines.node',
            strategy: 'version',
          },
          packageManager: {
            path: 'packageManager',
            strategy: 'name@version',
          },
        },
        semverGroups: [
          {
            dependencyTypes: ['enginesNode'],
            dependencies: ['**'],
            packages: ['**'],
            range: '>=',
          },
          {
            dependencyTypes: ['enginesNpm'],
            dependencies: ['**'],
            packages: ['**'],
            range: '^',
          },
          {
            dependencyTypes: ['packageManager'],
            dependencies: ['**'],
            packages: ['**'],
            range: '',
          },
        ],
      },
    );
  }

  describe('lint-semver-ranges', () => {
    it('list issues from multiple custom types and semver groups together', () => {
      const scenario = getScenario();
      const a = normalize('packages/a/package.json');
      lintSemverRangesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Semver Group 1/)],
        ['✘ node'],
        [`  16.16.0 → >=16.16.0 in engines.node of ${a}`],
        [expect.stringMatching(/Semver Group 2/)],
        ['✘ npm'],
        [`  7.24.2 → ^7.24.2 in engines.npm of ${a}`],
        [expect.stringMatching(/Semver Group 3/)],
        ['✘ yarn'],
        [`  ~2.0.0 → 2.0.0 in packageManager of ${a}`],
      ]);
    });
  });

  describe('set-semver-ranges', () => {
    it('fixes multiple custom types and semver groups together', () => {
      const scenario = getScenario();
      setSemverRangesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenChanged,
      ]);
    });
  });
});
