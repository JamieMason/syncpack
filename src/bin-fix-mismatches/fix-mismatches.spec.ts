import 'expect-more-jest';
import { scenarios } from '../../test/scenarios';
import { getInput } from '../lib/get-input';
import { fixMismatches } from './fix-mismatches';

describe('fixMismatches', () => {
  beforeEach(() => {
    jest.restoreAllMocks();
  });

  describe('when dependencies are installed with different versions', () => {
    describe('when the dependency is a package maintained in this workspace', () => {
      it('uses the workspace version', () => {
        const scenario = scenarios.dependentDoesNotMatchWorkspaceVersion();
        fixMismatches(getInput(scenario.disk, scenario.config), scenario.disk);
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

    it('replaces non-semver dependencies with valid semver dependencies', () => {
      const scenario = scenarios.mismatchesIncludeNonSemverVersions();
      fixMismatches(getInput(scenario.disk, scenario.config), scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].diskWriteWhenChanged,
        scenario.files['packages/b/package.json'].diskWriteWhenChanged,
        scenario.files['packages/d/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenChanged,
        scenario.files['packages/b/package.json'].logEntryWhenChanged,
        scenario.files['packages/c/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/d/package.json'].logEntryWhenChanged,
      ]);
    });

    it('removes banned/disallowed dependencies', () => {
      const scenario = scenarios.dependencyIsBanned();
      fixMismatches(getInput(scenario.disk, scenario.config), scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/b/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Version Group 1/)],
        scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/b/package.json'].logEntryWhenChanged,
      ]);
    });
  });
});
