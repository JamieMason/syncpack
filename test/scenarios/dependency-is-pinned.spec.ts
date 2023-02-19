import { normalize } from 'path';
import { fixMismatchesCli } from '../../src/bin-fix-mismatches/fix-mismatches-cli';
import { listMismatchesCli } from '../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../src/bin-list/list-cli';
import { ICON } from '../../src/constants';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/** "bar" should always be 2.2.2 but is not */
describe('Dependency is pinned', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', { deps: ['bar@0.2.0'] }),
          after: mockPackage('a', { deps: ['bar@2.2.2'] }),
        },
      ],
      {
        versionGroups: [
          {
            dependencies: ['bar'],
            dependencyTypes: [],
            packages: ['**'],
            pinVersion: '2.2.2',
          },
        ],
      },
    );
  }

  describe('fix-mismatches', () => {
    it('synchronises pinned versions', () => {
      const scenario = getScenario();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenChanged,
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
    it('synchronises pinned versions', () => {
      const scenario = getScenario();
      const a = 'packages/a/package.json';
      listMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Version Group 1/)],
        [`${ICON.cross} bar is pinned in this version group at 2.2.2`],
        [`  0.2.0 in dependencies of ${normalize(a)}`],
      ]);
    });
  });

  describe('list', () => {
    it('lists mismatching pinned versions', () => {
      const scenario = getScenario();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Version Group 1/)],
        ['✘ bar is pinned to 2.2.2 in this version group'],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });
  });

  describe('set-semver-ranges', () => {
    //
  });
});
