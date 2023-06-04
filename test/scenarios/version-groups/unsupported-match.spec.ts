import { fixMismatchesCli } from '../../../src/bin-fix-mismatches/fix-mismatches-cli';
import { lintCli } from '../../../src/bin-lint/lint-cli';
import { listMismatchesCli } from '../../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../../src/bin-list/list-cli';
import { promptCli } from '../../../src/bin-prompt/prompt-cli';
import { createScenarioVariants } from './lib/create-scenario-variants';

describe('versionGroups', () => {
  describe('unsupported versions which match each other', () => {
    [
      'yarn@git://github.com/user/project.git#commit-ish',
      'yarn@link:vendor/yarn-3.5.2',
      'yarn@patch:yarn@16.11.0#patches/yarn+16.11.0.patch',
    ]
      .flatMap((version) =>
        createScenarioVariants({
          config: {
            semverGroups: [
              {
                dependencies: ['**'],
                packages: ['**'],
                isIgnored: true,
              },
            ],
          },
          a: [version, version],
          b: [version, version],
        }),
      )
      .forEach((getScenario) => {
        describe('versionGroup.inspect()', () => {
          test('should identify as valid because they match', () => {
            const scenario = getScenario();
            expect(scenario.report.versionGroups).toEqual([
              [
                expect.objectContaining({
                  isValid: true,
                  name: 'yarn',
                  status: 'VALID',
                }),
              ],
            ]);
          });
        });

        describe('fix-mismatches', () => {
          test('should report as valid', () => {
            const scenario = getScenario();
            fixMismatchesCli({}, scenario.disk);
            expect(scenario.disk.process.exit).not.toHaveBeenCalled();
            expect(scenario.disk.writeFileSync).not.toHaveBeenCalled();
            expect(scenario.log.mock.calls).toEqual([
              scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
              scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
            ]);
          });
        });

        describe('list-mismatches', () => {
          test('should report as valid', () => {
            const scenario = getScenario();
            listMismatchesCli({}, scenario.disk);
            expect(scenario.disk.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('lint', () => {
          test('should report as valid', () => {
            const scenario = getScenario();
            lintCli({}, scenario.disk);
            expect(scenario.disk.process.exit).not.toHaveBeenCalled();
          });
        });

        describe('list', () => {
          test('should report as valid', () => {
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
