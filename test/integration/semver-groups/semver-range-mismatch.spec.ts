import * as Effect from '@effect/io/Effect';
import { lintSemverRanges } from '../../../src/bin-lint-semver-ranges/lint-semver-ranges';
import { list } from '../../../src/bin-list/list';
import { prompt } from '../../../src/bin-prompt/prompt';
import { setSemverRanges } from '../../../src/bin-set-semver-ranges/set-semver-ranges';
import { INTERNAL_TYPES } from '../../../src/constants';
import { toBeSemverRangeMismatch, toBeValid } from '../../lib/matchers/semver-group';
import { mockPackage } from '../../lib/mock';
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
              after: mockPackage('a', { otherProps: { packageManager: 'foo@>=2.0.0' } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { packageManager: 'bar@>=0.1.1' } }),
              after: mockPackage('b', { otherProps: { packageManager: 'bar@>=0.1.1' } }),
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
              dependencyTypes: ['**'],
              semverGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  range: '>=',
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
              after: mockPackage('a', { otherProps: { deps: { custom: { foo: '>=2.0.0' } } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { deps: { custom: { bar: '>=0.1.1' } } } }),
              after: mockPackage('b', { otherProps: { deps: { custom: { bar: '>=0.1.1' } } } }),
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
              dependencyTypes: ['**'],
              semverGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  range: '>=',
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
              after: mockPackage('a', { otherProps: { deps: { custom: { foo: '>=2.0.0' } } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { deps: { custom: { bar: '>=0.1.1' } } } }),
              after: mockPackage('b', { otherProps: { deps: { custom: { bar: '>=0.1.1' } } } }),
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
              dependencyTypes: ['**'],
              semverGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  range: '>=',
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
              cli: {},
              rcFile: {
                dependencyTypes: ['**'],
                semverGroups: [
                  {
                    dependencies: ['**'],
                    dependencyTypes: [...INTERNAL_TYPES],
                    label: 'Some group',
                    packages: ['**'],
                    range: '>=',
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
              toBeSemverRangeMismatch({ expectedVersion: '>=2.0.0', name: 'foo' }),
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
