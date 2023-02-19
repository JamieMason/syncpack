import { normalize } from 'path';
import { lintSemverRangesCli } from '../../src/bin-lint-semver-ranges/lint-semver-ranges-cli';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/**
 * - Only `dependencies` is unchecked
 * - The semver range `*` should be used
 * - A uses exact versions for `a`
 * - A should be fixed to use `*` in all other cases
 */
describe('Semver ranges do not match config wildcard', () => {
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
            devDeps: ['*'],
            overrides: ['*'],
            pnpmOverrides: ['*'],
            peerDeps: ['*'],
            resolutions: ['*'],
          }),
        },
      ],
      {
        semverRange: '*',
        types: 'dev,overrides,pnpmOverrides,peer,resolutions,workspace',
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
    it('ensures wildcard versions are supported', () => {
      const scenario = getScenario();
      const a = normalize('packages/a/package.json');
      lintSemverRangesCli(scenario.config, scenario.disk);
      expect(scenario.log.mock.calls).toEqual([
        ['✘ b'],
        [`  0.1.0 → * in devDependencies of ${a}`],
        ['✘ c'],
        [`  0.1.0 → * in overrides of ${a}`],
        ['✘ d'],
        [`  0.1.0 → * in pnpm.overrides of ${a}`],
        ['✘ e'],
        [`  0.1.0 → * in peerDependencies of ${a}`],
        ['✘ f'],
        [`  0.1.0 → * in resolutions of ${a}`],
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
    //
  });
});
