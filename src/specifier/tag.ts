import { BaseSpecifier } from './base';
import type { SpecificRegistryResult } from './lib/specific-registry-result';

/**
 * @example "latest"
 * @example "made-up-by-some-dev"
 */
export class TagSpecifier extends BaseSpecifier<SpecificRegistryResult<'tag'>> {
  _tag = 'Tag';

  /** The public name referenced in config */
  name = 'tag' as const;
}
