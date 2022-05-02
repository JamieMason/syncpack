import 'expect-more-jest';
import { scenarios } from '../../test/scenarios';
import { getInput } from '../lib/get-input';
import { setSemverRanges } from './set-semver-ranges';

describe('setSemverRanges', () => {
  beforeEach(() => {
    jest.restoreAllMocks();
  });

  it('sets all versions to use the supplied range', () => {
    const scenario = scenarios.semverRangesDoNotMatchConfig();
    setSemverRanges(getInput(scenario.disk, scenario.config), scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
    ]);
  });
});
