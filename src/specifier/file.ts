import type { FileResult } from 'npm-package-arg';
import { BaseSpecifier } from './base.js';

/** @example */
export class FileSpecifier extends BaseSpecifier<FileResult> {
  _tag = 'File';

  /** The public name referenced in config */
  name = 'file' as const;
}
