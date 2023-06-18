import * as Effect from '@effect/io/Effect';
import { fixMismatches } from '../../../src/bin-fix-mismatches/fix-mismatches';
import { lint } from '../../../src/bin-lint/lint';
import { listMismatches } from '../../../src/bin-list-mismatches/list-mismatches';
import { list } from '../../../src/bin-list/list';
import { prompt } from '../../../src/bin-prompt/prompt';
import { toBePinnedMismatch } from '../../matchers/version-group';
import { createScenarioVariants } from './lib/create-scenario-variants';

describe('versionGroups', () => {
  describe('PinnedMismatch', () => {
    createScenarioVariants({
      config: {
        cli: {},
        rcFile: {
          versionGroups: [
            {
              dependencies: ['**'],
              packages: ['**'],
              pinVersion: '2.2.2',
            },
          ],
        },
      },
      a: ['yarn@2.0.0', 'yarn@2.2.2'],
      b: ['yarn@3.0.0', 'yarn@2.2.2'],
    }).forEach((getScenario) => {
      describe('versionGroup.inspect()', () => {
        test('should identify as a mismatch where the pinned version must be used', () => {
          const scenario = getScenario();
          expect(scenario.report.versionGroups).toEqual([
            [
              toBePinnedMismatch({
                expectedVersion: '2.2.2',

                name: 'yarn',
              }),
            ],
          ]);
        });
      });

      describe('fix-mismatches', () => {
        test('should fix the mismatch', () => {
          const scenario = getScenario();
          Effect.runSync(fixMismatches({}, scenario.env));
          expect(scenario.env.exitProcess).not.toHaveBeenCalled();
          expect(scenario.env.writeFileSync.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].diskWriteWhenChanged,
            scenario.files['packages/b/package.json'].diskWriteWhenChanged,
          ]);
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
