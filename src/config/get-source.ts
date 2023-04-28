import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { DEFAULT_CONFIG } from '../constants';
import type { Context } from '../get-context';

export function getSource({ cli, rcFile }: Context['config']): string[] {
  return isArrayOfStrings(cli.source)
    ? cli.source
    : isArrayOfStrings(rcFile.source)
    ? rcFile.source
    : [...DEFAULT_CONFIG.source];
}
