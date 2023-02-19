import { normalize } from 'path';
import { fixMismatchesCli } from '../../src/bin-fix-mismatches/fix-mismatches-cli';
import { listMismatchesCli } from '../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../src/bin-list/list-cli';
import { ICON } from '../../src/constants';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * Variation of `dependentDoesNotMatchWorkspaceVersion` in a nested workspace.
 *
 * - C is developed in this monorepo, its version is `0.0.1`
 * - C's version is the single source of truth and should never be changed
 * - A and B depend on C incorrectly and should be fixed
 * - A, B, and C are in nested workspaces
 *
 * @see https://github.com/goldstack/goldstack/pull/170/files#diff-7ae45ad102eab3b6d7e7896acd08c427a9b25b346470d7bc6507b6481575d519R19
 * @see https://github.com/JamieMason/syncpack/pull/74
 * @see https://github.com/JamieMason/syncpack/issues/66
 */
describe('Dependent does not match nested workspace version', () => {
  function getScenario() {
    return createScenario(
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
          before: mockPackage('c', {
            otherProps: { name: 'c', version: '0.0.1' },
          }),
          after: mockPackage('c', {
            otherProps: { name: 'c', version: '0.0.1' },
          }),
        },
      ],
      {
        types: 'dev,prod,workspace',
        source: [
          'package.json',
          'workspaces/*/package.json',
          'workspaces/*/packages/*/package.json',
        ],
      },
    );
  }

  describe('fix-mismatches', () => {
    it('warns about the workspace version', () => {
      const scenario = getScenario();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['workspaces/a/packages/a/package.json']
          .diskWriteWhenChanged,
        scenario.files['workspaces/b/packages/b/package.json']
          .diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['workspaces/a/packages/a/package.json']
          .logEntryWhenChanged,
        scenario.files['workspaces/b/packages/b/package.json']
          .logEntryWhenChanged,
        scenario.files['workspaces/b/packages/c/package.json']
          .logEntryWhenUnchanged,
      ]);
    });
  });

  describe('format', () => {
    //
  });

  describe('lint-semver-ranges', () => {
    //
  });

  describe('list-mismatches', () => {
    it('warns about the workspace version', () => {
      const scenario = getScenario();
      const bc = 'workspaces/b/packages/c/package.json';
      const aa = 'workspaces/a/packages/a/package.json';
      const bb = 'workspaces/b/packages/b/package.json';
      listMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual(
        [
          [
            `${ICON.cross} c 0.0.1 is developed in this repo at ${normalize(
              bc,
            )}`,
          ],
          [`  0.1.0 in dependencies of ${normalize(aa)}`],
          [`  0.2.0 in devDependencies of ${normalize(bb)}`],
        ].map(([msg]) => [normalize(msg)]),
      );
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });
  });

  describe('list', () => {
    it('warns about the workspace version', () => {
      const scenario = getScenario();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([['✘ c 0.0.1, 0.1.0, 0.2.0']]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });
  });

  describe('set-semver-ranges', () => {
    //
  });
});
