import type { SemverGroupReport } from '.';
import type { SemverGroupConfig } from '../config/types';
import type { Instance } from '../get-package-json-files/instance';
import { isSupported } from '../lib/is-semver';
import { setSemverRange } from '../lib/set-semver-range';

export class WithRangeSemverGroup {
  _tag = 'WithRange';
  config: SemverGroupConfig.WithRange;
  instances: Instance[];

  constructor(config: SemverGroupConfig.WithRange) {
    this.config = config;
    this.instances = [];
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  inspect(): SemverGroupReport[] {
    return this.instances.map((instance) => {
      if (!isSupported(instance.version)) {
        return {
          status: 'UNSUPPORTED_VERSION',
          instance,
          isValid: false,
          name: instance.name,
        };
      }

      const isWsInstance = instance.strategy.name === 'workspace';
      const exactVersion = setSemverRange('', instance.version);
      const expectedVersion = setSemverRange(
        this.config.range,
        instance.version,
      );

      if (isWsInstance && instance.version !== exactVersion) {
        return {
          status: 'WORKSPACE_SEMVER_RANGE_MISMATCH',
          expectedVersion: exactVersion,
          instance,
          isValid: false,
          name: instance.name,
        };
      }
      if (instance.version === expectedVersion) {
        return {
          status: 'VALID',
          instance,
          isValid: true,
          name: instance.name,
        };
      }
      return {
        status: 'SEMVER_RANGE_MISMATCH',
        expectedVersion,
        instance,
        isValid: false,
        name: instance.name,
      };
    });
  }
}
