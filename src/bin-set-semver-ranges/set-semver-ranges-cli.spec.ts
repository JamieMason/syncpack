import 'expect-more-jest';
import { scenarios } from '../../test/scenarios';
import { setSemverRangesCli } from './set-semver-ranges-cli';

describe('setSemverRanges', () => {
  beforeEach(() => {
    jest.restoreAllMocks();
  });

  it.only('sets all versions to use the supplied range', () => {
    const scenario = scenarios.semverRangesDoNotMatchConfig();
    setSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
    ]);
  });

  it('leaves ignored dependencies unchanged', () => {
    const scenario = scenarios.semverIsIgnored();
    setSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
      scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
    ]);
  });

  it('fixes issue 84', () => {
    const scenario = scenarios.issue84Reproduction();
    setSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
      scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
    ]);
  });
});
