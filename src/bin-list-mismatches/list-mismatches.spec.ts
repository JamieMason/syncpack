import 'expect-more-jest';
import { normalize } from 'path';
import { scenarios } from '../../test/scenarios';
import { getInput } from '../lib/get-input';
import { listMismatches } from './list-mismatches';

describe('listMismatches', () => {
  beforeEach(() => {
    jest.restoreAllMocks();
  });

  describe('when dependencies are installed with different versions', () => {
    describe('when the dependency is a package maintained in this workspace', () => {
      describe('when using a typical workspace', () => {
        it('warns about the workspace version', () => {
          const scenario = scenarios.dependentDoesNotMatchWorkspaceVersion();
          const a = 'packages/a/package.json';
          const b = 'packages/b/package.json';
          const c = 'packages/c/package.json';
          listMismatches(
            getInput(scenario.disk, scenario.config),
            scenario.disk,
          );
          expect(scenario.log.mock.calls).toEqual(
            [
              [`- c: 0.0.1 is developed in this repo at ${normalize(c)}`],
              [`  0.1.0 in dependencies of ${normalize(a)}`],
              [`  0.2.0 in devDependencies of ${normalize(b)}`],
              [`  0.0.1 in version of ${normalize(c)}`],
            ].map(([msg]) => [normalize(msg)]),
          );
          expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('when using nested workspaces', () => {
        it('warns about the workspace version', () => {
          const scenario =
            scenarios.dependentDoesNotMatchNestedWorkspaceVersion();
          const bc = 'workspaces/b/packages/c/package.json';
          const aa = 'workspaces/a/packages/a/package.json';
          const bb = 'workspaces/b/packages/b/package.json';
          listMismatches(
            getInput(scenario.disk, scenario.config),
            scenario.disk,
          );
          expect(scenario.log.mock.calls).toEqual(
            [
              [`- c: 0.0.1 is developed in this repo at ${normalize(bc)}`],
              [`  0.1.0 in dependencies of ${normalize(aa)}`],
              [`  0.2.0 in devDependencies of ${normalize(bb)}`],
              [`  0.0.1 in version of ${normalize(bc)}`],
            ].map(([msg]) => [normalize(msg)]),
          );
          expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
        });
      });
    });

    it('replaces non-semver dependencies with valid semver dependencies', () => {
      const scenario = scenarios.mismatchesIncludeNonSemverVersions();
      const a = 'packages/a/package.json';
      const b = 'packages/b/package.json';
      const c = 'packages/c/package.json';
      const d = 'packages/d/package.json';
      listMismatches(getInput(scenario.disk, scenario.config), scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        ['- foo: 0.3.0 is the highest valid semver version in use'],
        [`  link:vendor/foo-0.1.0 in dependencies of ${normalize(a)}`],
        [`  link:vendor/foo-0.2.0 in dependencies of ${normalize(b)}`],
        [`  0.3.0 in dependencies of ${normalize(c)}`],
        [`  0.2.0 in dependencies of ${normalize(d)}`],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('removes banned/disallowed dependencies', () => {
      const scenario = scenarios.dependencyIsBanned();
      const b = 'packages/b/package.json';
      listMismatches(getInput(scenario.disk, scenario.config), scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Version Group 1/)],
        ['✘ bar is defined in this version group as banned from use'],
        [`  0.2.0 in dependencies of ${normalize(b)}`],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('does not mention ignored dependencies', () => {
      const scenario = scenarios.versionIsIgnored();
      listMismatches(getInput(scenario.disk, scenario.config), scenario.disk);
      expect(scenario.log.mock.calls).toBeEmptyArray();
      expect(scenario.disk.process.exit).not.toHaveBeenCalled();
    });
  });
});
