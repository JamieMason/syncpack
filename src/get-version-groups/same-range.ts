import intersects from 'semver/ranges/intersects';
import type { VersionGroupReport } from '.';
import type { VersionGroupConfig } from '../config/types';
import type { Instance } from '../get-package-json-files/instance';
import { groupBy } from './lib/group-by';

export class SameRangeVersionGroup {
  _tag = 'SameRange';
  config: VersionGroupConfig.SameRange;
  instances: Instance[];

  constructor(config: VersionGroupConfig.SameRange) {
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
      if (hasMismatch(instances)) {
        report.push({
          instances,
          isValid: false,
          name,
          status: 'SAME_RANGE_MISMATCH',
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

/** Every range must fall within every other range */
function hasMismatch(instances: Instance[]) {
  const loose = true;
  return instances.some((a) =>
    instances.some((b) => !intersects(a.version, b.version, loose)),
  );
}
