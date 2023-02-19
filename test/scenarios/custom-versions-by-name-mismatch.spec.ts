import { fixMismatchesCli } from '../../src/bin-fix-mismatches/fix-mismatches-cli';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - .syncpackrc has a custom type defined to check the "engines" property.
 * - A has node 14
 * - B has node 16
 * - A should be fixed to use 16
 */
describe('Custom versions by name mismatch', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', {
            otherProps: { engines: { node: '14.0.0' } },
          }),
          after: mockPackage('a', {
            otherProps: { engines: { node: '16.0.0' } },
          }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', {
            otherProps: { engines: { node: '16.0.0' } },
          }),
          after: mockPackage('b', {
            otherProps: { engines: { node: '14.0.0' } },
          }),
        },
      ],
      {
        customTypes: {
          engines: {
            strategy: 'versionsByName',
            path: 'engines',
          },
        },
      },
    );
  }

  describe('fix-mismatches', () => {
    it('fixes "versionsByName" mismatches in custom locations', () => {
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
