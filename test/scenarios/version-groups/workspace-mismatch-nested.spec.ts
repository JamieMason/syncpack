import * as Effect from '@effect/io/Effect';
import { fixMismatches } from '../../../src/bin-fix-mismatches/fix-mismatches';
import { listMismatches } from '../../../src/bin-list-mismatches/list-mismatches';
import { list } from '../../../src/bin-list/list';
import { prompt } from '../../../src/bin-prompt/prompt';
import { toBeWorkspaceMismatch } from '../../matchers/version-group';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

describe('versionGroups', () => {
  describe('WorkspaceMismatch', () => {
    describe('some packages are nested within sub-folders on the file system', () => {
      [
        () =>
          createScenario(
            [
              {
                path: 'workspaces/a/packages/a/package.json',
                before: mockPackage('a', { otherProps: { packageManager: 'c@2.0.0' } }),
                after: mockPackage('a', { otherProps: { packageManager: 'c@0.0.1' } }),
              },
              {
                path: 'workspaces/b/packages/b/package.json',
                before: mockPackage('b', { otherProps: { packageManager: 'c@3.0.0' } }),
                after: mockPackage('b', { otherProps: { packageManager: 'c@0.0.1' } }),
              },
              {
                path: 'workspaces/b/packages/c/package.json',
                before: mockPackage('c', { otherProps: { name: 'c', version: '0.0.1' } }),
                after: mockPackage('c', { otherProps: { name: 'c', version: '0.0.1' } }),
              },
            ],
            {
              cli: {},
              rcFile: {
                customTypes: {
                  engines: {
                    strategy: 'name@version',
                    path: 'packageManager',
                  },
                },
                dependencyTypes: ['dev', 'engines', 'prod', 'workspace'],
                source: [
                  'package.json',
                  'workspaces/*/package.json',
                  'workspaces/*/packages/*/package.json',
                ],
              },
            },
          ),
        () =>
          createScenario(
            [
              {
                path: 'workspaces/a/packages/a/package.json',
                before: mockPackage('a', { otherProps: { deps: { custom: { c: '2.0.0' } } } }),
                after: mockPackage('a', { otherProps: { deps: { custom: { c: '0.0.1' } } } }),
              },
              {
                path: 'workspaces/b/packages/b/package.json',
                before: mockPackage('b', { otherProps: { deps: { custom: { c: '3.0.0' } } } }),
                after: mockPackage('b', { otherProps: { deps: { custom: { c: '0.0.1' } } } }),
              },
              {
                path: 'workspaces/b/packages/c/package.json',
                before: mockPackage('c', { otherProps: { name: 'c', version: '0.0.1' } }),
                after: mockPackage('c', { otherProps: { name: 'c', version: '0.0.1' } }),
              },
            ],
            {
              cli: {},
              rcFile: {
                customTypes: {
                  engines: {
                    strategy: 'versionsByName',
                    path: 'deps.custom',
                  },
                },
                dependencyTypes: ['dev', 'engines', 'prod', 'workspace'],
                source: [
                  'package.json',
                  'workspaces/*/package.json',
                  'workspaces/*/packages/*/package.json',
                ],
              },
            },
          ),
        () =>
          createScenario(
            [
              {
                path: 'workspaces/a/packages/a/package.json',
                before: mockPackage('a', { otherProps: { deps: { custom: { c: '2.0.0' } } } }),
                after: mockPackage('a', { otherProps: { deps: { custom: { c: '0.0.1' } } } }),
              },
              {
                path: 'workspaces/b/packages/b/package.json',
                before: mockPackage('b', { otherProps: { deps: { custom: { c: '3.0.0' } } } }),
                after: mockPackage('b', { otherProps: { deps: { custom: { c: '0.0.1' } } } }),
              },
              {
                path: 'workspaces/b/packages/c/package.json',
                before: mockPackage('c', { otherProps: { name: 'c', version: '0.0.1' } }),
                after: mockPackage('c', { otherProps: { name: 'c', version: '0.0.1' } }),
              },
            ],
            {
              cli: {},
              rcFile: {
                customTypes: {
                  engines: {
                    strategy: 'version',
                    path: 'deps.custom.c',
                  },
                },
                dependencyTypes: ['dev', 'engines', 'prod', 'workspace'],
                source: [
                  'package.json',
                  'workspaces/*/package.json',
                  'workspaces/*/packages/*/package.json',
                ],
              },
            },
          ),
        () =>
          createScenario(
            [
              {
                path: 'workspaces/a/packages/a/package.json',
                before: mockPackage('a', { deps: ['c@0.1.0'] }),
                after: mockPackage('a', { deps: ['c@0.0.1'] }),
              },
              {
                path: 'workspaces/b/packages/b/package.json',
                before: mockPackage('b', { devDeps: ['c@0.2.0'] }),
                after: mockPackage('b', { devDeps: ['c@0.0.1'] }),
              },
              {
                path: 'workspaces/b/packages/c/package.json',
                before: mockPackage('c', { otherProps: { name: 'c', version: '0.0.1' } }),
                after: mockPackage('c', { otherProps: { name: 'c', version: '0.0.1' } }),
              },
            ],
            {
              cli: {},
              rcFile: {
                dependencyTypes: ['dev', 'prod', 'workspace'],
                source: [
                  'package.json',
                  'workspaces/*/package.json',
                  'workspaces/*/packages/*/package.json',
                ],
              },
            },
          ),
      ].forEach((getScenario) => {
        describe('versionGroup.inspect()', () => {
          test('should identify as a mismatch against the canonical local package version', () => {
            const scenario = getScenario();
            expect(scenario.report.versionGroups).toEqual([
              [toBeWorkspaceMismatch({ expectedVersion: '0.0.1', name: 'c' })],
            ]);
          });
        });

        describe('fix-mismatches', () => {
          test('should fix the mismatch', () => {
            const scenario = getScenario();
            Effect.runSync(fixMismatches({}, scenario.env));
            expect(scenario.env.exitProcess).not.toHaveBeenCalled();
            expect(scenario.env.writeFileSync.mock.calls).toEqual([
              scenario.files['workspaces/a/packages/a/package.json'].diskWriteWhenChanged,
              scenario.files['workspaces/b/packages/b/package.json'].diskWriteWhenChanged,
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
});
