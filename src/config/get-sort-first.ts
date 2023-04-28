import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import type { Context } from '../get-context';

export function getSortFirst({ rcFile }: Context['config']): string[] {
  return isArrayOfStrings(rcFile.sortFirst)
    ? rcFile.sortFirst
    : ['name', 'description', 'version', 'author'];
}
