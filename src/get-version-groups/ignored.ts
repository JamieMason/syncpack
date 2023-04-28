import type { VersionGroupReport } from '.';
import type { VersionGroupConfig } from '../config/types';
import type { Instance } from '../get-package-json-files/instance';
import { groupBy } from './lib/group-by';

export class IgnoredVersionGroup {
  _tag = 'Ignored';
  config: VersionGroupConfig.Ignored;
  instances: Instance[];

  constructor(config: VersionGroupConfig.Ignored) {
    this.config = config;
    this.instances = [];
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  inspect(): VersionGroupReport[] {
    const instancesByName = groupBy('name', this.instances);
    return Object.entries(instancesByName).map(([name, instances]) => ({
      instances,
      isValid: true,
      name,
      status: 'IGNORED',
    }));
  }
}
