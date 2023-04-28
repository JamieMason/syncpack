import type { VersionGroupReport } from '.';
import type { VersionGroupConfig } from '../config/types';
import type { Instance } from '../get-package-json-files/instance';
import { groupBy } from './lib/group-by';

export class SnappedToVersionGroup {
  _tag = 'SnappedTo';
  config: VersionGroupConfig.SnappedTo;
  instances: Instance[];

  constructor(config: VersionGroupConfig.SnappedTo) {
    this.config = config;
    this.instances = [];
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  inspect(): VersionGroupReport[] {
    const report: VersionGroupReport[] = [];
    const instancesByName = groupBy('name', this.instances);

    Object.entries(instancesByName).forEach(([name, instances]) => {
      const snapTo = this.config.snapTo;
      const expectedVersion = getExpectedVersion(snapTo, instances);

      if (hasMismatch(expectedVersion, instances)) {
        report.push({
          expectedVersion,
          instances,
          isValid: false,
          name,
          status: 'SNAPPED_TO_MISMATCH',
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

function getExpectedVersion(snapTo: string[], instances: Instance[]): string {
  const expectedVersion = instances
    .filter((i) => snapTo.includes(i.pkgName))
    .find((i) => i.version)?.version;
  if (expectedVersion) return expectedVersion;
  throw new Error('versionGroup.snapTo does not match any package versions');
}

function hasMismatch(expectedVersion: string, instances: Instance[]) {
  return instances.some((instance) => instance.version !== expectedVersion);
}
