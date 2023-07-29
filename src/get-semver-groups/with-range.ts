import * as Data from '@effect/data/Data';
import * as Option from '@effect/data/Option';
import * as Effect from '@effect/io/Effect';
import { SemverGroupReport } from '.';
import type { SemverGroupConfig } from '../config/types';
import type { Instance } from '../instance';
import { setSemverRange } from '../lib/set-semver-range';

export class WithRangeSemverGroup extends Data.TaggedClass('WithRange')<{
  config: SemverGroupConfig.WithRange;
  instances: Instance.Any[];
  isCatchAll: boolean;
}> {
  constructor(isCatchAll: boolean, config: SemverGroupConfig.WithRange) {
    super({
      config,
      instances: [],
      isCatchAll,
    });
  }

  canAdd(_: Instance.Any): boolean {
    return true;
  }

  inspect(): Effect.Effect<
    never,
    | SemverGroupReport.NonSemverVersion
    | SemverGroupReport.LocalPackageSemverRangeMismatch
    | SemverGroupReport.SemverRangeMismatch,
    SemverGroupReport.Valid
  >[] {
    return this.instances.map((instance) => {
      if (Option.isNone(instance.getSemverSpecifier())) {
        return Effect.fail(
          new SemverGroupReport.NonSemverVersion({
            name: instance.name,
            instance,
            isValid: false,
          }),
        );
      }

      const isLocalPackageInstance = instance.strategy.name === 'localPackage';
      const exactVersion = setSemverRange('', instance.specifier);
      const expectedVersion = setSemverRange(this.config.range, instance.specifier);

      if (isLocalPackageInstance && instance.specifier !== exactVersion) {
        return Effect.fail(
          new SemverGroupReport.LocalPackageSemverRangeMismatch({
            name: instance.name,
            instance,
            isValid: false,
            expectedVersion: exactVersion,
          }),
        );
      }
      if (instance.specifier === expectedVersion) {
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
