import type { VersionGroupReport } from '.';
import { getFilter } from '../config/get-filter';
import type { GroupConfig } from '../config/types';
import type { Context } from '../get-context';
import type { Instance } from '../get-package-json-files/instance';
import { groupBy } from './lib/group-by';

export class FilteredOutVersionGroup {
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

  inspect(): VersionGroupReport[] {
    const instancesByName = groupBy('name', this.instances);
    return Object.entries(instancesByName).map(([name, instances]) => ({
      instances,
      isValid: true,
      name,
      status: 'FILTERED_OUT',
    }));
  }
}
