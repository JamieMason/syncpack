import * as Effect from '@effect/io/Effect';
import { fixMismatches } from '../../../src/bin-fix-mismatches/fix-mismatches';
import { lint } from '../../../src/bin-lint/lint';
import { listMismatches } from '../../../src/bin-list-mismatches/list-mismatches';
import { list } from '../../../src/bin-list/list';
import { prompt } from '../../../src/bin-prompt/prompt';
import { toBeBanned } from '../../matchers/version-group';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

describe('versionGroups', () => {
  describe('Banned', () => {
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
                  dependencies: ['**'],
                  packages: ['**'],
                  isBanned: true,
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
            cli: {},
            rcFile: {
              customTypes: {
                engines: {
                  strategy: 'versionsByName',
                  path: 'customDeps',
                },
              },
              dependencyTypes: ['**'],
              versionGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  isBanned: true,
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
            cli: {},
            rcFile: {
              customTypes: {
                engines: {
                  strategy: 'version',
                  path: 'customDeps.yarn',
                },
              },
              dependencyTypes: ['**'],
              versionGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  isBanned: true,
                },
              ],
            },
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
              cli: {},
              rcFile: {
                dependencyTypes: ['**'],
                versionGroups: [
                  {
                    dependencies: ['**'],
                    packages: ['**'],
                    isBanned: true,
                  },
                ],
              },
            },
          ),
      ),
    ].forEach((getScenario) => {
      describe('versionGroup.inspect()', () => {
        test('should identify as banned', () => {
          const scenario = getScenario();
          expect(scenario.report.versionGroups).toEqual([[toBeBanned({ name: 'yarn' })]]);
        });
      });

      describe('fix-mismatches', () => {
        test('should fix the mismatch', () => {
          const scenario = getScenario();
          Effect.runSync(fixMismatches({}, scenario.env));
          expect(scenario.env.exitProcess).not.toHaveBeenCalled();
          expect(scenario.env.writeFileSync.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].diskWriteWhenChanged,
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
