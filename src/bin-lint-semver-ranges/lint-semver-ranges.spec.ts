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
      ['✕ b 0.1.0 in devDependencies of a should be ~0.1.0'],
      ['✕ c 0.1.0 in overrides of a should be ~0.1.0'],
      ['✕ d 0.1.0 in peerDependencies of a should be ~0.1.0'],
      ['✕ e 0.1.0 in resolutions of a should be ~0.1.0'],
    ]);
  });
});
