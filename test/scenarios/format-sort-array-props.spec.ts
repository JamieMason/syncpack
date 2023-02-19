import { formatCli } from '../../src/bin-format/format-cli';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/** "keywords" array should be A-Z but is not */
describe('format: Sort array props', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', {
            otherProps: { keywords: ['B', 'A'] },
          }),
          after: mockPackage('a', {
            otherProps: { keywords: ['A', 'B'] },
          }),
        },
      ],
      {
        sortAz: ['keywords'],
      },
    );
  }

  describe('fix-mismatches', () => {
    //
  });

  describe('format', () => {
    it('sorts array properties alphabetically by value', () => {
      const scenario = getScenario();
      formatCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenChanged,
      ]);
    });
  });

  describe('lint-semver-ranges', () => {
    //
  });

  describe('list-mismatches', () => {
    //
  });

  describe('list', () => {
    //
  });

  describe('set-semver-ranges', () => {
    //
  });
});
