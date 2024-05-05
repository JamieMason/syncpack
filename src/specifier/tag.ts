import { BaseSpecifier } from './base.js';
import type { SpecificRegistryResult } from './lib/specific-registry-result.js';

/**
 * @example "latest"
 * @example "made-up-by-some-dev"
 */
export class TagSpecifier extends BaseSpecifier<SpecificRegistryResult<'tag'>> {
  _tag = 'Tag';

  /** The public name referenced in config */
  name = 'tag' as const;
}
