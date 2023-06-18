import * as Data from '@effect/data/Data';
import * as Effect from '@effect/io/Effect';
import { SemverGroupReport } from '.';
import type { SemverGroupConfig } from '../config/types';
import type { Instance } from '../get-package-json-files/instance';
import { isSupported } from '../guards/is-supported';
import { setSemverRange } from '../lib/set-semver-range';

export class WithRangeSemverGroup extends Data.TaggedClass('WithRange')<{
  config: SemverGroupConfig.WithRange;
  instances: Instance[];
  isCatchAll: boolean;
}> {
  constructor(isCatchAll: boolean, config: SemverGroupConfig.WithRange) {
    super({
      config,
      instances: [],
      isCatchAll,
    });
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  inspect(): Effect.Effect<
    never,
    | SemverGroupReport.UnsupportedVersion
    | SemverGroupReport.WorkspaceSemverRangeMismatch
    | SemverGroupReport.SemverRangeMismatch,
    SemverGroupReport.Valid
  >[] {
    return this.instances.map((instance) => {
      if (!isSupported(instance.version)) {
        return Effect.fail(
          new SemverGroupReport.UnsupportedVersion({
            name: instance.name,
            instance,
            isValid: false,
          }),
        );
      }

      const isWsInstance = instance.strategy.name === 'workspace';
      const exactVersion = setSemverRange('', instance.version);
      const expectedVersion = setSemverRange(this.config.range, instance.version);

      if (isWsInstance && instance.version !== exactVersion) {
        return Effect.fail(
          new SemverGroupReport.WorkspaceSemverRangeMismatch({
            name: instance.name,
            instance,
            isValid: false,
            expectedVersion: exactVersion,
          }),
        );
      }
      if (instance.version === expectedVersion) {
        return Effect.succeed(
          new SemverGroupReport.Valid({
            name: instance.name,
            instance,
            isValid: true,
          }),
        );
      }
      return Effect.fail(
        new SemverGroupReport.SemverRangeMismatch({
          name: instance.name,
          instance,
          isValid: false,
          expectedVersion,
        }),
      );
    });
  }
}
