import * as Effect from '@effect/io/Effect';
import { formatCli } from '../../../src/bin-format/format-cli';
import { mockPackage } from '../../lib/mock';
import { createScenario } from '../lib/create-scenario';

/** "keywords" array should be A-Z but is not */

describe('format', () => {
  it('sorts array properties alphabetically by value', () => {
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
          before: mockPackage('a', { otherProps: { keywords: ['B', 'A'] } }),
          after: mockPackage('a', { otherProps: { keywords: ['A', 'B'] } }),
        },
      ],
      {
        cli: {},
        rcFile: {
          sortAz: ['keywords'],
        },
      },
    );
  }
});
