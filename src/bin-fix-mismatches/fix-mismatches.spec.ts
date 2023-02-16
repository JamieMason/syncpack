import 'expect-more-jest';
import { customNameAndVersionMismatch } from '../../test/scenarios/custom-name-and-version-mismatch';
import { customVersionMismatch } from '../../test/scenarios/custom-version-mismatch';
import { customVersionsByNameMismatch } from '../../test/scenarios/custom-versions-by-name-mismatch';
import { dependencyIsBanned } from '../../test/scenarios/dependency-is-banned';
import { dependencyIsPinned } from '../../test/scenarios/dependency-is-pinned';
import { dependentDoesNotMatchNestedWorkspaceVersion } from '../../test/scenarios/dependent-does-not-match-nested-workspace-version';
import { dependentDoesNotMatchNpmOverrideVersion } from '../../test/scenarios/dependent-does-not-match-npm-override-version';
import { dependentDoesNotMatchPnpmOverrideVersion } from '../../test/scenarios/dependent-does-not-match-pnpm-override-version';
import { dependentDoesNotMatchWorkspaceVersion } from '../../test/scenarios/dependent-does-not-match-workspace-version';
import { matchingUnsupportedVersions } from '../../test/scenarios/matching-unsupported-versions';
import { mismatchingUnsupportedVersions } from '../../test/scenarios/mismatching-unsupported-versions';
import { unusedCustomType } from '../../test/scenarios/unused-custom-type';
import { useHighestVersion } from '../../test/scenarios/use-highest-version';
import { versionIsIgnored } from '../../test/scenarios/version-is-ignored';
import { fixMismatchesCli } from './fix-mismatches-cli';

describe('fixMismatches', () => {
  afterEach(() => {
    jest.resetAllMocks();
  });

  describe('when dependencies are installed with different versions', () => {
    describe('when the dependency is a package maintained in this workspace', () => {
      describe('when using a typical workspace', () => {
        it('warns about the workspace version', () => {
          const scenario = dependentDoesNotMatchWorkspaceVersion();
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
          const scenario = dependentDoesNotMatchNestedWorkspaceVersion();
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

      describe('when using custom types', () => {
        it('ignores the mismatch in the custom location if it has been filtered out', () => {
          const scenario = unusedCustomType();
          fixMismatchesCli(scenario.config, scenario.disk);
          expect(scenario.disk.writeFileSync).not.toHaveBeenCalled();
          expect(scenario.log.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
            scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
          ]);
        });

        it('fixes "versionsByName" mismatches in custom locations', () => {
          const scenario = customVersionsByNameMismatch();
          fixMismatchesCli(scenario.config, scenario.disk);
          expect(scenario.disk.writeFileSync.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].diskWriteWhenChanged,
          ]);
          expect(scenario.log.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].logEntryWhenChanged,
            scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
          ]);
        });

        it('fixes "name@version" mismatches in custom locations', () => {
          const scenario = customNameAndVersionMismatch();
          fixMismatchesCli(scenario.config, scenario.disk);
          expect(scenario.disk.writeFileSync.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].diskWriteWhenChanged,
          ]);
          expect(scenario.log.mock.calls).toEqual([
            scenario.files['packages/a/package.json'].logEntryWhenChanged,
            scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
          ]);
        });

        it('fixes "version" mismatches in custom locations', () => {
          const scenario = customVersionMismatch();
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
    });

    it('skips mismatched versions which syncpack cannot fix', () => {
      const scenario = mismatchingUnsupportedVersions();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync).not.toHaveBeenCalled();
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/c/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/d/package.json'].logEntryWhenUnchanged,
      ]);
    });

    it('skips matching versions which syncpack cannot fix anyway', () => {
      const scenario = matchingUnsupportedVersions();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync).not.toHaveBeenCalled();
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
      ]);
    });

    it('removes banned/disallowed dependencies', () => {
      const scenario = dependencyIsBanned();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/b/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/b/package.json'].logEntryWhenChanged,
      ]);
    });

    it('does not consider versions of ignored dependencies', () => {
      const scenario = versionIsIgnored();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync).not.toHaveBeenCalled();
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
      ]);
    });

    it('replaces mismatching npm overrides', () => {
      const scenario = dependentDoesNotMatchNpmOverrideVersion();
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
      const scenario = dependentDoesNotMatchPnpmOverrideVersion();
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
      const scenario = dependencyIsPinned();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenChanged,
      ]);
    });

    it('uses the highest installed version', () => {
      const scenario = useHighestVersion();
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
});
