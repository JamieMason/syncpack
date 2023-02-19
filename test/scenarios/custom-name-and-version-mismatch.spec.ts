import { fixMismatchesCli } from '../../src/bin-fix-mismatches/fix-mismatches-cli';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - .syncpackrc has a custom type to check the "packageManager" property.
 * - A has yarn@2
 * - B has yarn@3
 * - A should be fixed to use yarn@3
 */
describe('Custom name and version mismatch', () => {
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
});
