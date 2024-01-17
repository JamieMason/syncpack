import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string.js';
import { DEFAULT_CONFIG } from '../constants.js';
import type { Ctx } from '../get-context/index.js';

export function getIndent({ cli, rcFile }: Ctx['config']): string {
  return isNonEmptyString(cli.indent)
    ? cli.indent.replaceAll('\\t', '\t')
    : isNonEmptyString(rcFile.indent)
      ? rcFile.indent
      : DEFAULT_CONFIG.indent;
}
