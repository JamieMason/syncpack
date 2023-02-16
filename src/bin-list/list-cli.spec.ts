import 'expect-more-jest';
import { dependencyIsBanned } from '../../test/scenarios/dependency-is-banned';
import { dependencyIsPinned } from '../../test/scenarios/dependency-is-pinned';
import { dependentDoesNotMatchNestedWorkspaceVersion } from '../../test/scenarios/dependent-does-not-match-nested-workspace-version';
import { dependentDoesNotMatchNpmOverrideVersion } from '../../test/scenarios/dependent-does-not-match-npm-override-version';
import { dependentDoesNotMatchPnpmOverrideVersion } from '../../test/scenarios/dependent-does-not-match-pnpm-override-version';
import { dependentDoesNotMatchWorkspaceVersion } from '../../test/scenarios/dependent-does-not-match-workspace-version';
import type { TestScenario } from '../../test/scenarios/lib/create-scenario';
import { matchingUnsupportedVersions } from '../../test/scenarios/matching-unsupported-versions';
import { mismatchingUnsupportedVersions } from '../../test/scenarios/mismatching-unsupported-versions';
import { useHighestVersion } from '../../test/scenarios/use-highest-version';
import { versionIsIgnored } from '../../test/scenarios/version-is-ignored';
import { listCli } from './list-cli';

import { mismatchIsFilteredOut } from '../../test/scenarios/mismatch-is-filtered-out';

describe('list', () => {
  beforeEach(() => {
    jest.restoreAllMocks();
  });

  describe('when dependencies are installed with different versions', () => {
    describe('when the dependency is a package maintained in this workspace', () => {
      const variants: [string, () => TestScenario][] = [
        [
          'when using a typical workspace',
          dependentDoesNotMatchWorkspaceVersion,
        ],
        [
          'when using nested workspaces',
          dependentDoesNotMatchNestedWorkspaceVersion,
        ],
      ];
      variants.forEach(([context, getScenario]) => {
        describe(`${context}`, () => {
          it('warns about the workspace version', () => {
            const scenario = getScenario();
            listCli(scenario.config, scenario.disk);
            expect(scenario.log.mock.calls).toEqual([
              ['✘ c 0.0.1, 0.1.0, 0.2.0'],
            ]);
            expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
          });
        });
      });
    });

    it('lists mismatched versions which syncpack cannot fix', () => {
      const scenario = mismatchingUnsupportedVersions();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [
          '✘ foo has mismatched versions which syncpack cannot fix: 0.2.0, 0.3.0, link:vendor/foo-0.1.0, workspace:*',
        ],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('lists matching versions which syncpack cannot fix anyway', () => {
      const scenario = matchingUnsupportedVersions();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([['- foo workspace:*']]);
      expect(scenario.disk.process.exit).not.toHaveBeenCalled();
    });

    it('lists mismatching npm overrides', () => {
      const scenario = dependentDoesNotMatchNpmOverrideVersion();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([['✘ c 0.1.0, 0.2.0']]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('lists mismatching pnpm overrides', () => {
      const scenario = dependentDoesNotMatchPnpmOverrideVersion();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([['✘ c 0.1.0, 0.2.0']]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('removes banned/disallowed dependencies', () => {
      const scenario = dependencyIsBanned();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        ['- foo 0.1.0'],
        [expect.stringMatching(/Version Group 1/)],
        ['✘ bar is banned in this version group'],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('mentions ignored dependencies', () => {
      const scenario = versionIsIgnored();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        ['- foo 0.1.0'],
        [expect.stringMatching(/Version Group 1/)],
        ['- bar is ignored in this version group'],
      ]);
      expect(scenario.disk.process.exit).not.toHaveBeenCalled();
    });

    it('lists mismatching pinned versions', () => {
      const scenario = dependencyIsPinned();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Version Group 1/)],
        ['✘ bar is pinned to 2.2.2 in this version group'],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('uses the highest installed version', () => {
      const scenario = useHighestVersion();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([['✘ bar 0.1.0, 0.2.0, 0.3.0']]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });

    it('ignores mismatches which do not match applied filter', () => {
      const scenario = mismatchIsFilteredOut();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([['- d 1.1.1']]);
      expect(scenario.disk.process.exit).not.toHaveBeenCalled();
    });
  });
});
