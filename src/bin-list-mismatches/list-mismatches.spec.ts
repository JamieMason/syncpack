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
      it('warns ab the workspace version', () => {
        const scenario = scenarios.dependentDoesNotMatchWorkspaceVersion();
        listMismatches(getInput(scenario.disk, scenario.config), scenario.disk);
        expect(scenario.log.mock.calls).toEqual([
          ['- c 0.0.1'],
          ['  0.1.0 in dependencies of a'],
          ['  0.2.0 in devDependencies of b'],
        ]);
        expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
      });
    });

    it('replaces non-semver dependencies with valid semver dependencies', () => {
      const scenario = scenarios.mismatchesIncludeNonSemverVersions();
      listMismatches(getInput(scenario.disk, scenario.config), scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        ['- foo 0.3.0'],
        ['  link:vendor/foo-0.1.0 in dependencies of a'],
        ['  link:vendor/foo-0.2.0 in dependencies of b'],
        ['  0.3.0 in dependencies of c'],
        ['  0.2.0 in dependencies of d'],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('removes banned/disallowed dependencies', () => {
      const scenario = scenarios.dependencyIsBanned();
      listMismatches(getInput(scenario.disk, scenario.config), scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Version Group 1/)],
        ['✕ bar remove this dependency'],
        ['  0.2.0 in dependencies of b'],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });
  });
});
