import 'expect-more-jest';
import { scenarios } from '../../test/scenarios';
import { getInput } from '../lib/get-input';
import { lintSemverRanges } from './lint-semver-ranges';

describe('lintSemverRanges', () => {
  beforeEach(() => {
    jest.restoreAllMocks();
  });

  it('lists versions with ranges which do not match the project config', () => {
    const scenario = scenarios.semverRangesDoNotMatchConfig();
    lintSemverRanges(getInput(scenario.disk, scenario.config), scenario.disk);
    expect(scenario.log.mock.calls).toEqual([
      ['✕ bar 2.0.0 in dependencies of a should be ~2.0.0'],
      ['✕ foo 0.1.0 in dependencies of a should be ~0.1.0'],
    ]);
  });
});
