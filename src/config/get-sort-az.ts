import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import type { Context } from '../get-context';

export function getSortAz({ rcFile }: Context['config']): string[] {
  return isArrayOfStrings(rcFile.sortAz)
    ? rcFile.sortAz
    : [
        'contributors',
        'dependencies',
        'devDependencies',
        'keywords',
        'peerDependencies',
        'resolutions',
        'scripts',
      ];
}
