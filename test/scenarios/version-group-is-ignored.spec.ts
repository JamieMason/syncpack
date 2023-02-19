import 'expect-more-jest';
import { fixMismatchesCli } from '../../src/bin-fix-mismatches/fix-mismatches-cli';
import { listMismatchesCli } from '../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../src/bin-list/list-cli';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - A does not depend on `bar`
 * - B does depend on `bar`
 * - `bar` is ignored by syncpack in every package
 * - `bar` is unprotected so can mismatch etc
 */
describe('versionGroup.isIgnored', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', { deps: ['foo@0.1.0', 'bar@1.1.1'] }),
          after: mockPackage('a', { deps: ['foo@0.1.0', 'bar@1.1.1'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', { deps: ['bar@0.2.0'] }),
          after: mockPackage('b', { deps: ['bar@0.2.0'] }),
        },
      ],
      {
        versionGroups: [
          {
            dependencies: ['bar'],
            dependencyTypes: [],
            packages: ['**'],
            isIgnored: true,
          },
        ],
      },
    );
  }

  describe('fix-mismatches', () => {
    it('does not consider versions of ignored dependencies', () => {
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
    it('does not mention ignored dependencies', () => {
      const scenario = getScenario();
      listMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toBeEmptyArray();
      expect(scenario.disk.process.exit).not.toHaveBeenCalled();
    });
  });

  describe('list', () => {
    it('mentions ignored dependencies', () => {
      const scenario = getScenario();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Version Group 1/)],
        ['- bar is ignored in this version group'],
        [expect.stringMatching(/Default Version Group/)],
        ['- foo 0.1.0'],
      ]);
      expect(scenario.disk.process.exit).not.toHaveBeenCalled();
    });
  });

  describe('set-semver-ranges', () => {
    //
  });
});
