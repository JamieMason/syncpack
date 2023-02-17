import 'expect-more-jest';
import { normalize } from 'path';
import { customTypesAndSemverGroups } from '../../test/scenarios/custom-types-and-semver-groups';
import { semverIsIgnored } from '../../test/scenarios/semver-is-ignored';
import { semverRangesDoNotMatchConfig } from '../../test/scenarios/semver-ranges-do-not-match-config';
import { semverRangesDoNotMatchConfigWildcard } from '../../test/scenarios/semver-ranges-do-not-match-config-wildcard';
import { lintSemverRangesCli } from './lint-semver-ranges-cli';

describe('lintSemverRanges', () => {
  beforeEach(() => {
    jest.restoreAllMocks();
  });

  it('lists versions with ranges which do not match the project config', () => {
    const scenario = semverRangesDoNotMatchConfig();
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
    const scenario = semverRangesDoNotMatchConfigWildcard();
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
    const scenario = semverIsIgnored();
    const a = normalize('packages/a/package.json');
    lintSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.log.mock.calls).toEqual([
      [expect.stringMatching(/Default Semver Group/)],
      ['✘ foo'],
      [`  0.1.0 → ~0.1.0 in dependencies of ${a}`],
    ]);
  });

  it('list issues from multiple custom types and semver groups together', () => {
    const scenario = customTypesAndSemverGroups();
    const a = normalize('packages/a/package.json');
    lintSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.log.mock.calls).toEqual([
      [expect.stringMatching(/Semver Group 1/)],
      ['✘ node'],
      [`  16.16.0 → >=16.16.0 in engines.node of ${a}`],
      [expect.stringMatching(/Semver Group 2/)],
      ['✘ npm'],
      [`  7.24.2 → ^7.24.2 in engines.npm of ${a}`],
      [expect.stringMatching(/Semver Group 3/)],
      ['✘ yarn'],
      [`  ~2.0.0 → 2.0.0 in packageManager of ${a}`],
    ]);
  });
});
