import { BaseSpecifier } from './base';

/** A specifier not supported by the `npm` package manager */
export class UnsupportedSpecifier extends BaseSpecifier<unknown> {
  _tag = 'Unsupported' as const;
}
