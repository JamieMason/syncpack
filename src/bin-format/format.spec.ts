import 'expect-more-jest';
import { scenarios } from '../../test/scenarios';
import { formatCli } from './format-cli';

describe('format', () => {
  it('sorts array properties alphabetically by value', () => {
    const scenario = scenarios.sortArrayProps();
    formatCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
    ]);
  });
  it('sorts object properties alphabetically by key', () => {
    const scenario = scenarios.sortObjectProps();
    formatCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
    ]);
  });
  it('sorts named properties first, then the rest alphabetically', () => {
    const scenario = scenarios.sortFirst();
    formatCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
    ]);
  });
  it('uses shorthand format for "bugs" and "repository"', () => {
    const scenario = scenarios.shorthand();
    formatCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
    ]);
  });
  it('retains long form format for "repository" when directory property used', () => {
    const scenario = scenarios.protectedShorthand();
    formatCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
    ]);
  });
  it('uses github shorthand format for "repository"', () => {
    const scenario = scenarios.githubShorthand();
    formatCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
    ]);
  });
});
