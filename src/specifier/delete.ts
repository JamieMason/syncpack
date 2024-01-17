import type { Instance } from '../get-instances/instance.js';
import type { Delete } from '../version-group/lib/delete.js';
import { BaseSpecifier } from './base.js';

/** A instance which should be deleted */
export class DeleteSpecifier {
  _tag = 'Delete';

  /** The public name referenced in config */
  name = 'delete' as const;

  raw: Delete;
  instance: Instance;

  constructor(data: { raw: Delete; instance: Instance }) {
    this.raw = data.raw;
    this.instance = data.instance;
  }

  getSemver = BaseSpecifier.prototype.getSemver;

  setSemver = BaseSpecifier.prototype.setSemver;

  replaceWith = BaseSpecifier.prototype.replaceWith;
}
