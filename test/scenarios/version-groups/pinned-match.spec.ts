import { fixMismatchesCli } from '../../../src/bin-fix-mismatches/fix-mismatches-cli';
import { lintCli } from '../../../src/bin-lint/lint-cli';
import { listMismatchesCli } from '../../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../../src/bin-list/list-cli';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

describe('versionGroups', () => {
  describe('pinVersion already matches', () => {
    [
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { packageManager: 'yarn@2.2.2' } }),
              after: mockPackage('a', { otherProps: { packageManager: 'yarn@2.2.2' } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { packageManager: 'yarn@2.2.2' } }),
              after: mockPackage('b', { otherProps: { packageManager: 'yarn@2.2.2' } }),
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
                pinVersion: '2.2.2',
              },
            ],
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { deps: { custom: { yarn: '2.2.2' } } } }),
              after: mockPackage('a', { otherProps: { deps: { custom: { yarn: '2.2.2' } } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { deps: { custom: { yarn: '2.2.2' } } } }),
              after: mockPackage('b', { otherProps: { deps: { custom: { yarn: '2.2.2' } } } }),
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
                pinVersion: '2.2.2',
              },
            ],
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { deps: { custom: { yarn: '2.2.2' } } } }),
              after: mockPackage('a', { otherProps: { deps: { custom: { yarn: '2.2.2' } } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { deps: { custom: { yarn: '2.2.2' } } } }),
              after: mockPackage('b', { otherProps: { deps: { custom: { yarn: '2.2.2' } } } }),
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
                pinVersion: '2.2.2',
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
                before: mockPackage('a', { [type]: ['yarn@2.2.2'] }),
                after: mockPackage('a', { [type]: ['yarn@2.2.2'] }),
              },
              {
                path: 'packages/b/package.json',
                before: mockPackage('b', { [type]: ['yarn@2.2.2'] }),
                after: mockPackage('b', { [type]: ['yarn@2.2.2'] }),
              },
            ],
            {
              versionGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  pinVersion: '2.2.2',
                },
              ],
            },
          ),
      ),
    ].forEach((getScenario) => {
      describe('versionGroup.inspect()', () => {
        test('should identify as valid', () => {
          const scenario = getScenario();
          expect(scenario.report.versionGroups).toEqual([
            [
              expect.objectContaining({
                isValid: true,
                name: 'yarn',
                status: 'VALID',
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

      describe('lint', () => {
        test('should report as valid', () => {
          const scenario = getScenario();
          lintCli({}, scenario.disk);
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
