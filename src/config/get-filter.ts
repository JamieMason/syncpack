import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import type { Ctx } from '../get-context';
import { DEFAULT_CONFIG } from '../constants';

export function getFilter({ cli, rcFile }: Ctx['config']): string {
  // @TODO Deprecate `filter` in .syncpackrc
  return isNonEmptyString(cli.filter)
    ? cli.filter
    : isNonEmptyString(rcFile.filter)
      ? rcFile.filter
      : DEFAULT_CONFIG.filter;
}
