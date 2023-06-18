import * as Effect from '@effect/io/Effect';
import { lintSemverRanges } from '../../../src/bin-lint-semver-ranges/lint-semver-ranges';
import { list } from '../../../src/bin-list/list';
import { prompt } from '../../../src/bin-prompt/prompt';
import { setSemverRanges } from '../../../src/bin-set-semver-ranges/set-semver-ranges';
import { toBeSemverRangeMismatch, toBeValid } from '../../matchers/semver-group';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

describe('semverGroups', () => {
  describe('SemverRangeMismatch', () => {
    [
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { packageManager: 'foo@2.0.0' } }),
              after: mockPackage('a', { otherProps: { packageManager: 'foo@*' } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { packageManager: 'bar@*' } }),
              after: mockPackage('b', { otherProps: { packageManager: 'bar@*' } }),
            },
          ],
          {
            cli: {},
            rcFile: {
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
                  range: '*',
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
              before: mockPackage('a', { otherProps: { deps: { custom: { foo: '2.0.0' } } } }),
              after: mockPackage('a', { otherProps: { deps: { custom: { foo: '*' } } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { deps: { custom: { bar: '*' } } } }),
              after: mockPackage('b', { otherProps: { deps: { custom: { bar: '*' } } } }),
            },
          ],
          {
            cli: {},
            rcFile: {
              customTypes: {
                custom: {
                  strategy: 'versionsByName',
                  path: 'deps.custom',
                },
              },
              semverRange: '*',
            },
          },
        ),
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { deps: { custom: { foo: '2.0.0' } } } }),
              after: mockPackage('a', { otherProps: { deps: { custom: { foo: '*' } } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { deps: { custom: { bar: '*' } } } }),
              after: mockPackage('b', { otherProps: { deps: { custom: { bar: '*' } } } }),
            },
          ],
          {
            cli: {},
            rcFile: {
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
              semverRange: '*',
            },
          },
        ),
      ...['deps', 'devDeps', 'overrides', 'peerDeps', 'pnpmOverrides', 'resolutions'].map(
        (type: string) => () =>
          createScenario(
            [
              {
                path: 'packages/a/package.json',
                before: mockPackage('a', { [type]: ['foo@2.0.0'] }),
                after: mockPackage('a', { [type]: ['foo@*'] }),
              },
              {
                path: 'packages/b/package.json',
                before: mockPackage('b', { [type]: ['bar@*'] }),
                after: mockPackage('b', { [type]: ['bar@*'] }),
              },
            ],
            {
              cli: {},
              rcFile: {
                semverGroups: [
                  {
                    dependencies: ['**'],
                    packages: ['**'],
                    range: '*',
                  },
                ],
              },
            },
          ),
      ),
    ].forEach((getScenario) => {
      describe('semverGroup.inspect()', () => {
        test('should identify as a mismatch', () => {
          const scenario = getScenario();
          expect(scenario.report.semverGroups).toEqual([
            [
              toBeValid({ name: 'bar' }),
              toBeSemverRangeMismatch({ expectedVersion: '*', name: 'foo' }),
            ],
          ]);
        });
      });

      describe('set-semver-ranges', () => {
        test('should fix the mismatch', () => {
          const scenario = getScenario();
          Effect.runSync(setSemverRanges({}, scenario.env));
          expect(scenario.env.exitProcess).not.toHaveBeenCalled();
          expect(scenario.env.writeFileSync.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].diskWriteWhenChanged,
          ]);
        });
      });

      describe('lint-semver-ranges', () => {
        test('should exit with 1 on the mismatch', () => {
          const scenario = getScenario();
          Effect.runSync(lintSemverRanges({}, scenario.env));
          expect(scenario.env.exitProcess).toHaveBeenCalledWith(1);
        });
      });

      describe('list', () => {
        test('does not exit with 1 on semver range issues', () => {
          const scenario = getScenario();
          Effect.runSync(list({}, scenario.env));
          expect(scenario.env.exitProcess).not.toHaveBeenCalled();
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
