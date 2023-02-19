import { normalize } from 'path';
import { lintSemverRangesCli } from '../../src/bin-lint-semver-ranges/lint-semver-ranges-cli';
import { setSemverRangesCli } from '../../src/bin-set-semver-ranges/set-semver-ranges-cli';
import { ICON } from '../../src/constants';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - Only `dependencies` is unchecked
 * - The semver range `*` should be used
 * - A uses exact versions for `a`
 * - A should be fixed to use `*` in all other cases
 */
describe('range mismatch: asterisk and --types filter', () => {
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
            deps: ['a@0.1.0'],
            devDeps: ['b@*'],
            overrides: ['c@*'],
            pnpmOverrides: ['d@*'],
            peerDeps: ['e@*'],
            resolutions: ['f@*'],
          }),
        },
      ],
      {
        semverRange: '*',
        types: 'dev,overrides,pnpmOverrides,peer,resolutions,workspace',
      },
    );
  }

  describe('lint-semver-ranges', () => {
    it('lists the mismatches except for ignored ones in production', () => {
      const scenario = getScenario();
      const a = normalize('packages/a/package.json');
      lintSemverRangesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        [`${ICON.cross} b`],
        [`  0.1.0 → * in devDependencies of ${a}`],
        [`${ICON.cross} c`],
        [`  0.1.0 → * in overrides of ${a}`],
        [`${ICON.cross} d`],
        [`  0.1.0 → * in pnpm.overrides of ${a}`],
        [`${ICON.cross} e`],
        [`  0.1.0 → * in peerDependencies of ${a}`],
        [`${ICON.cross} f`],
        [`  0.1.0 → * in resolutions of ${a}`],
      ]);
    });
  });

  describe('set-semver-ranges', () => {
    it('fixes range mismatches except for production', () => {
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
