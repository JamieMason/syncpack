import type { SemverGroupReport } from '.';
import { getFilter } from '../config/get-filter';
import type { GroupConfig } from '../config/types';
import type { Context } from '../get-context';
import type { Instance } from '../get-package-json-files/instance';

export class FilteredOutSemverGroup {
  _tag = 'FilteredOut';
  config: GroupConfig = {
    dependencies: ['**'],
    dependencyTypes: [],
    label: 'Filtered out',
    packages: ['**'],
  };
  filter: string;
  instances: Instance[];

  constructor(ctx: Context) {
    this.filter = getFilter(ctx.config);
    this.instances = [];
  }

  canAdd(instance: Instance): boolean {
    return instance.name.search(new RegExp(this.filter)) === -1;
  }

  inspect(): SemverGroupReport[] {
    return this.instances.map((instance) => ({
      status: 'FILTERED_OUT',
      instance,
      isValid: true,
      name: instance.name,
    }));
  }
}
