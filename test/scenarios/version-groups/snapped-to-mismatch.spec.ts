import * as Effect from '@effect/io/Effect';
import { fixMismatches } from '../../../src/bin-fix-mismatches/fix-mismatches';
import { lint } from '../../../src/bin-lint/lint';
import { listMismatches } from '../../../src/bin-list-mismatches/list-mismatches';
import { list } from '../../../src/bin-list/list';
import { prompt } from '../../../src/bin-prompt/prompt';
import { toBeSnappedToMismatch, toBeValid } from '../../matchers/version-group';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

describe('versionGroups', () => {
  describe('SnappedToMismatch', () => {
    [
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { packageManager: 'react@15.6.1' } }),
              after: mockPackage('a', { otherProps: { packageManager: 'react@15.6.1' } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { packageManager: 'react@18.0.0' } }),
              after: mockPackage('b', { otherProps: { packageManager: 'react@15.6.1' } }),
            },
            {
              path: 'packages/c/package.json',
              before: mockPackage('c', { deps: ['foo@0.1.0'] }),
              after: mockPackage('c', { deps: ['foo@0.1.0'] }),
            },
          ],
          {
            cli: {},
            rcFile: {
              customTypes: {
                engines: {
                  strategy: 'name@version',
                  path: 'packageManager',
                },
              },
              dependencyTypes: ['**'],
              versionGroups: [
                {
                  dependencies: ['react'],
                  packages: ['**'],
                  snapTo: ['a'],
                },
              ],
            },
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { deps: { custom: { react: '15.6.1' } } } }),
              after: mockPackage('a', { otherProps: { deps: { custom: { react: '15.6.1' } } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { deps: { custom: { react: '18.0.0' } } } }),
              after: mockPackage('b', { otherProps: { deps: { custom: { react: '15.6.1' } } } }),
            },
            {
              path: 'packages/c/package.json',
              before: mockPackage('c', { deps: ['foo@0.1.0'] }),
              after: mockPackage('c', { deps: ['foo@0.1.0'] }),
            },
          ],
          {
            cli: {},
            rcFile: {
              customTypes: {
                engines: {
                  strategy: 'versionsByName',
                  path: 'deps.custom',
                },
              },
              dependencyTypes: ['**'],
              versionGroups: [
                {
                  dependencies: ['react'],
                  packages: ['**'],
                  snapTo: ['a'],
                },
              ],
            },
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { customDeps: { react: '15.6.1' } } }),
              after: mockPackage('a', { otherProps: { customDeps: { react: '15.6.1' } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { customDeps: { react: '18.0.0' } } }),
              after: mockPackage('b', { otherProps: { customDeps: { react: '15.6.1' } } }),
            },
            {
              path: 'packages/c/package.json',
              before: mockPackage('c', { deps: ['foo@0.1.0'] }),
              after: mockPackage('c', { deps: ['foo@0.1.0'] }),
            },
          ],
          {
            cli: {},
            rcFile: {
              customTypes: {
                engines: {
                  strategy: 'version',
                  path: 'customDeps.react',
                },
              },
              dependencyTypes: ['**'],
              versionGroups: [
                {
                  dependencies: ['react'],
                  packages: ['**'],
                  snapTo: ['a'],
                },
              ],
            },
          },
        ),
      ...['deps', 'devDeps', 'overrides', 'peerDeps', 'pnpmOverrides', 'resolutions'].map(
        (type: string) => () =>
          createScenario(
            [
              {
                path: 'packages/a/package.json',
                before: mockPackage('a', { [type]: ['react@15.6.1'] }),
                after: mockPackage('a', { [type]: ['react@15.6.1'] }),
              },
              {
                path: 'packages/b/package.json',
                before: mockPackage('b', { [type]: ['react@18.0.0'] }),
                after: mockPackage('b', { [type]: ['react@15.6.1'] }),
              },
              {
                path: 'packages/c/package.json',
                before: mockPackage('c', { [type]: ['foo@0.1.0'] }),
                after: mockPackage('c', { [type]: ['foo@0.1.0'] }),
              },
            ],
            {
              cli: {},
              rcFile: {
                dependencyTypes: ['**'],
                versionGroups: [
                  {
                    dependencies: ['react'],
                    packages: ['**'],
                    snapTo: ['a'],
                  },
                ],
              },
            },
          ),
      ),
    ].forEach((getScenario) => {
      describe('versionGroup.inspect()', () => {
        test('should identify as a mismatch where the version present in package "a" should be followed', () => {
          const scenario = getScenario();
          expect(scenario.report.versionGroups).toEqual([
            [toBeSnappedToMismatch({ expectedVersion: '15.6.1', name: 'react' })],
            [toBeValid({ name: 'foo' })],
          ]);
        });
      });

      describe('fix-mismatches', () => {
        test('should fix the mismatch', () => {
          const scenario = getScenario();
          Effect.runSync(fixMismatches({}, scenario.env));
          expect(scenario.env.exitProcess).not.toHaveBeenCalled();
          expect(scenario.env.writeFileSync.mock.calls).toEqual([
            scenario.files['packages/b/package.json'].diskWriteWhenChanged,
          ]);
        });
      });

      describe('list-mismatches', () => {
        test('should exit with 1 on the mismatch', () => {
          const scenario = getScenario();
          Effect.runSync(listMismatches({}, scenario.env));
          expect(scenario.env.exitProcess).toHaveBeenCalledWith(1);
        });
      });

      describe('lint', () => {
        test('should exit with 1 on the mismatch', () => {
          const scenario = getScenario();
          Effect.runSync(lint({}, scenario.env));
          expect(scenario.env.exitProcess).toHaveBeenCalledWith(1);
        });
      });

      describe('list', () => {
        test('should exit with 1 on the mismatch', () => {
          const scenario = getScenario();
          Effect.runSync(list({}, scenario.env));
          expect(scenario.env.exitProcess).toHaveBeenCalledWith(1);
        });
      });

      describe('prompt', () => {
        test('should have nothing to do', async () => {
          const scenario = getScenario();
          await Effect.runPromise(prompt({}, scenario.env));
          expect(scenario.env.askForChoice).not.toHaveBeenCalled();
          expect(scenario.env.askForInput).not.toHaveBeenCalled();
        });
      });
    });
  });
});
