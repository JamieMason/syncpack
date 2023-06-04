import { fixMismatchesCli } from '../../../src/bin-fix-mismatches/fix-mismatches-cli';
import { lintCli } from '../../../src/bin-lint/lint-cli';
import { listMismatchesCli } from '../../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../../src/bin-list/list-cli';
import { promptCli } from '../../../src/bin-prompt/prompt-cli';
import { createScenarioVariants } from './lib/create-scenario-variants';

describe('versionGroups', () => {
  describe('HIGHEST_SEMVER_MISMATCH', () => {
    createScenarioVariants({
      config: {
        versionGroups: [
          {
            dependencies: ['**'],
            packages: ['**'],
            preferVersion: 'highestSemver',
          },
        ],
      },
      a: ['yarn@2.0.0', 'yarn@3.0.0'],
      b: ['yarn@3.0.0', 'yarn@3.0.0'],
    }).forEach((getScenario) => {
      describe('versionGroup.inspect()', () => {
        test('should identify as a mismatch where the highest valid semver version wins', () => {
          const scenario = getScenario();
          expect(scenario.report.versionGroups).toEqual([
            [
              expect.objectContaining({
                expectedVersion: '3.0.0',
                isValid: false,
                name: 'yarn',
                status: 'HIGHEST_SEMVER_MISMATCH',
              }),
            ],
          ]);
        });
      });

      describe('fix-mismatches', () => {
        test('should fix the mismatch', () => {
          const scenario = getScenario();
          fixMismatchesCli({}, scenario.effects);
          expect(scenario.effects.process.exit).not.toHaveBeenCalled();
          expect(scenario.effects.writeFileSync.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].effectsWriteWhenChanged,
          ]);
          expect(scenario.log.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].logEntryWhenChanged,
            scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
          ]);
        });
      });

      describe('list-mismatches', () => {
        test('should exit with 1 on the mismatch', () => {
          const scenario = getScenario();
          listMismatchesCli({}, scenario.effects);
          expect(scenario.effects.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('lint', () => {
        test('should exit with 1 on the mismatch', () => {
          const scenario = getScenario();
          lintCli({}, scenario.effects);
          expect(scenario.effects.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('list', () => {
        test('should exit with 1 on the mismatch', () => {
          const scenario = getScenario();
          listCli({}, scenario.effects);
          expect(scenario.effects.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('prompt', () => {
        test('should have nothing to do', () => {
          const scenario = getScenario();
          promptCli({}, scenario.effects);
          expect(scenario.effects.askForChoice).not.toHaveBeenCalled();
          expect(scenario.effects.askForInput).not.toHaveBeenCalled();
        });
      });
    });
  });
});
