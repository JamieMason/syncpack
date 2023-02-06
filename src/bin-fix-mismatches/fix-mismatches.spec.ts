import 'expect-more-jest';
import { scenarios } from '../../test/scenarios';
import { fixMismatchesCli } from './fix-mismatches-cli';

describe('fixMismatches', () => {
  beforeEach(() => {
    jest.restoreAllMocks();
  });

  describe('when dependencies are installed with different versions', () => {
    describe('when the dependency is a package maintained in this workspace', () => {
      describe('when using a typical workspace', () => {
        it('warns about the workspace version', () => {
          const scenario = scenarios.dependentDoesNotMatchWorkspaceVersion();
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

      describe('when using nested workspaces', () => {
        it('warns about the workspace version', () => {
          const scenario =
            scenarios.dependentDoesNotMatchNestedWorkspaceVersion();
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
    });

    it('replaces non-semver dependencies with valid semver dependencies', () => {
      const scenario = scenarios.mismatchesIncludeNonSemverVersions();
      fixMismatchesCli(scenario.config, scenario.disk);
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
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/b/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Version Group 1/)],
        scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/b/package.json'].logEntryWhenChanged,
      ]);
    });

    it('does not consider versions of ignored dependencies', () => {
      const scenario = scenarios.versionIsIgnored();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync).not.toHaveBeenCalled();
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Version Group 1/)],
        scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
      ]);
    });

    it('replaces mismatching npm overrides', () => {
      const scenario = scenarios.dependentDoesNotMatchNpmOverrideVersion();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenChanged,
        scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
      ]);
    });

    it('replaces mismatching pnpm overrides', () => {
      const scenario = scenarios.dependentDoesNotMatchPnpmOverrideVersion();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenChanged,
        scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
      ]);
    });

    it('synchronises pinned versions', () => {
      const scenario = scenarios.dependencyIsPinned();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Version Group 1/)],
        scenario.files['packages/a/package.json'].logEntryWhenChanged,
      ]);
    });

    it('uses the highest installed version', () => {
      const scenario = scenarios.useHighestVersion();
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

    it('fix version in dependenciesCustomPath using the highest installed version', () => {
      const scenario = scenarios.customDepPath();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/b/package.json'].diskWriteWhenChanged,
        scenario.files['packages/c/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/b/package.json'].logEntryWhenChanged,
        scenario.files['packages/c/package.json'].logEntryWhenChanged,
      ]);
    });
  });
});
