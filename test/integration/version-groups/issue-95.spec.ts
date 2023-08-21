import * as Effect from '@effect/io/Effect';
import { fixMismatches } from '../../../src/bin-fix-mismatches/fix-mismatches';
import { lint } from '../../../src/bin-lint/lint';
import { listMismatches } from '../../../src/bin-list-mismatches/list-mismatches';
import { list } from '../../../src/bin-list/list';
import { prompt } from '../../../src/bin-prompt/prompt';
import { toBeValid } from '../../lib/matchers/version-group';
import { mockPackage } from '../../lib/mock';
import { createScenario } from '../lib/create-scenario';

describe('versionGroups', () => {
  describe('https://github.com/JamieMason/syncpack/issues/95 reproduction', () => {
    [
      () =>
        createScenario(
          [
            {
              path: 'packages/api/package.json',
              before: mockPackage('api', { deps: ['@pnpm-syncpack/shared@workspace:*'] }),
              after: mockPackage('api', { deps: ['@pnpm-syncpack/shared@workspace:*'] }),
            },
            {
              path: 'packages/app/package.json',
              before: mockPackage('app', { deps: ['@pnpm-syncpack/shared@workspace:*'] }),
              after: mockPackage('app', { deps: ['@pnpm-syncpack/shared@workspace:*'] }),
            },
            {
              path: 'packages/shared/package.json',
              before: mockPackage('@pnpm-syncpack/shared', {
                otherProps: { name: '@pnpm-syncpack/shared', version: '1.0.0' },
              }),
              after: mockPackage('@pnpm-syncpack/shared', {
                otherProps: { name: '@pnpm-syncpack/shared', version: '1.0.0' },
              }),
            },
          ],
          {
            cli: {},
            rcFile: {
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
        test('should identify as valid because "workspace:*" is allowed to mismatch the canonical package version', () => {
          const scenario = getScenario();
          expect(scenario.report.versionGroups).toEqual([
            [
              toBeValid({
                name: '@pnpm-syncpack/shared',
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
