import { fixMismatchesCli } from '../../../src/bin-fix-mismatches/fix-mismatches-cli';
import { lintCli } from '../../../src/bin-lint/lint-cli';
import { listMismatchesCli } from '../../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../../src/bin-list/list-cli';
import { DEFAULT_CONFIG } from '../../../src/constants';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

describe('versionGroups', () => {
  describe('HIGHEST_SEMVER_MISMATCH', () => {
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
                packages: ['**'],
                preferVersion: 'highestSemver',
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
                packages: ['**'],
                preferVersion: 'highestSemver',
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
                packages: ['**'],
                preferVersion: 'highestSemver',
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
                  dependencyTypes: [...DEFAULT_CONFIG.dependencyTypes],
                  label: 'Some group',
                  packages: ['**'],
                  preferVersion: 'highestSemver',
                },
              ],
            },
          ),
      ),
    ].forEach((getScenario) => {
      describe('versionGroup.inspect()', () => {
        test('should identify as a mismatch where the highest valid semver version wins', () => {
          const scenario = getScenario();
          expect(scenario.report.versionGroups).toEqual([
            [
              expect.objectContaining({
                expectedVersion: '3.0.0',
                isValid: false,
                name: 'yarn',
                status: 'HIGHEST_SEMVER_MISMATCH',
              }),
            ],
          ]);
        });
      });

      describe('fix-mismatches', () => {
        test('should fix the mismatch', () => {
          const scenario = getScenario();
          fixMismatchesCli({}, scenario.disk);
          expect(scenario.disk.process.exit).not.toHaveBeenCalled();
          expect(scenario.disk.writeFileSync.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].diskWriteWhenChanged,
          ]);
          expect(scenario.log.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].logEntryWhenChanged,
            scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
          ]);
        });
      });

      describe('list-mismatches', () => {
        test('should exit with 1 on the mismatch', () => {
          const scenario = getScenario();
          listMismatchesCli({}, scenario.disk);
          expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('lint', () => {
        test('should exit with 1 on the mismatch', () => {
          const scenario = getScenario();
          lintCli({}, scenario.disk);
          expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('list', () => {
        test('should exit with 1 on the mismatch', () => {
          const scenario = getScenario();
          listCli({}, scenario.disk);
          expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
        });
      });
    });
  });
});
