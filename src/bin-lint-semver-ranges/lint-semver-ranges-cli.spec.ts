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
      ['✕ b 0.1.0 in devDependencies of a should be ~0.1.0'],
      ['✕ c 0.1.0 in overrides of a should be ~0.1.0'],
      ['✕ d 0.1.0 in pnpmOverrides of a should be ~0.1.0'],
      ['✕ e 0.1.0 in peerDependencies of a should be ~0.1.0'],
      ['✕ f 0.1.0 in resolutions of a should be ~0.1.0'],
    ]);
  });

  it('ensures wildcard versions are supported', () => {
    const scenario = scenarios.semverRangesDoNotMatchConfigWildcard();
    lintSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.log.mock.calls).toEqual([
      ['✕ b 0.1.0 in devDependencies of a should be *'],
      ['✕ c 0.1.0 in overrides of a should be *'],
      ['✕ d 0.1.0 in pnpmOverrides of a should be *'],
      ['✕ e 0.1.0 in peerDependencies of a should be *'],
      ['✕ f 0.1.0 in resolutions of a should be *'],
    ]);
  });

  it('does not include ignored dependencies in its output', () => {
    const scenario = scenarios.semverIsIgnored();
    lintSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.log.mock.calls).toEqual([
      ['✕ foo 0.1.0 in dependencies of a should be ~0.1.0'],
    ]);
  });
});
