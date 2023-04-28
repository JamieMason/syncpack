import { fixMismatchesCli } from '../../../src/bin-fix-mismatches/fix-mismatches-cli';
import { listMismatchesCli } from '../../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../../src/bin-list/list-cli';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

describe('versionGroups', () => {
  describe('BANNED', () => {
    [
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { packageManager: 'yarn@3.5.2' } }),
              after: mockPackage('a', { otherProps: {} }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { packageManager: 'yarn@1.0.1' } }),
              after: mockPackage('b', { otherProps: {} }),
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
                isBanned: true,
              },
            ],
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { customDeps: { yarn: '3.5.2' } } }),
              after: mockPackage('a', {}),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { customDeps: { yarn: '1.0.1' } } }),
              after: mockPackage('b', {}),
            },
          ],
          {
            customTypes: {
              engines: {
                strategy: 'versionsByName',
                path: 'customDeps',
              },
            },
            versionGroups: [
              {
                dependencies: ['**'],
                packages: ['**'],
                isBanned: true,
              },
            ],
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { customDeps: { yarn: '3.5.2' } } }),
              after: mockPackage('a', { otherProps: {} }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { customDeps: { yarn: '1.0.1' } } }),
              after: mockPackage('b', { otherProps: {} }),
            },
          ],
          {
            customTypes: {
              engines: {
                strategy: 'version',
                path: 'customDeps.yarn',
              },
            },
            versionGroups: [
              {
                dependencies: ['**'],
                packages: ['**'],
                isBanned: true,
              },
            ],
          },
        ),
      // @TODO remove empty pnpm.overrides after banning its only entry then
      // add 'pnpmOverrides' back in this test
      ...['deps', 'devDeps', 'overrides', 'peerDeps', 'resolutions'].map(
        (type: string) => () =>
          createScenario(
            [
              {
                path: 'packages/a/package.json',
                before: mockPackage('a', { [type]: ['yarn@3.5.2'] }),
                after: mockPackage('a', {}),
              },
              {
                path: 'packages/b/package.json',
                before: mockPackage('b', { [type]: ['yarn@1.0.1'] }),
                after: mockPackage('b', {}),
              },
            ],
            {
              versionGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  isBanned: true,
                },
              ],
            },
          ),
      ),
    ].forEach((getScenario) => {
      describe('versionGroup.inspect()', () => {
        test('should identify as banned', () => {
          const scenario = getScenario();
          expect(scenario.report.versionGroups).toEqual([
            [
              expect.objectContaining({
                isValid: false,
                name: 'yarn',
                status: 'BANNED',
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
            scenario.files['packages/b/package.json'].diskWriteWhenChanged,
          ]);
          expect(scenario.log.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].logEntryWhenChanged,
            scenario.files['packages/b/package.json'].logEntryWhenChanged,
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
