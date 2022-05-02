import 'expect-more-jest';
import { scenarios } from '../../test/scenarios';
import { getInput } from '../lib/get-input';
import { list } from './list';

describe('list', () => {
  beforeEach(() => {
    jest.restoreAllMocks();
  });

  describe('when dependencies are installed with different versions', () => {
    describe('when the dependency is a package maintained in this workspace', () => {
      it('warns ab the workspace version', () => {
        const scenario = scenarios.dependentDoesNotMatchWorkspaceVersion();
        list(getInput(scenario.disk, scenario.config), scenario.disk);
        expect(scenario.log.mock.calls).toEqual([['✕ c 0.1.0, 0.2.0']]);
        expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
      });
    });

    it('replaces non-semver dependencies with valid semver dependencies', () => {
      const scenario = scenarios.mismatchesIncludeNonSemverVersions();
      list(getInput(scenario.disk, scenario.config), scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        ['✕ foo 0.2.0, 0.3.0, link:vendor/foo-0.1.0, link:vendor/foo-0.2.0'],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('removes banned/disallowed dependencies', () => {
      const scenario = scenarios.dependencyIsBanned();
      list(getInput(scenario.disk, scenario.config), scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        ['- foo 0.1.0'],
        [expect.stringMatching(/Version Group 1/)],
        ['✕ bar remove this dependency'],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });
  });
});
