import * as Effect from '@effect/io/Effect';
import { fixMismatches } from '../../../src/bin-fix-mismatches/fix-mismatches';
import { lint } from '../../../src/bin-lint/lint';
import { listMismatches } from '../../../src/bin-list-mismatches/list-mismatches';
import { list } from '../../../src/bin-list/list';
import { prompt } from '../../../src/bin-prompt/prompt';
import { toBeIgnored } from '../../matchers/version-group';
import { createScenarioVariants } from './lib/create-scenario-variants';

describe('versionGroups', () => {
  describe('has mismatches but they are excluded by dependencyTypes', () => {
    createScenarioVariants({
      config: {
        cli: {},
        rcFile: {
          versionGroups: [
            {
              dependencies: ['**'],
              dependencyTypes: ['this-does-not-match-anything'],
              packages: ['**'],
              preferVersion: 'highestSemver',
            },
            {
              dependencies: ['**'],
              packages: ['**'],
              isIgnored: true,
            },
          ],
        },
      },
      a: ['yarn@2.0.0', 'yarn@3.0.0'],
      b: ['yarn@3.0.0', 'yarn@3.0.0'],
    }).forEach((getScenario) => {
      describe('versionGroup.inspect()', () => {
        test('should fall back to the 2nd isIgnored group', () => {
          expect(getScenario().report.versionGroups).toEqual([
            [
              toBeIgnored({
                name: 'yarn',
              }),
            ],
          ]);
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
