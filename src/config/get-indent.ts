import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import { DEFAULT_CONFIG } from '../constants';
import type { Ctx } from '../get-context';

export function getIndent({ cli, rcFile }: Ctx['config']): string {
  return isNonEmptyString(cli.indent)
    ? cli.indent.replaceAll('\\t', '\t')
    : isNonEmptyString(rcFile.indent)
      ? rcFile.indent
      : DEFAULT_CONFIG.indent;
}
