import * as Effect from '@effect/io/Effect';
import { formatCli } from '../../../src/bin-format/format-cli';
import { mockPackage } from '../../lib/mock';
import { createScenario } from '../lib/create-scenario';

/** "repository" contains properties which cannot be shortened */
describe('format', () => {
  it('retains long form format for "repository" when directory property used', () => {
    const scenario = getScenario();
    Effect.runSync(formatCli(scenario.config.cli, scenario.env));
    expect(scenario.env.writeFileSync.mock.calls).toEqual([]);

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
        { cli: {}, rcFile: {} },
      );
    }
  });
});
