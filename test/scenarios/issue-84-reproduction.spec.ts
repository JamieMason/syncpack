import { setSemverRangesCli } from '../../src/bin-set-semver-ranges/set-semver-ranges-cli';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * @see https://github.com/JamieMason/syncpack/issues/84#issue-1284878219
 */
describe('Issue 84 reproduction', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('@myscope/a', { deps: ['@myscope/a@1.0.0'] }),
          after: mockPackage('@myscope/a', { deps: ['@myscope/a@^1.0.0'] }),
        },
        {
          path: 'packages/b/package.json',
          before: mockPackage('@myscope/b', {}),
          after: mockPackage('@myscope/b', {}),
        },
      ],
      {
        semverGroups: [
          {
            range: '^',
            dependencies: ['@myscope/**'],
            dependencyTypes: [],
            packages: ['**'],
          },
        ],
        semverRange: '~',
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
    //
  });

  describe('list-mismatches', () => {
    //
  });

  describe('list', () => {
    //
  });

  describe('set-semver-ranges', () => {
    it('fixes issue 84', () => {
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
