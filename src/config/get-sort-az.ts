import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { isEmptyArray } from 'tightrope/guard/is-empty-array';
import { DEFAULT_CONFIG } from '../constants';
import type { Ctx } from '../get-context';

export function getSortAz({ rcFile }: Ctx['config']): string[] {
  return isArrayOfStrings(rcFile.sortAz) || isEmptyArray(rcFile.sortAz)
    ? rcFile.sortAz
    : DEFAULT_CONFIG.sortAz;
}
