import { mockPackage } from '../mock';
import { createScenario } from './lib/create-scenario';

/** F E D should appear first, then the rest in A-Z order */
export function sortFirst() {
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
      sortFirst: ['F', 'E', 'D'],
    },
  );
}
