import { formatCli } from '../../src/bin-format/format-cli';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/** "bugs" and "repository" can safely use equivalent shorthands */
describe('shorthand properties', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', {
            omitName: true,
            otherProps: {
              bugs: { url: 'https://github.com/User/repo/issues' },
              repository: { url: 'git://gitlab.com/User/repo', type: 'git' },
            },
          }),
          after: mockPackage('a', {
            omitName: true,
            otherProps: {
              bugs: 'https://github.com/User/repo/issues',
              repository: 'git://gitlab.com/User/repo',
            },
          }),
        },
      ],
      {},
    );
  }

  describe('fix-mismatches', () => {
    //
  });

  describe('format', () => {
    it('uses shorthand format for "bugs" and "repository"', () => {
      const scenario = getScenario();
      formatCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].diskWriteWhenChanged,
      ]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenChanged,
      ]);
    });
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
