import { BaseSpecifier } from './base.js';

/** A specifier not supported by the `npm` package manager */
export class UnsupportedSpecifier extends BaseSpecifier<unknown> {
  _tag = 'Unsupported';

  /** The public name referenced in config */
  name = 'unsupported' as const;
}
