import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import type { Ctx } from '../get-context';
import { DEFAULT_CONFIG } from '../constants';

export function getIndent({ cli, rcFile }: Ctx['config']): string {
  return isNonEmptyString(cli.indent)
    ? cli.indent.replaceAll('\\t', '\t')
    : isNonEmptyString(rcFile.indent)
      ? rcFile.indent
      : DEFAULT_CONFIG.indent;
}
