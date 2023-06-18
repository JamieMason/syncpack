import * as Effect from '@effect/io/Effect';
import { formatCli } from '../../../src/bin-format/format-cli';
import { mockPackage } from '../../mock';
import { createScenario } from '../lib/create-scenario';

/** F E D should appear first, then the rest in A-Z order */
describe('format', () => {
  it('sorts named properties first, then the rest alphabetically', () => {
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
          before: mockPackage('a', {
            omitName: true,
            otherProps: { A: '', F: '', B: '', D: '', E: '' },
          }),
          after: mockPackage('a', {
            omitName: true,
            otherProps: { F: '', E: '', D: '', A: '', B: '' },
          }),
        },
      ],
      {
        cli: {},
        rcFile: {
          sortFirst: ['F', 'E', 'D'],
        },
      },
    );
  }
});
