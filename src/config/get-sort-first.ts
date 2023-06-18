import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import type { Ctx } from '../get-context';

export function getSortFirst({ rcFile }: Ctx['config']): string[] {
  return isArrayOfStrings(rcFile.sortFirst)
    ? rcFile.sortFirst
    : ['name', 'description', 'version', 'author'];
}
