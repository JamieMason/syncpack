import 'expect-more-jest';
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
          listMismatches(
            getInput(scenario.disk, scenario.config),
            scenario.disk,
          );
          expect(scenario.log.mock.calls).toEqual([
            ['- c 0.0.1 is developed in this repo at packages/c/package.json'],
            ['  0.1.0 in dependencies of packages/a/package.json'],
            ['  0.2.0 in devDependencies of packages/b/package.json'],
            ['  0.0.1 in version of packages/c/package.json'],
          ]);
          expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
        });
      });

      describe('when using nested workspaces', () => {
        it('warns about the workspace version', () => {
          const scenario =
            scenarios.dependentDoesNotMatchNestedWorkspaceVersion();
          listMismatches(
            getInput(scenario.disk, scenario.config),
            scenario.disk,
          );
          expect(scenario.log.mock.calls).toEqual([
            [
              '- c 0.0.1 is developed in this repo at workspaces/b/packages/c/package.json',
            ],
            ['  0.1.0 in dependencies of workspaces/a/packages/a/package.json'],
            [
              '  0.2.0 in devDependencies of workspaces/b/packages/b/package.json',
            ],
            ['  0.0.1 in version of workspaces/b/packages/c/package.json'],
          ]);
          expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
        });
      });
    });

    it('replaces non-semver dependencies with valid semver dependencies', () => {
      const scenario = scenarios.mismatchesIncludeNonSemverVersions();
      listMismatches(getInput(scenario.disk, scenario.config), scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        ['- foo 0.3.0 is the highest valid semver version in use'],
        ['  link:vendor/foo-0.1.0 in dependencies of packages/a/package.json'],
        ['  link:vendor/foo-0.2.0 in dependencies of packages/b/package.json'],
        ['  0.3.0 in dependencies of packages/c/package.json'],
        ['  0.2.0 in dependencies of packages/d/package.json'],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('removes banned/disallowed dependencies', () => {
      const scenario = scenarios.dependencyIsBanned();
      listMismatches(getInput(scenario.disk, scenario.config), scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Version Group 1/)],
        ['✘ bar is defined in this version group as banned from use'],
        ['  0.2.0 in dependencies of packages/b/package.json'],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });
  });
});
