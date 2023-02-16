import 'expect-more-jest';
import { customTypesAndSemverGroups } from '../../test/scenarios/custom-types-and-semver-groups';
import { issue84Reproduction } from '../../test/scenarios/issue84-reproduction';
import { matchingUnsupportedVersions } from '../../test/scenarios/matching-unsupported-versions';
import { semverIsIgnored } from '../../test/scenarios/semver-is-ignored';
import { semverRangesDoNotMatchConfig } from '../../test/scenarios/semver-ranges-do-not-match-config';
import { setSemverRangesCli } from './set-semver-ranges-cli';

describe('setSemverRanges', () => {
  beforeEach(() => {
    jest.restoreAllMocks();
  });

  it('sets all versions to use the supplied range', () => {
    const scenario = semverRangesDoNotMatchConfig();
    setSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
    ]);
  });

  it('leaves ignored dependencies unchanged', () => {
    const scenario = semverIsIgnored();
    setSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
      scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
    ]);
  });

  it('leaves non-semver versions unchanged', () => {
    const scenario = matchingUnsupportedVersions();
    setSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
      scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
    ]);
    expect(scenario.disk.process.exit).not.toHaveBeenCalled();
  });

  it('fixes issue 84', () => {
    const scenario = issue84Reproduction();
    setSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
      scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
    ]);
  });

  it('fixes multiple custom types and semver groups together', () => {
    const scenario = customTypesAndSemverGroups();
    setSemverRangesCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
    ]);
  });
});
