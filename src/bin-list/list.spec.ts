import 'expect-more-jest';
import { scenarios } from '../../test/scenarios';
import type { TestScenario } from '../../test/scenarios/create-scenario';
import { getInput } from '../lib/get-input';
import { list } from './list';

describe('list', () => {
  beforeEach(() => {
    jest.restoreAllMocks();
  });

  describe('when dependencies are installed with different versions', () => {
    describe('when the dependency is a package maintained in this workspace', () => {
      const variants: [string, () => TestScenario][] = [
        [
          'when using a typical workspace',
          scenarios.dependentDoesNotMatchWorkspaceVersion,
        ],
        [
          'when using nested workspaces',
          scenarios.dependentDoesNotMatchNestedWorkspaceVersion,
        ],
      ];
      variants.forEach(([context, getScenario]) => {
        describe(context, () => {
          it('warns about the workspace version', () => {
            const scenario = getScenario();
            list(getInput(scenario.disk, scenario.config), scenario.disk);
            expect(scenario.log.mock.calls).toEqual([
              ['✘ c 0.0.1, 0.1.0, 0.2.0'],
            ]);
            expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
          });
        });
      });
    });

    it('lists non-semver dependencies with valid semver dependencies', () => {
      const scenario = scenarios.mismatchesIncludeNonSemverVersions();
      list(getInput(scenario.disk, scenario.config), scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        ['✘ foo 0.2.0, 0.3.0, link:vendor/foo-0.1.0, link:vendor/foo-0.2.0'],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('lists mismatching npm overrides', () => {
      const scenario = scenarios.dependentDoesNotMatchNpmOverrideVersion();
      list(getInput(scenario.disk, scenario.config), scenario.disk);
      expect(scenario.log.mock.calls).toEqual([['✘ c 0.1.0, 0.2.0']]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('lists mismatching pnpm overrides', () => {
      const scenario = scenarios.dependentDoesNotMatchPnpmOverrideVersion();
      list(getInput(scenario.disk, scenario.config), scenario.disk);
      expect(scenario.log.mock.calls).toEqual([['✘ c 0.1.0, 0.2.0']]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('removes banned/disallowed dependencies', () => {
      const scenario = scenarios.dependencyIsBanned();
      list(getInput(scenario.disk, scenario.config), scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        ['- foo 0.1.0'],
        [expect.stringMatching(/Version Group 1/)],
        ['✘ bar is defined in this version group as banned from use'],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });
  });
});
