import * as Effect from '@effect/io/Effect';
import 'expect-more-jest';
import { fixMismatches } from '../../../src/bin-fix-mismatches/fix-mismatches';
import { lint } from '../../../src/bin-lint/lint';
import { listMismatches } from '../../../src/bin-list-mismatches/list-mismatches';
import { list } from '../../../src/bin-list/list';
import { prompt } from '../../../src/bin-prompt/prompt';
import { toBeNonSemverMismatch } from '../../matchers/version-group';
import { createScenarioVariants } from './lib/create-scenario-variants';

describe('versionGroups', () => {
  describe('NonSemverMismatch', () => {
    const cases: [string, string][] = [
      [
        'yarn@git://github.com/user/project.git#commit1',
        'yarn@git://github.com/user/project.git#commit2',
      ],
      ['yarn@link:vendor/yarn-3.5.2', 'yarn@link:vendor/yarn-1.0.1'],
      [
        'yarn@patch:yarn@1.1.0#patches/yarn+1.1.0.patch',
        'yarn@patch:yarn@0.2.0#patches/yarn+0.2.0.patch',
      ],
    ];
    cases
      .flatMap(([versionA, versionB]) =>
        createScenarioVariants({
          config: { cli: {}, rcFile: {} },
          a: [versionA, versionA],
          b: [versionB, versionB],
        }),
      )
      .forEach((getScenario) => {
        describe('versionGroup.inspect()', () => {
          test('should identify as mismatching, but not possible to fix', () => {
            const scenario = getScenario();
            expect(scenario.report.versionGroups).toEqual([
              [toBeNonSemverMismatch({ name: 'yarn' })],
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
