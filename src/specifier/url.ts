import type { URLResult } from 'npm-package-arg';
import { BaseSpecifier } from './base.js';

/** @example "http://x.com/foo.tgz" */
export class UrlSpecifier extends BaseSpecifier<URLResult> {
  _tag = 'Url';

  /** The public name referenced in config */
  name = 'url' as const;

  // @TODO: If file name is semver, return that in getSemver()
}
