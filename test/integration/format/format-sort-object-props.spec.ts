import * as Effect from '@effect/io/Effect';
import { formatCli } from '../../../src/bin-format/format-cli';
import { mockPackage } from '../../lib/mock';
import { createScenario } from '../lib/create-scenario';

/** "scripts" object keys should be A-Z but is not */

describe('format', () => {
  it('sorts object properties alphabetically by key', () => {
    const scenario = getScenario();
    Effect.runSync(formatCli(scenario.config.cli, scenario.env));
    expect(scenario.env.writeFileSync.mock.calls).toEqual([
      scenario.files['packages/a/package.json'].diskWriteWhenChanged,
    ]);
  });

  function getScenario() {
    return createScenario(
      [
        {
          path: 'packages/a/package.json',
          before: mockPackage('a', { otherProps: { scripts: { B: '', A: '' } } }),
          after: mockPackage('a', { otherProps: { scripts: { A: '', B: '' } } }),
        },
      ],
      {
        cli: {},
        rcFile: {
          sortAz: ['scripts'],
        },
      },
    );
  }
});
