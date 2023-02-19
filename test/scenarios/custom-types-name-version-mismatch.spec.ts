import { normalize } from 'path';
import { fixMismatchesCli } from '../../src/bin-fix-mismatches/fix-mismatches-cli';
import { listMismatchesCli } from '../../src/bin-list-mismatches/list-mismatches-cli';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - .syncpackrc has a custom type to check the "packageManager" property.
 * - A has yarn@2
 * - B has yarn@3
 * - A should be fixed to use yarn@3
 */
describe('customTypes: name@version mismatch', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', {
            otherProps: { packageManager: 'yarn@2.0.0' },
          }),
          after: mockPackage('a', {
            otherProps: { packageManager: 'yarn@3.0.0' },
          }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', {
            otherProps: { packageManager: 'yarn@3.0.0' },
          }),
          after: mockPackage('b', {
            otherProps: { packageManager: 'yarn@3.0.0' },
          }),
        },
      ],
      {
        customTypes: {
          engines: {
            strategy: 'name@version',
            path: 'packageManager',
          },
        },
      },
    );
  }

  describe('fix-mismatches', () => {
    it('fixes "name@version" mismatches in custom locations', () => {
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

  describe('list-mismatches', () => {
    it('fixes "name@version" mismatches in custom locations', () => {
      const scenario = getScenario();
      const a = normalize('packages/a/package.json');
      listMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        ['✘ yarn 3.0.0 is the highest valid semver version in use'],
        [`  2.0.0 in packageManager of ${a}`],
      ]);
    });
  });
});
