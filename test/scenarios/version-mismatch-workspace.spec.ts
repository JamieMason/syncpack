import { normalize } from 'path';
import { fixMismatchesCli } from '../../src/bin-fix-mismatches/fix-mismatches-cli';
import { listMismatchesCli } from '../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../src/bin-list/list-cli';
import { ICON } from '../../src/constants';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - C is developed in this monorepo, its version is `0.0.1`
 * - C's version is the single source of truth and should never be changed
 * - A depends on C incorrectly and should be fixed
 * - B depends on C incorrectly and should be fixed
 */
describe('version mismatch: workspace', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', { deps: ['c@0.1.0'] }),
          after: mockPackage('a', { deps: ['c@0.0.1'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', { devDeps: ['c@0.2.0'] }),
          after: mockPackage('b', { devDeps: ['c@0.0.1'] }),
        },
        {
          path: 'packages/c/package.json',
          before: mockPackage('c', {
            otherProps: { name: 'c', version: '0.0.1' },
          }),
          after: mockPackage('c', {
            otherProps: { name: 'c', version: '0.0.1' },
          }),
        },
      ],
      {},
    );
  }

  describe('fix-mismatches', () => {
    it('warns about the workspace version', () => {
      const scenario = getScenario();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].diskWriteWhenChanged,
        scenario.files['packages/b/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenChanged,
        scenario.files['packages/b/package.json'].logEntryWhenChanged,
        scenario.files['packages/c/package.json'].logEntryWhenUnchanged,
      ]);
    });
  });

  describe('list-mismatches', () => {
    it('warns about the workspace version', () => {
      const scenario = getScenario();
      const a = 'packages/a/package.json';
      const b = 'packages/b/package.json';
      const c = 'packages/c/package.json';
      listMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual(
        [
          [
            `${ICON.cross} c 0.0.1 is developed in this repo at ${normalize(
              c,
            )}`,
          ],
          [`  0.1.0 in dependencies of ${normalize(a)}`],
          [`  0.2.0 in devDependencies of ${normalize(b)}`],
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
});
