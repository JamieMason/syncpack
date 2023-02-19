import { normalize } from 'path';
import { fixMismatchesCli } from '../../src/bin-fix-mismatches/fix-mismatches-cli';
import { listMismatchesCli } from '../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../src/bin-list/list-cli';
import { ICON } from '../../src/constants';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - A has an older version of react than B
 * - A is marked as a source of truth via the `snapTo` property
 * - C does not use react so should be unchanged
 * - B should be downgraded, even though it is on a newer version
 *
 * @see https://github.com/JamieMason/syncpack/issues/87#issuecomment-1182456452
 */
describe('versionGroup.snapTo', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', { deps: ['react@15.6.1'] }),
          after: mockPackage('a', { deps: ['react@15.6.1'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', { deps: ['react@18.0.0'] }),
          after: mockPackage('b', { deps: ['react@15.6.1'] }),
        },
        {
          path: 'packages/c/package.json',
          before: mockPackage('c', { deps: ['foo@0.1.0'] }),
          after: mockPackage('c', { deps: ['foo@0.1.0'] }),
        },
      ],
      {
        versionGroups: [
          {
            dependencies: ['react'],
            packages: ['**'],
            snapTo: ['a'],
          },
        ],
      },
    );
  }

  describe('fix-mismatches', () => {
    it('fixes using the version from the snapTo target package', () => {
      const scenario = getScenario();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/b/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/b/package.json'].logEntryWhenChanged,
        scenario.files['packages/c/package.json'].logEntryWhenUnchanged,
      ]);
    });
  });

  describe('list-mismatches', () => {
    it('suggests using the version from the snapTo target package', () => {
      const scenario = getScenario();
      const b = normalize('packages/b/package.json');
      listMismatchesCli(scenario.config, scenario.disk);

      expect(scenario.log.mock.calls).toEqual([
        [expect.stringContaining('Version Group 1')],
        [`${ICON.cross} react should snap to 15.6.1, used by a`],
        [`  18.0.0 in dependencies of ${b}`],
      ]);
    });
  });

  describe('list', () => {
    it('suggests using the version from the snapTo target package', () => {
      const scenario = getScenario();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringContaining('Version Group 1')],
        [`${ICON.cross} react 15.6.1, 18.0.0`],
        [expect.stringContaining('Default Version Group')],
        ['- foo 0.1.0'],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });
  });
});
