import * as Effect from '@effect/io/Effect';
import { lintSemverRanges } from '../../../src/bin-lint-semver-ranges/lint-semver-ranges';
import { list } from '../../../src/bin-list/list';
import { prompt } from '../../../src/bin-prompt/prompt';
import { setSemverRanges } from '../../../src/bin-set-semver-ranges/set-semver-ranges';
import { toBeLocalPackageSemverRangeMismatch, toBeValid } from '../../matchers/semver-group';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

describe('semverGroups', () => {
  describe('WORKSPACE_RANGE_MISMATCH', () => {
    [
      () =>
        createScenario(
          [
            {
              path: 'packages/a/package.json',
              before: mockPackage('a', { otherProps: { packageManager: 'foo@>=0.1.1' } }),
              after: mockPackage('a', { otherProps: { packageManager: 'foo@>=0.1.1' } }),
            },
            {
              path: 'packages/b/package.json',
              before: mockPackage('b', { otherProps: { version: '>=0.1.1' } }),
              after: mockPackage('b', { otherProps: { version: '0.1.1' } }),
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
                  range: '>=',
                },
              ],
            },
          },
        ),
    ].forEach((getScenario) => {
      describe('semverGroup.inspect()', () => {
        test("should identify as a workspace mismatch as a package's version must be exact", () => {
          const scenario = getScenario();
          expect(scenario.report.semverGroups).toEqual([
            [
              toBeLocalPackageSemverRangeMismatch({ expectedVersion: '0.1.1', name: 'b' }),
              toBeValid({ name: 'foo' }),
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
            scenario.files['packages/b/package.json'].diskWriteWhenChanged,
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
