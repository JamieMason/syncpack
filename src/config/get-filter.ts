import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string.js';
import { DEFAULT_CONFIG } from '../constants.js';
import type { Ctx } from '../get-context/index.js';

export function getFilter({ cli, rcFile }: Ctx['config']): string {
  // @TODO Deprecate `filter` in .syncpackrc
  return isNonEmptyString(cli.filter)
    ? cli.filter
    : isNonEmptyString(rcFile.filter)
      ? rcFile.filter
      : DEFAULT_CONFIG.filter;
}
