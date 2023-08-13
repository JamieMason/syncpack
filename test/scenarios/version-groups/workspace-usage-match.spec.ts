import * as Effect from '@effect/io/Effect';
import { fixMismatches } from '../../../src/bin-fix-mismatches/fix-mismatches';
import { lint } from '../../../src/bin-lint/lint';
import { listMismatches } from '../../../src/bin-list-mismatches/list-mismatches';
import { list } from '../../../src/bin-list/list';
import { prompt } from '../../../src/bin-prompt/prompt';
import { toBeValid } from '../../matchers/version-group';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

describe('versionGroups', () => {
  describe('installs of a local package match each other but not the canonical source', () => {
    [
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { packageManager: 'c@>=1.0.0' } }),
              after: mockPackage('a', { otherProps: { packageManager: 'c@>=1.0.0' } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { packageManager: 'c@>=1.0.0' } }),
              after: mockPackage('b', { otherProps: { packageManager: 'c@>=1.0.0' } }),
            },
            {
              path: 'packages/c/package.json',
              before: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
              after: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
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
              semverGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  isIgnored: true,
                },
              ],
              versionGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  policy: 'sameRange',
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
              before: mockPackage('a', { otherProps: { deps: { custom: { c: 'workspace:*' } } } }),
              after: mockPackage('a', { otherProps: { deps: { custom: { c: 'workspace:*' } } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { deps: { custom: { c: 'workspace:*' } } } }),
              after: mockPackage('b', { otherProps: { deps: { custom: { c: 'workspace:*' } } } }),
            },
            {
              path: 'packages/c/package.json',
              before: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
              after: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
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
              semverGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  isIgnored: true,
                },
              ],
              versionGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  policy: 'sameRange',
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
              before: mockPackage('a', { otherProps: { deps: { custom: { c: '>=1.0.0' } } } }),
              after: mockPackage('a', { otherProps: { deps: { custom: { c: '>=1.0.0' } } } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { deps: { custom: { c: '>=1.0.0' } } } }),
              after: mockPackage('b', { otherProps: { deps: { custom: { c: '>=1.0.0' } } } }),
            },
            {
              path: 'packages/c/package.json',
              before: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
              after: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
            },
          ],
          {
            cli: {},
            rcFile: {
              customTypes: {
                engines: {
                  strategy: 'version',
                  path: 'deps.custom.c',
                },
              },
              semverGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  isIgnored: true,
                },
              ],
              versionGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  policy: 'sameRange',
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
              before: mockPackage('a', { deps: ['c@>=1.0.0'] }),
              after: mockPackage('a', { deps: ['c@>=1.0.0'] }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { devDeps: ['c@>=1.0.0'] }),
              after: mockPackage('b', { devDeps: ['c@>=1.0.0'] }),
            },
            {
              path: 'packages/c/package.json',
              before: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
              after: mockPackage('c', { otherProps: { name: 'c', version: '1.1.0' } }),
            },
          ],
          {
            cli: {},
            rcFile: {
              semverGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  isIgnored: true,
                },
              ],
              versionGroups: [
                {
                  dependencies: ['**'],
                  packages: ['**'],
                  policy: 'sameRange',
                },
              ],
            },
          },
        ),
    ].forEach((getScenario) => {
      describe('versionGroup.inspect()', () => {
        test('should identify as valid', () => {
          const scenario = getScenario();
          expect(scenario.report.versionGroups).toEqual([[toBeValid({ name: 'c' })]]);
        });
      });

      describe('fix-mismatches', () => {
        test('should report as valid', () => {
          const scenario = getScenario();
          Effect.runSync(fixMismatches({}, scenario.env));
          expect(scenario.env.exitProcess).not.toHaveBeenCalled();
          expect(scenario.env.writeFileSync).not.toHaveBeenCalled();
        });
      });

      describe('list-mismatches', () => {
        test('should report as valid', () => {
          const scenario = getScenario();
          Effect.runSync(listMismatches({}, scenario.env));
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
