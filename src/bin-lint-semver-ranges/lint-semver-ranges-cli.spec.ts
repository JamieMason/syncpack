import 'expect-more-jest';
import { scenarios } from '../../test/scenarios';
import { lintSemverRangesCli } from './lint-semver-ranges-cli';

describe('lintSemverRanges', () => {
  beforeEach(() => {
    jest.restoreAllMocks();
  });

  it('lists versions with ranges which do not match the project config', () => {
    const scenario = scenarios.semverRangesDoNotMatchConfig();
    lintSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.log.mock.calls).toEqual([
      ['✘ b'],
      ['  0.1.0 → ~0.1.0 in devDependencies of packages/a/package.json'],
      ['✘ c'],
      ['  0.1.0 → ~0.1.0 in overrides of packages/a/package.json'],
      ['✘ d'],
      ['  0.1.0 → ~0.1.0 in pnpmOverrides of packages/a/package.json'],
      ['✘ e'],
      ['  0.1.0 → ~0.1.0 in peerDependencies of packages/a/package.json'],
      ['✘ f'],
      ['  0.1.0 → ~0.1.0 in resolutions of packages/a/package.json'],
    ]);
  });

  it('ensures wildcard versions are supported', () => {
    const scenario = scenarios.semverRangesDoNotMatchConfigWildcard();
    lintSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.log.mock.calls).toEqual([
      ['✘ b'],
      ['  0.1.0 → * in devDependencies of packages/a/package.json'],
      ['✘ c'],
      ['  0.1.0 → * in overrides of packages/a/package.json'],
      ['✘ d'],
      ['  0.1.0 → * in pnpmOverrides of packages/a/package.json'],
      ['✘ e'],
      ['  0.1.0 → * in peerDependencies of packages/a/package.json'],
      ['✘ f'],
      ['  0.1.0 → * in resolutions of packages/a/package.json'],
    ]);
  });

  it('does not include ignored dependencies in its output', () => {
    const scenario = scenarios.semverIsIgnored();
    lintSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.log.mock.calls).toEqual([
      ['✘ foo'],
      ['  0.1.0 → ~0.1.0 in dependencies of packages/a/package.json'],
    ]);
  });
});
