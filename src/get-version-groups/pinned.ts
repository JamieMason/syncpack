import type { VersionGroupReport } from '.';
import type { VersionGroupConfig } from '../config/types';
import type { Instance } from '../get-package-json-files/instance';
import { groupBy } from './lib/group-by';

export class PinnedVersionGroup {
  _tag = 'Pinned';
  config: VersionGroupConfig.Pinned;
  instances: Instance[];

  constructor(config: VersionGroupConfig.Pinned) {
    this.config = config;
    this.instances = [];
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  inspect(): VersionGroupReport[] {
    const report: VersionGroupReport[] = [];
    const instancesByName = groupBy('name', this.instances);
    const expectedVersion = this.config.pinVersion;

    Object.entries(instancesByName).forEach(([name, instances]) => {
      if (hasMismatch(expectedVersion, instances)) {
        report.push({
          expectedVersion,
          instances,
          isValid: false,
          name,
          status: 'PINNED_MISMATCH',
        });
      } else {
        report.push({
          instances,
          isValid: true,
          name,
          status: 'VALID',
        });
      }
    });

    return report;
  }
}

function hasMismatch(pinVersion: string, instances: Instance[]) {
  return instances.some((instance) => instance.version !== pinVersion);
}
