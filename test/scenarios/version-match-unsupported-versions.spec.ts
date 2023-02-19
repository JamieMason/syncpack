import { fixMismatchesCli } from '../../src/bin-fix-mismatches/fix-mismatches-cli';
import { listMismatchesCli } from '../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../src/bin-list/list-cli';
import { setSemverRangesCli } from '../../src/bin-set-semver-ranges/set-semver-ranges-cli';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - A and B have versions syncpack does not support
 * - The versions match
 * - All packages should be left unchanged
 */
describe('version match: unsupported versions', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', { deps: ['foo@workspace:*'] }),
          after: mockPackage('a', { deps: ['foo@workspace:*'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', { deps: ['foo@workspace:*'] }),
          after: mockPackage('b', { deps: ['foo@workspace:*'] }),
        },
      ],
      {},
    );
  }

  describe('fix-mismatches', () => {
    it('skips matching versions which syncpack cannot fix anyway', () => {
      const scenario = getScenario();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync).not.toHaveBeenCalled();
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
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
    it('does not list matching versions which syncpack cannot fix anyway', () => {
      const scenario = getScenario();
      listMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.log).not.toHaveBeenCalled();
      expect(scenario.disk.process.exit).not.toHaveBeenCalled();
    });
  });

  describe('list', () => {
    it('lists matching versions which syncpack cannot fix anyway', () => {
      const scenario = getScenario();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([['- foo workspace:*']]);
      expect(scenario.disk.process.exit).not.toHaveBeenCalled();
    });
  });

  describe('set-semver-ranges', () => {
    it('leaves non-semver versions unchanged', () => {
      const scenario = getScenario();
      setSemverRangesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
      ]);
      expect(scenario.disk.process.exit).not.toHaveBeenCalled();
    });
  });
});
