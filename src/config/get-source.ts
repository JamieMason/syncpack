import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings.js';
import type { Ctx } from '../get-context/index.js';

export function getSource({ cli, rcFile }: Ctx['config']): string[] {
  return isArrayOfStrings(cli.source)
    ? cli.source
    : isArrayOfStrings(rcFile.source)
      ? rcFile.source
      : [];
}
