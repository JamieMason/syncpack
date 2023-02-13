import 'expect-more-jest';
import { normalize } from 'path';
import { dependencyIsBanned } from '../../test/scenarios/dependency-is-banned';
import { dependencyIsPinned } from '../../test/scenarios/dependency-is-pinned';
import { dependentDoesNotMatchNestedWorkspaceVersion } from '../../test/scenarios/dependent-does-not-match-nested-workspace-version';
import { dependentDoesNotMatchWorkspaceVersion } from '../../test/scenarios/dependent-does-not-match-workspace-version';
import { mismatchesIncludeNonSemverVersions } from '../../test/scenarios/mismatches-include-non-semver-versions';
import { useHighestVersion } from '../../test/scenarios/use-highest-version';
import { versionIsIgnored } from '../../test/scenarios/version-is-ignored';
import { ICON } from '../constants';
import { listMismatchesCli } from './list-mismatches-cli';

describe('listMismatches', () => {
  beforeEach(() => {
    jest.restoreAllMocks();
  });

  describe('when dependencies are installed with different versions', () => {
    describe('when the dependency is a package maintained in this workspace', () => {
      describe('when using a typical workspace', () => {
        it('warns about the workspace version', () => {
          const scenario = dependentDoesNotMatchWorkspaceVersion();
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

      describe('when using nested workspaces', () => {
        it('warns about the workspace version', () => {
          const scenario = dependentDoesNotMatchNestedWorkspaceVersion();
          const bc = 'workspaces/b/packages/c/package.json';
          const aa = 'workspaces/a/packages/a/package.json';
          const bb = 'workspaces/b/packages/b/package.json';
          listMismatchesCli(scenario.config, scenario.disk);
          expect(scenario.log.mock.calls).toEqual(
            [
              [
                `${ICON.cross} c 0.0.1 is developed in this repo at ${normalize(
                  bc,
                )}`,
              ],
              [`  0.1.0 in dependencies of ${normalize(aa)}`],
              [`  0.2.0 in devDependencies of ${normalize(bb)}`],
            ].map(([msg]) => [normalize(msg)]),
          );
          expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
        });
      });
    });

    it('replaces non-semver dependencies with valid semver dependencies', () => {
      const scenario = mismatchesIncludeNonSemverVersions();
      const a = 'packages/a/package.json';
      const b = 'packages/b/package.json';
      const c = 'packages/c/package.json';
      const d = 'packages/d/package.json';
      listMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [`${ICON.cross} foo 0.3.0 is the highest valid semver version in use`],
        [`  link:vendor/foo-0.1.0 in dependencies of ${normalize(a)}`],
        [`  link:vendor/foo-0.2.0 in dependencies of ${normalize(b)}`],
        [`  0.2.0 in dependencies of ${normalize(d)}`],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('removes banned/disallowed dependencies', () => {
      const scenario = dependencyIsBanned();
      const b = 'packages/b/package.json';
      listMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Version Group 1/)],
        [`${ICON.cross} bar is banned in this version group`],
        [`  0.2.0 in dependencies of ${normalize(b)}`],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('does not mention ignored dependencies', () => {
      const scenario = versionIsIgnored();
      listMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toBeEmptyArray();
      expect(scenario.disk.process.exit).not.toHaveBeenCalled();
    });

    it('synchronises pinned versions', () => {
      const scenario = dependencyIsPinned();
      const a = 'packages/a/package.json';
      listMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Version Group 1/)],
        [`${ICON.cross} bar is pinned in this version group at 2.2.2`],
        [`  0.2.0 in dependencies of ${normalize(a)}`],
      ]);
    });

    it('uses the highest installed version', () => {
      const scenario = useHighestVersion();
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
});
