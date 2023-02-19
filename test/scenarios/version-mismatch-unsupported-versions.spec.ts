import { normalize } from 'path';
import { fixMismatchesCli } from '../../src/bin-fix-mismatches/fix-mismatches-cli';
import { listMismatchesCli } from '../../src/bin-list-mismatches/list-mismatches-cli';
import { listCli } from '../../src/bin-list/list-cli';
import { ICON } from '../../src/constants';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - A, B, C & D depend on foo
 * - The versions mismatch
 * - Some versions are not semver
 * - `0.3.0` is the highest valid semver version
 * - Syncpack can't know what the Developers intend with them
 * - All packages should be left unchanged
 */
describe('version mismatch: unsupported versions', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', { deps: ['foo@link:vendor/foo-0.1.0'] }),
          after: mockPackage('a', { deps: ['foo@link:vendor/foo-0.1.0'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', { deps: ['foo@workspace:*'] }),
          after: mockPackage('b', { deps: ['foo@workspace:*'] }),
        },
        {
          path: 'packages/c/package.json',
          before: mockPackage('c', { deps: ['foo@0.3.0'] }),
          after: mockPackage('c', { deps: ['foo@0.3.0'] }),
        },
        {
          path: 'packages/d/package.json',
          before: mockPackage('d', { deps: ['foo@0.2.0'] }),
          after: mockPackage('d', { deps: ['foo@0.2.0'] }),
        },
      ],
      {},
    );
  }

  describe('fix-mismatches', () => {
    it('skips mismatched versions which syncpack cannot fix', () => {
      const scenario = getScenario();
      fixMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync).not.toHaveBeenCalled();
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/b/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/c/package.json'].logEntryWhenUnchanged,
        scenario.files['packages/d/package.json'].logEntryWhenUnchanged,
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
    it('lists mismatched versions which syncpack cannot fix', () => {
      const scenario = getScenario();
      const a = 'packages/a/package.json';
      const b = 'packages/b/package.json';
      const c = 'packages/c/package.json';
      const d = 'packages/d/package.json';
      listMismatchesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [`${ICON.cross} foo has mismatched versions which syncpack cannot fix`],
        [`  link:vendor/foo-0.1.0 in dependencies of ${normalize(a)}`],
        [`  workspace:* in dependencies of ${normalize(b)}`],
        [`  0.3.0 in dependencies of ${normalize(c)}`],
        [`  0.2.0 in dependencies of ${normalize(d)}`],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });
  });

  describe('list', () => {
    it('lists mismatched versions which syncpack cannot fix', () => {
      const scenario = getScenario();
      listCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [
          '✘ foo has mismatched versions which syncpack cannot fix: 0.2.0, 0.3.0, link:vendor/foo-0.1.0, workspace:*',
        ],
      ]);
      expect(scenario.disk.process.exit).toHaveBeenCalledWith(1);
    });
  });

  describe('set-semver-ranges', () => {
    //
  });
});
