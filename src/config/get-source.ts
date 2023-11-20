import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import type { Ctx } from '../get-context';

export function getSource({ cli, rcFile }: Ctx['config']): string[] {
  return isArrayOfStrings(cli.source)
    ? cli.source
    : isArrayOfStrings(rcFile.source)
      ? rcFile.source
      : [];
}
