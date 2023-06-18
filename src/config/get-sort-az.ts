import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import type { Ctx } from '../get-context';

export function getSortAz({ rcFile }: Ctx['config']): string[] {
  return isArrayOfStrings(rcFile.sortAz)
    ? rcFile.sortAz
    : [
        'bin',
        'contributors',
        'dependencies',
        'devDependencies',
        'keywords',
        'peerDependencies',
        'resolutions',
        'scripts',
      ];
}
