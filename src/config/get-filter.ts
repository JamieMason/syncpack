import { isNonEmptyString } from 'tightrope/guard/is-non-empty-string';
import type { Context } from '../get-context';

export function getFilter({ cli, rcFile }: Context['config']): string {
  // @TODO Deprecate `filter` in .syncpackrc
  return isNonEmptyString(cli.filter)
    ? cli.filter
    : isNonEmptyString(rcFile.filter)
    ? rcFile.filter
    : '.';
}
