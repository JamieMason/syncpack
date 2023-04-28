import { fixMismatchesCli } from '../../../src/bin-fix-mismatches/fix-mismatches-cli';
import { listMismatchesCli } from '../../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../../src/bin-list/list-cli';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

describe('versionGroups', () => {
  describe('has mismatches but they are excluded by dependencyTypes', () => {
    [
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { packageManager: 'yarn@2.0.0' } }),
              after: mockPackage('a', { otherProps: { packageManager: 'yarn@3.0.0' } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { packageManager: 'yarn@3.0.0' } }),
              after: mockPackage('b', { otherProps: { packageManager: 'yarn@3.0.0' } }),
            },
          ],
          {
            customTypes: {
              engines: {
                strategy: 'name@version',
                path: 'packageManager',
              },
            },
            versionGroups: [
              {
                dependencies: ['**'],
                dependencyTypes: ['prod'],
                packages: ['**'],
                preferVersion: 'highestSemver',
              },
              {
                dependencies: ['**'],
                packages: ['**'],
                isIgnored: true,
              },
            ],
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { deps: { custom: { yarn: '2.0.0' } } } }),
              after: mockPackage('a', { otherProps: { deps: { custom: { yarn: '3.0.0' } } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { deps: { custom: { yarn: '3.0.0' } } } }),
              after: mockPackage('b', { otherProps: { deps: { custom: { yarn: '3.0.0' } } } }),
            },
          ],
          {
            customTypes: {
              engines: {
                strategy: 'versionsByName',
                path: 'deps.custom',
              },
            },
            versionGroups: [
              {
                dependencies: ['**'],
                dependencyTypes: ['dev'],
                packages: ['**'],
                preferVersion: 'highestSemver',
              },
              {
                dependencies: ['**'],
                packages: ['**'],
                isIgnored: true,
              },
            ],
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { deps: { custom: { yarn: '2.0.0' } } } }),
              after: mockPackage('a', { otherProps: { deps: { custom: { yarn: '3.0.0' } } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { deps: { custom: { yarn: '3.0.0' } } } }),
              after: mockPackage('b', { otherProps: { deps: { custom: { yarn: '3.0.0' } } } }),
            },
          ],
          {
            customTypes: {
              engines: {
                strategy: 'version',
                path: 'deps.custom.yarn',
              },
            },
            versionGroups: [
              {
                dependencies: ['**'],
                dependencyTypes: ['workspace'],
                packages: ['**'],
                preferVersion: 'highestSemver',
              },
              {
                dependencies: ['**'],
                packages: ['**'],
                isIgnored: true,
              },
            ],
          },
        ),
      ...['deps', 'devDeps', 'overrides', 'peerDeps', 'pnpmOverrides', 'resolutions'].map(
        (type: string) => () =>
          createScenario(
            [
              {
                path: 'packages/a/package.json',
                before: mockPackage('a', { [type]: ['yarn@2.0.0'] }),
                after: mockPackage('a', { [type]: ['yarn@3.0.0'] }),
              },
              {
                path: 'packages/b/package.json',
                before: mockPackage('b', { [type]: ['yarn@3.0.0'] }),
                after: mockPackage('b', { [type]: ['yarn@3.0.0'] }),
              },
            ],
            {
              versionGroups: [
                {
                  dependencies: ['**'],
                  dependencyTypes: ['matchesNone'],
                  packages: ['**'],
                  preferVersion: 'highestSemver',
                },
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  isIgnored: true,
                },
              ],
            },
          ),
      ),
    ].forEach((getScenario) => {
      describe('versionGroup.inspect()', () => {
        test('should fall back to the 2nd isIgnored group', () => {
          expect(getScenario().report.versionGroups).toEqual([
            [
              expect.objectContaining({
                isValid: true,
                name: 'yarn',
                status: 'IGNORED',
              }),
            ],
          ]);
        });
      });

      describe('fix-mismatches', () => {
        test('should report as valid', () => {
          const scenario = getScenario();
          fixMismatchesCli({}, scenario.disk);
          expect(scenario.disk.process.exit).not.toHaveBeenCalled();
          expect(scenario.disk.writeFileSync).not.toHaveBeenCalled();
          expect(scenario.log.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
            scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
          ]);
        });
      });

      describe('list-mismatches', () => {
        test('should report as valid', () => {
          const scenario = getScenario();
          listMismatchesCli({}, scenario.disk);
          expect(scenario.disk.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('list', () => {
        test('should report as valid', () => {
          const scenario = getScenario();
          listCli({}, scenario.disk);
          expect(scenario.disk.process.exit).not.toHaveBeenCalled();
        });
      });
    });
  });
});
