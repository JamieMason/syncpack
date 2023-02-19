import { normalize } from 'path';
import { fixMismatchesCli } from '../../src/bin-fix-mismatches/fix-mismatches-cli';
import { listMismatchesCli } from '../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../src/bin-list/list-cli';
import { ICON } from '../../src/constants';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - A does not depend on `bar`
 * - B does depend on `bar`
 * - `bar` is banned in every package from being installed
 * - `bar` should be removed from B
 */
describe('Dependency is banned', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', { deps: ['foo@0.1.0'] }),
          after: mockPackage('a', { deps: ['foo@0.1.0'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', { deps: ['bar@0.2.0'] }),
          after: mockPackage('b'),
        },
      ],
      {
        versionGroups: [
          {
            dependencies: ['bar'],
            dependencyTypes: [],
            packages: ['**'],
            isBanned: true,
          },
        ],
      },
    );
  }

  describe('fix-mismatches', () => {
    it('removes banned/disallowed dependencies', () => {
      const scenario = getScenario();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/b/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/b/package.json'].logEntryWhenChanged,
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
    it('removes banned/disallowed dependencies', () => {
      const scenario = getScenario();
      const b = 'packages/b/package.json';
      listMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Version Group 1/)],
        [`${ICON.cross} bar is banned in this version group`],
        [`  0.2.0 in dependencies of ${normalize(b)}`],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });
  });

  describe('list', () => {
    it('removes banned/disallowed dependencies', () => {
      const scenario = getScenario();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Version Group 1/)],
        ['✘ bar is banned in this version group'],
        [expect.stringMatching(/Default Version Group/)],
        ['- foo 0.1.0'],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });
  });

  describe('set-semver-ranges', () => {
    //
  });
});
