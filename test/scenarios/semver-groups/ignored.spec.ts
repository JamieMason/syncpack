import * as Effect from '@effect/io/Effect';
import { lintSemverRanges } from '../../../src/bin-lint-semver-ranges/lint-semver-ranges';
import { lint } from '../../../src/bin-lint/lint';
import { list } from '../../../src/bin-list/list';
import { prompt } from '../../../src/bin-prompt/prompt';
import { setSemverRanges } from '../../../src/bin-set-semver-ranges/set-semver-ranges';
import { toBeIgnored } from '../../matchers/semver-group';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

describe('semverGroups', () => {
  describe('Ignored', () => {
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
              dependencyTypes: ['**'],
              semverGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  isIgnored: true,
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
              dependencyTypes: ['**'],
              semverGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  isIgnored: true,
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
                  isIgnored: true,
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
                dependencyTypes: ['**'],
                semverGroups: [
                  {
                    dependencies: ['**'],
                    packages: ['**'],
                    isIgnored: true,
                  },
                ],
              },
            },
          ),
      ),
    ].forEach((getScenario) => {
      describe('semverGroup.inspect()', () => {
        test('should identify as ignored', () => {
          const scenario = getScenario();
          expect(scenario.report.semverGroups).toEqual([
            [toBeIgnored({ name: 'bar' }), toBeIgnored({ name: 'foo' })],
          ]);
        });
      });

      describe('set-semver-ranges', () => {
        test('should report as valid', () => {
          const scenario = getScenario();
          Effect.runSync(setSemverRanges({}, scenario.env));
          expect(scenario.env.exitProcess).not.toHaveBeenCalled();
          expect(scenario.env.writeFileSync).not.toHaveBeenCalled();
        });
      });

      describe('lint-semver-ranges', () => {
        test('should report as valid', () => {
          const scenario = getScenario();
          Effect.runSync(lintSemverRanges({}, scenario.env));
          expect(scenario.env.exitProcess).not.toHaveBeenCalled();
        });
      });

      describe('lint', () => {
        test('should report as valid', () => {
          const scenario = getScenario();
          Effect.runSync(lint({}, scenario.env));
          expect(scenario.env.exitProcess).not.toHaveBeenCalled();
        });
      });

      describe('list', () => {
        test('should report as valid', () => {
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
