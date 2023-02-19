import { normalize } from 'path';
import { lintSemverRangesCli } from '../../src/bin-lint-semver-ranges/lint-semver-ranges-cli';
import { setSemverRangesCli } from '../../src/bin-set-semver-ranges/set-semver-ranges-cli';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - A does not depend on `bar`
 * - B does depend on `bar`
 * - `bar` is ignored by syncpack in every package
 * - `foo` is not ignored
 * - `bar` is unprotected so can have mismatching range etc
 * - only `foo` should have its semver range fixed
 */
describe('Semver is ignored', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', { deps: ['foo@0.1.0', 'bar@1.1.1'] }),
          after: mockPackage('a', { deps: ['foo@~0.1.0', 'bar@1.1.1'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('b', { deps: ['bar@0.2.0'] }),
          after: mockPackage('b', { deps: ['bar@0.2.0'] }),
        },
      ],
      {
        semverRange: '~',
        semverGroups: [
          {
            dependencies: ['bar'],
            dependencyTypes: [],
            packages: ['**'],
            isIgnored: true,
          },
        ],
      },
    );
  }

  describe('fix-mismatches', () => {
    //
  });

  describe('format', () => {
    //
  });

  describe('lint-semver-ranges', () => {
    it('does not include ignored dependencies in its output', () => {
      const scenario = getScenario();
      const a = normalize('packages/a/package.json');
      lintSemverRangesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching(/Default Semver Group/)],
        ['✘ foo'],
        [`  0.1.0 → ~0.1.0 in dependencies of ${a}`],
      ]);
    });
  });

  describe('list-mismatches', () => {
    //
  });

  describe('list', () => {
    //
  });

  describe('set-semver-ranges', () => {
    it('leaves ignored dependencies unchanged', () => {
      const scenario = getScenario();
      setSemverRangesCli(scenario.config, scenario.disk);
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
