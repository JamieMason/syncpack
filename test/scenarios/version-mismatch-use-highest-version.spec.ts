import { normalize } from 'path';
import { fixMismatchesCli } from '../../src/bin-fix-mismatches/fix-mismatches-cli';
import { listMismatchesCli } from '../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../src/bin-list/list-cli';
import { ICON } from '../../src/constants';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/** "bar" should be 0.3.0, which is the highest installed version */
describe('version mismatch: Use highest version', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', { deps: ['bar@0.2.0'] }),
          after: mockPackage('a', { deps: ['bar@0.3.0'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', { deps: ['bar@0.3.0'] }),
          after: mockPackage('b', { deps: ['bar@0.3.0'] }),
        },
        {
          path: 'packages/c/package.json',
          before: mockPackage('c', { deps: ['bar@0.1.0'] }),
          after: mockPackage('c', { deps: ['bar@0.3.0'] }),
        },
      ],
      {},
    );
  }

  describe('fix-mismatches', () => {
    it('uses the highest installed version', () => {
      const scenario = getScenario();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].diskWriteWhenChanged,
        scenario.files['packages/c/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenChanged,
        scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/c/package.json'].logEntryWhenChanged,
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
    it('uses the highest installed version', () => {
      const scenario = getScenario();
      const a = 'packages/a/package.json';
      const c = 'packages/c/package.json';
      listMismatchesCli(scenario.config, scenario.disk);

      expect(scenario.log.mock.calls).toEqual([
        [`${ICON.cross} bar 0.3.0 is the highest valid semver version in use`],
        [`  0.2.0 in dependencies of ${normalize(a)}`],
        [`  0.1.0 in dependencies of ${normalize(c)}`],
      ]);
    });
  });

  describe('list', () => {
    it('uses the highest installed version', () => {
      const scenario = getScenario();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([['✘ bar 0.1.0, 0.2.0, 0.3.0']]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });
  });

  describe('set-semver-ranges', () => {
    //
  });
});
