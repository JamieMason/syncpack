import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { isEmptyArray } from 'tightrope/guard/is-empty-array';
import { DEFAULT_CONFIG } from '../constants';
import type { Ctx } from '../get-context';

export function getSortExports({ rcFile }: Ctx['config']): string[] {
  return isArrayOfStrings(rcFile.sortExports) || isEmptyArray(rcFile.sortExports)
    ? rcFile.sortExports
    : DEFAULT_CONFIG.sortExports;
}
