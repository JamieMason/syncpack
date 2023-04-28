import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import type { Context } from '../get-context';

export function getIndent({ cli, rcFile }: Context['config']): string {
  return isNonEmptyString(cli.indent)
    ? cli.indent
    : isNonEmptyString(rcFile.indent)
    ? rcFile.indent
    : '  ';
}
