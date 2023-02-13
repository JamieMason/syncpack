import 'expect-more-jest';
import { githubShorthand } from '../../test/scenarios/github-shorthand';
import { protectedShorthand } from '../../test/scenarios/protected-shorthand';
import { shorthand } from '../../test/scenarios/shorthand';
import { sortArrayProps } from '../../test/scenarios/sort-array-props';
import { sortFirst } from '../../test/scenarios/sort-first';
import { sortObjectProps } from '../../test/scenarios/sort-object-props';
import { formatCli } from './format-cli';

describe('format', () => {
  it('sorts array properties alphabetically by value', () => {
    const scenario = sortArrayProps();
    formatCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
    ]);
  });
  it('sorts object properties alphabetically by key', () => {
    const scenario = sortObjectProps();
    formatCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
    ]);
  });
  it('sorts named properties first, then the rest alphabetically', () => {
    const scenario = sortFirst();
    formatCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
    ]);
  });
  it('uses shorthand format for "bugs" and "repository"', () => {
    const scenario = shorthand();
    formatCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
    ]);
  });
  it('retains long form format for "repository" when directory property used', () => {
    const scenario = protectedShorthand();
    formatCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
    ]);
  });
  it('uses github shorthand format for "repository"', () => {
    const scenario = githubShorthand();
    formatCli(scenario.config, scenario.disk);
    expect(scenario.disk.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
    expect(scenario.log.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].logEntryWhenChanged,
    ]);
  });
});
