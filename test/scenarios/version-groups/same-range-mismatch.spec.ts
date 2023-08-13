import * as Effect from '@effect/io/Effect';
import { fixMismatches } from '../../../src/bin-fix-mismatches/fix-mismatches';
import { lint } from '../../../src/bin-lint/lint';
import { listMismatches } from '../../../src/bin-list-mismatches/list-mismatches';
import { list } from '../../../src/bin-list/list';
import { prompt } from '../../../src/bin-prompt/prompt';
import { toBeSameRangeMismatch } from '../../matchers/version-group';
import { createScenarioVariants } from './lib/create-scenario-variants';

describe('versionGroups', () => {
  describe('SameRangeMismatch', () => {
    createScenarioVariants({
      config: {
        cli: {},
        rcFile: {
          dependencyTypes: ['**'],
          versionGroups: [
            {
              dependencies: ['**'],
              packages: ['**'],
              policy: 'sameRange',
            },
          ],
        },
      },
      a: ['yarn@<=2.0.0', 'yarn@<=2.0.0'],
      b: ['yarn@>=3.0.0', 'yarn@>=3.0.0'],
    }).forEach((getScenario) => {
      describe('versionGroup.inspect()', () => {
        test('should identify as a mismatch where not every semver range includes all the others', () => {
          const scenario = getScenario();
          expect(scenario.report.versionGroups).toEqual([
            [toBeSameRangeMismatch({ name: 'yarn' })],
          ]);
        });
      });

      describe('fix-mismatches', () => {
        test('should exit with 1 on the unfixable mismatch', () => {
          const scenario = getScenario();
          Effect.runSync(fixMismatches({}, scenario.env));
          expect(scenario.env.exitProcess).toHaveBeenCalledWith(1);
          expect(scenario.env.writeFileSync).not.toHaveBeenCalled();
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
        test('should ask the user to choose the correct version', async () => {
          const scenario = getScenario();
          await Effect.runPromise(prompt({}, scenario.env));
          expect(scenario.env.askForChoice).toHaveBeenCalled();
          expect(scenario.env.askForInput).not.toHaveBeenCalled();
        });
      });
    });
  });
});
