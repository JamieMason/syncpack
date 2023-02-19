import { formatCli } from '../../src/bin-format/format-cli';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/** F E D should appear first, then the rest in A-Z order */
describe('Sort first', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', {
            omitName: true,
            otherProps: { A: '', F: '', B: '', D: '', E: '' },
          }),
          after: mockPackage('a', {
            omitName: true,
            otherProps: { F: '', E: '', D: '', A: '', B: '' },
          }),
        },
      ],
      {
        sortFirst: ['F', 'E', 'D'],
      },
    );
  }

  describe('fix-mismatches', () => {
    //
  });

  describe('format', () => {
    it('sorts named properties first, then the rest alphabetically', () => {
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
