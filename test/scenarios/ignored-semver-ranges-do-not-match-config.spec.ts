import { normalize } from 'path';
import { lintSemverRangesCli } from '../../src/bin-lint-semver-ranges/lint-semver-ranges-cli';
import { setSemverRangesCli } from '../../src/bin-set-semver-ranges/set-semver-ranges-cli';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - All dependencies are checked
 * - The semver range `~` should be used
 * - `one` uses exact versions
 * - `b` and `c` are ignored
 * - All but `b` and `c` should use `~`
 */
describe('Ignored semver ranges do not match config', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', {
            deps: ['a@0.1.0'],
            devDeps: ['b@0.1.0'],
            overrides: ['c@0.1.0'],
            pnpmOverrides: ['d@0.1.0'],
            peerDeps: ['e@0.1.0'],
            resolutions: ['f@0.1.0'],
          }),
          after: mockPackage('a', {
            deps: ['a@~0.1.0'],
            devDeps: ['b@0.1.0'],
            overrides: ['c@0.1.0'],
            pnpmOverrides: ['d@~0.1.0'],
            peerDeps: ['e@~0.1.0'],
            resolutions: ['f@~0.1.0'],
          }),
        },
      ],
      {
        semverRange: '~',
        semverGroups: [
          {
            dependencies: ['b', 'c'],
            dependencyTypes: [],
            packages: ['**'],
            isIgnored: true,
          },
        ],
        types: 'dev,overrides,pnpmOverrides,peer,prod,resolutions,workspace',
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
    it('list issues from multiple custom types and semver groups together', () => {
      const scenario = getScenario();
      const a = normalize('packages/a/package.json');
      lintSemverRangesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [expect.stringMatching('Default Semver Group')],
        ['✘ a'],
        [`  0.1.0 → ~0.1.0 in dependencies of ${a}`],
        ['✘ d'],
        [`  0.1.0 → ~0.1.0 in pnpm.overrides of ${a}`],
        ['✘ e'],
        [`  0.1.0 → ~0.1.0 in peerDependencies of ${a}`],
        ['✘ f'],
        [`  0.1.0 → ~0.1.0 in resolutions of ${a}`],
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
    it('fixes multiple custom types and semver groups together', () => {
      const scenario = getScenario();
      setSemverRangesCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenChanged,
      ]);
    });
  });
});
