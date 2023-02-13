import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/** "scripts" object keys should be A-Z but is not */
export function sortObjectProps() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', {
          otherProps: { scripts: { B: '', A: '' } },
        }),
        after: mockPackage('a', {
          otherProps: { scripts: { A: '', B: '' } },
        }),
      },
    ],
    {
      sortAz: ['scripts'],
    },
  );
}
