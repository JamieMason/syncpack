import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/** "keywords" array should be A-Z but is not */
export function sortArrayProps() {
  return createScenario(
    [
      {
        path: 'packages/a/package.json',
        before: mockPackage('a', {
          otherProps: { keywords: ['B', 'A'] },
        }),
        after: mockPackage('a', {
          otherProps: { keywords: ['A', 'B'] },
        }),
      },
    ],
    {
      sortAz: ['keywords'],
    },
  );
}
