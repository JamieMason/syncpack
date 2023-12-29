import type { FileResult } from 'npm-package-arg';
import { BaseSpecifier } from './base';

/** @example */
export class FileSpecifier extends BaseSpecifier<FileResult> {
  _tag = 'File' as const;
}
