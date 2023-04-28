import type { SemverGroupReport } from '.';
import type { SemverGroupConfig } from '../config/types';
import type { Instance } from '../get-package-json-files/instance';

export class IgnoredSemverGroup {
  _tag = 'Ignored';
  config: SemverGroupConfig.Ignored;
  instances: Instance[];

  constructor(config: SemverGroupConfig.Ignored) {
    this.config = config;
    this.instances = [];
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  inspect(): SemverGroupReport[] {
    return this.instances.map((instance) => ({
      status: 'IGNORED',
      instance,
      isValid: true,
      name: instance.name,
    }));
  }
}
