import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings.js';
import { isEmptyArray } from 'tightrope/guard/is-empty-array.js';
import { DEFAULT_CONFIG } from '../constants.js';
import type { Ctx } from '../get-context/index.js';

export function getSortExports({ rcFile }: Ctx['config']): string[] {
  return isArrayOfStrings(rcFile.sortExports) || isEmptyArray(rcFile.sortExports)
    ? rcFile.sortExports
    : DEFAULT_CONFIG.sortExports;
}
