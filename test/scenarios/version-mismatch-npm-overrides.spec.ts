import { fixMismatchesCli } from '../../src/bin-fix-mismatches/fix-mismatches-cli';
import { listCli } from '../../src/bin-list/list-cli';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - A has an npm override of C
 * - B has an npm override of C
 * - The versions do not match
 * - The highest semver version wins
 */
describe('version mismatch: npm overrides', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', { overrides: ['c@0.1.0'] }),
          after: mockPackage('a', { overrides: ['c@0.2.0'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', { overrides: ['c@0.2.0'] }),
          after: mockPackage('b', { overrides: ['c@0.2.0'] }),
        },
      ],
      {
        types: 'overrides',
      },
    );
  }

  describe('fix-mismatches', () => {
    it('replaces mismatching npm overrides', () => {
      const scenario = getScenario();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenChanged,
        scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
      ]);
    });
  });

  describe('list', () => {
    it('lists mismatching npm overrides', () => {
      const scenario = getScenario();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([['✘ c 0.1.0, 0.2.0']]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });
  });
});
