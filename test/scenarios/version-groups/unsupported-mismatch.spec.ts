import { fixMismatchesCli } from '../../../src/bin-fix-mismatches/fix-mismatches-cli';
import { lintCli } from '../../../src/bin-lint/lint-cli';
import { listMismatchesCli } from '../../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../../src/bin-list/list-cli';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

describe('versionGroups', () => {
  describe('UNSUPPORTED_MISMATCH', () => {
    [
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', {
                otherProps: { packageManager: 'yarn@link:vendor/yarn-0.1.0' },
              }),
              after: mockPackage('a', {
                otherProps: { packageManager: 'yarn@link:vendor/yarn-0.1.0' },
              }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', {
                otherProps: { packageManager: 'yarn@link:vendor/yarn-1.0.1' },
              }),
              after: mockPackage('b', {
                otherProps: { packageManager: 'yarn@link:vendor/yarn-1.0.1' },
              }),
            },
          ],
          {
            customTypes: {
              engines: {
                strategy: 'name@version',
                path: 'packageManager',
              },
            },
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', {
                otherProps: {
                  customDeps: { yarn: 'git://github.com/user/project.git#commit-ish' },
                },
              }),
              after: mockPackage('a', {
                otherProps: {
                  customDeps: { yarn: 'git://github.com/user/project.git#commit-ish' },
                },
              }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', {
                otherProps: {
                  customDeps: { yarn: 'git://github.com/user/project.git#some-commit' },
                },
              }),
              after: mockPackage('b', {
                otherProps: {
                  customDeps: { yarn: 'git://github.com/user/project.git#some-commit' },
                },
              }),
            },
          ],
          {
            customTypes: {
              engines: {
                strategy: 'versionsByName',
                path: 'customDeps',
              },
            },
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', {
                otherProps: { customDeps: { yarn: 'patch:yarn@3.5.2#patches/yarn.patch' } },
              }),
              after: mockPackage('a', {
                otherProps: { customDeps: { yarn: 'patch:yarn@3.5.2#patches/yarn.patch' } },
              }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', {
                otherProps: { customDeps: { yarn: 'patch:yarn@1.0.1#patches/yarn.patch' } },
              }),
              after: mockPackage('b', {
                otherProps: { customDeps: { yarn: 'patch:yarn@1.0.1#patches/yarn.patch' } },
              }),
            },
          ],
          {
            customTypes: {
              engines: {
                strategy: 'version',
                path: 'customDeps.yarn',
              },
            },
          },
        ),
      ...['deps', 'devDeps', 'overrides', 'peerDeps', 'pnpmOverrides', 'resolutions'].map(
        (type: string) => () =>
          createScenario(
            [
              {
                path: 'packages/a/package.json',
                before: mockPackage('a', { [type]: ['yarn@link:vendor/yarn-3.5.2'] }),
                after: mockPackage('a', { [type]: ['yarn@link:vendor/yarn-3.5.2'] }),
              },
              {
                path: 'packages/b/package.json',
                before: mockPackage('b', { [type]: ['yarn@link:vendor/yarn-1.0.1'] }),
                after: mockPackage('b', { [type]: ['yarn@link:vendor/yarn-1.0.1'] }),
              },
            ],
            {},
          ),
      ),
    ].forEach((getScenario) => {
      describe('versionGroup.inspect()', () => {
        test('should identify as mismatching, but not possible to fix', () => {
          const scenario = getScenario();
          expect(scenario.report.versionGroups).toEqual([
            [
              expect.objectContaining({
                isValid: false,
                name: 'yarn',
                status: 'UNSUPPORTED_MISMATCH',
              }),
            ],
          ]);
        });
      });

      describe('fix-mismatches', () => {
        test('should exit with 1 on the unfixable mismatch', () => {
          const scenario = getScenario();
          fixMismatchesCli({}, scenario.disk);
          expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
          expect(scenario.disk.writeFileSync).not.toHaveBeenCalled();
          expect(scenario.log.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
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
