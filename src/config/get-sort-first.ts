import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { isEmptyArray } from 'tightrope/guard/is-empty-array';
import { DEFAULT_CONFIG } from '../constants';
import type { Ctx } from '../get-context';

export function getSortFirst({ rcFile }: Ctx['config']): string[] {
  return isArrayOfStrings(rcFile.sortFirst) || isEmptyArray(rcFile.sortFirst)
    ? rcFile.sortFirst
    : DEFAULT_CONFIG.sortFirst;
}
