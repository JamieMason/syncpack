import { normalize } from 'path';
import { fixMismatchesCli } from '../../src/bin-fix-mismatches/fix-mismatches-cli';
import { listMismatchesCli } from '../../src/bin-list-mismatches/list-mismatches-cli';
import { ICON } from '../../src/constants';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - .syncpackrc has a custom type defined to check the "somePlugin.version" property.
 * - A has 2.0.0
 * - B has 3.0.0
 * - A should be fixed to use 3.0.0
 */
describe('customTypes: version mismatch', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', {
            otherProps: { somePlugin: { version: '2.0.0' } },
          }),
          after: mockPackage('a', {
            otherProps: { somePlugin: { version: '3.0.0' } },
          }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', {
            otherProps: { somePlugin: { version: '3.0.0' } },
          }),
          after: mockPackage('b', {
            otherProps: { somePlugin: { version: '3.0.0' } },
          }),
        },
      ],
      {
        customTypes: {
          engines: {
            strategy: 'version',
            path: 'somePlugin.version',
          },
        },
      },
    );
  }

  describe('fix-mismatches', () => {
    it('fixes "version" mismatches in custom locations', () => {
      const scenario = getScenario();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenChanged,
        scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
      ]);
    });
  });

  describe('format', () => {
    //
  });

  describe('lint-semver-ranges', () => {
    //
  });

  it('synchronises pinned versions', () => {
    const scenario = getScenario();
    const a = 'packages/a/package.json';
    listMismatchesCli(scenario.config, scenario.disk);
    expect(scenario.log.mock.calls).toEqual([
      [
        `${ICON.cross} version 3.0.0 is the highest valid semver version in use`,
      ],
      [`  2.0.0 in somePlugin.version of ${normalize(a)}`],
    ]);
  });

  describe('list', () => {
    //
  });

  describe('set-semver-ranges', () => {
    //
  });
});
