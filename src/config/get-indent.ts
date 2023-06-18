import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import type { Ctx } from '../get-context';

export function getIndent({ cli, rcFile }: Ctx['config']): string {
  return isNonEmptyString(cli.indent)
    ? cli.indent
    : isNonEmptyString(rcFile.indent)
    ? rcFile.indent
    : '  ';
}
