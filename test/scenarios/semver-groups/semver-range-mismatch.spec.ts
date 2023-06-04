import { lintSemverRangesCli } from '../../../src/bin-lint-semver-ranges/lint-semver-ranges-cli';
import { listCli } from '../../../src/bin-list/list-cli';
import { promptCli } from '../../../src/bin-prompt/prompt-cli';
import { setSemverRangesCli } from '../../../src/bin-set-semver-ranges/set-semver-ranges-cli';
import { DEFAULT_CONFIG } from '../../../src/constants';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

describe('semverGroups', () => {
  describe('SEMVER_RANGE_MISMATCH', () => {
    [
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { packageManager: 'foo@2.0.0' } }),
              after: mockPackage('a', { otherProps: { packageManager: 'foo@>=2.0.0' } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { packageManager: 'bar@>=0.1.1' } }),
              after: mockPackage('b', { otherProps: { packageManager: 'bar@>=0.1.1' } }),
            },
          ],
          {
            customTypes: {
              packageManager: {
                strategy: 'name@version',
                path: 'packageManager',
              },
            },
            semverGroups: [
              {
                dependencies: ['**'],
                packages: ['**'],
                range: '>=',
              },
            ],
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { deps: { custom: { foo: '2.0.0' } } } }),
              after: mockPackage('a', { otherProps: { deps: { custom: { foo: '>=2.0.0' } } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { deps: { custom: { bar: '>=0.1.1' } } } }),
              after: mockPackage('b', { otherProps: { deps: { custom: { bar: '>=0.1.1' } } } }),
            },
          ],
          {
            customTypes: {
              custom: {
                strategy: 'versionsByName',
                path: 'deps.custom',
              },
            },
            semverGroups: [
              {
                dependencies: ['**'],
                packages: ['**'],
                range: '>=',
              },
            ],
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { deps: { custom: { foo: '2.0.0' } } } }),
              after: mockPackage('a', { otherProps: { deps: { custom: { foo: '>=2.0.0' } } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { deps: { custom: { bar: '>=0.1.1' } } } }),
              after: mockPackage('b', { otherProps: { deps: { custom: { bar: '>=0.1.1' } } } }),
            },
          ],
          {
            customTypes: {
              foo: {
                strategy: 'version',
                path: 'deps.custom.foo',
              },
              bar: {
                strategy: 'version',
                path: 'deps.custom.bar',
              },
            },
            semverGroups: [
              {
                dependencies: ['**'],
                packages: ['**'],
                range: '>=',
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
                before: mockPackage('a', { [type]: ['foo@2.0.0'] }),
                after: mockPackage('a', { [type]: ['foo@>=2.0.0'] }),
              },
              {
                path: 'packages/b/package.json',
                before: mockPackage('b', { [type]: ['bar@>=0.1.1'] }),
                after: mockPackage('b', { [type]: ['bar@>=0.1.1'] }),
              },
            ],
            {
              semverGroups: [
                {
                  dependencies: ['**'],
                  dependencyTypes: [...DEFAULT_CONFIG.dependencyTypes],
                  label: 'Some group',
                  packages: ['**'],
                  range: '>=',
                },
              ],
            },
          ),
      ),
    ].forEach((getScenario) => {
      describe('semverGroup.inspect()', () => {
        test('should identify as a mismatch', () => {
          const scenario = getScenario();
          expect(scenario.report.semverGroups).toEqual([
            [
              expect.objectContaining({
                expectedVersion: '>=2.0.0',
                isValid: false,
                name: 'foo',
                status: 'SEMVER_RANGE_MISMATCH',
              }),
              expect.objectContaining({
                isValid: true,
                name: 'bar',
                status: 'VALID',
              }),
            ],
          ]);
        });
      });

      describe('set-semver-ranges', () => {
        test('should fix the mismatch', () => {
          const scenario = getScenario();
          setSemverRangesCli({}, scenario.disk);
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

      describe('lint-semver-ranges', () => {
        test('should exit with 1 on the mismatch', () => {
          const scenario = getScenario();
          lintSemverRangesCli({}, scenario.disk);
          expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('list', () => {
        test('does not exit with 1 on semver range issues', () => {
          const scenario = getScenario();
          listCli({}, scenario.disk);
          expect(scenario.disk.process.exit).not.toHaveBeenCalled();
        });
      });

      describe('prompt', () => {
        test('should have nothing to do', () => {
          const scenario = getScenario();
          promptCli({}, scenario.disk);
          expect(scenario.disk.askForChoice).not.toHaveBeenCalled();
          expect(scenario.disk.askForInput).not.toHaveBeenCalled();
        });
      });
    });
  });
});
