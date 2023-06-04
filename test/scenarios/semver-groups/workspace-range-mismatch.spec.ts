import { lintSemverRangesCli } from '../../../src/bin-lint-semver-ranges/lint-semver-ranges-cli';
import { listCli } from '../../../src/bin-list/list-cli';
import { promptCli } from '../../../src/bin-prompt/prompt-cli';
import { setSemverRangesCli } from '../../../src/bin-set-semver-ranges/set-semver-ranges-cli';
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
    ].forEach((getScenario) => {
      describe('semverGroup.inspect()', () => {
        test("should identify as a workspace mismatch as a package's version must be exact", () => {
          const scenario = getScenario();
          expect(scenario.report.semverGroups).toEqual([
            [
              expect.objectContaining({
                isValid: true,
                name: 'foo',
                status: 'VALID',
              }),
              expect.objectContaining({
                expectedVersion: '0.1.1',
                isValid: false,
                name: 'b',
                status: 'WORKSPACE_SEMVER_RANGE_MISMATCH',
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
            scenario.files['packages/b/package.json'].diskWriteWhenChanged,
          ]);
          expect(scenario.log.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
            scenario.files['packages/b/package.json'].logEntryWhenChanged,
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
