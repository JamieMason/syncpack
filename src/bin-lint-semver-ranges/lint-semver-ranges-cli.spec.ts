import 'expect-more-jest';
import { normalize } from 'path';
import { scenarios } from '../../test/scenarios';
import { lintSemverRangesCli } from './lint-semver-ranges-cli';

describe('lintSemverRanges', () => {
  beforeEach(() => {
    jest.restoreAllMocks();
  });

  it('lists versions with ranges which do not match the project config', () => {
    const scenario = scenarios.semverRangesDoNotMatchConfig();
    const a = normalize('packages/a/package.json');
    lintSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.log.mock.calls).toEqual([
      ['✘ b'],
      [`  0.1.0 → ~0.1.0 in devDependencies of ${a}`],
      ['✘ c'],
      [`  0.1.0 → ~0.1.0 in overrides of ${a}`],
      ['✘ d'],
      [`  0.1.0 → ~0.1.0 in pnpm.overrides of ${a}`],
      ['✘ e'],
      [`  0.1.0 → ~0.1.0 in peerDependencies of ${a}`],
      ['✘ f'],
      [`  0.1.0 → ~0.1.0 in resolutions of ${a}`],
    ]);
  });

  it('ensures wildcard versions are supported', () => {
    const scenario = scenarios.semverRangesDoNotMatchConfigWildcard();
    const a = normalize('packages/a/package.json');
    lintSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.log.mock.calls).toEqual([
      ['✘ b'],
      [`  0.1.0 → * in devDependencies of ${a}`],
      ['✘ c'],
      [`  0.1.0 → * in overrides of ${a}`],
      ['✘ d'],
      [`  0.1.0 → * in pnpm.overrides of ${a}`],
      ['✘ e'],
      [`  0.1.0 → * in peerDependencies of ${a}`],
      ['✘ f'],
      [`  0.1.0 → * in resolutions of ${a}`],
    ]);
  });

  it('does not include ignored dependencies in its output', () => {
    const scenario = scenarios.semverIsIgnored();
    const a = normalize('packages/a/package.json');
    lintSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.log.mock.calls).toEqual([
      ['✘ foo'],
      [`  0.1.0 → ~0.1.0 in dependencies of ${a}`],
    ]);
  });
});
