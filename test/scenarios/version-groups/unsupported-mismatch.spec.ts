import 'expect-more-jest';
import { fixMismatchesCli } from '../../../src/bin-fix-mismatches/fix-mismatches-cli';
import { lintCli } from '../../../src/bin-lint/lint-cli';
import { listMismatchesCli } from '../../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../../src/bin-list/list-cli';
import { promptCli } from '../../../src/bin-prompt/prompt-cli';
import { createScenarioVariants } from './lib/create-scenario-variants';

describe('versionGroups', () => {
  describe('UNSUPPORTED_MISMATCH', () => {
    [
      [
        'yarn@git://github.com/user/project.git#commit1',
        'yarn@git://github.com/user/project.git#commit2',
      ],
      ['yarn@link:vendor/yarn-3.5.2', 'yarn@link:vendor/yarn-1.0.1'],
      [
        'yarn@patch:yarn@1.1.0#patches/yarn+1.1.0.patch',
        'yarn@patch:yarn@0.2.0#patches/yarn+0.2.0.patch',
      ],
    ]
      .flatMap(([versionA, versionB]) =>
        createScenarioVariants({
          config: {},
          a: [versionA, versionA],
          b: [versionB, versionB],
        }),
      )
      .forEach((getScenario) => {
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

        describe('prompt', () => {
          test('should ask the user to choose the correct version', () => {
            const scenario = getScenario();
            promptCli({}, scenario.disk);
            expect(scenario.disk.askForChoice).toHaveBeenCalled();
            expect(scenario.disk.askForInput).not.toHaveBeenCalled();
          });
        });
      });
  });
});
