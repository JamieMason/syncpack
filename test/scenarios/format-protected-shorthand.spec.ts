import { formatCli } from '../../src/bin-format/format-cli';
import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/** "repository" contains properties which cannot be shortened */
describe('format: Protected shorthand', () => {
  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', {
            omitName: true,
            otherProps: {
              repository: {
                url: 'git://gitlab.com/User/repo',
                type: 'git',
                directory: 'packages/foo',
              },
            },
          }),
          after: mockPackage('a', {
            omitName: true,
            otherProps: {
              repository: {
                url: 'git://gitlab.com/User/repo',
                type: 'git',
                directory: 'packages/foo',
              },
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
    it('retains long form format for "repository" when directory property used', () => {
      const scenario = getScenario();
      formatCli(scenario.config, scenario.disk);
      expect(scenario.disk.writeFileSync.mock.calls).toEqual([]);
      expect(scenario.log.mock.calls).toEqual([
        scenario.files['packages/a/package.json'].logEntryWhenUnchanged,
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
