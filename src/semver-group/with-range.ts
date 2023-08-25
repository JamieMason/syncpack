import { Data, Effect, pipe } from 'effect';
import type { SemverGroupConfig } from '../config/types';
import type { Instance } from '../get-instances/instance';
import { setSemverRange } from '../lib/set-semver-range';
import { Report } from '../report';
import { Specifier } from '../specifier';
import type { NonSemverError } from '../specifier/lib/non-semver-error';

export class WithRangeSemverGroup extends Data.TaggedClass('WithRange')<{
  config: SemverGroupConfig.WithRange;
  instances: Instance[];
  isCatchAll: boolean;
}> {
  groupType = 'semverGroup';

  constructor(isCatchAll: boolean, config: SemverGroupConfig.WithRange) {
    super({
      config,
      instances: [],
      isCatchAll,
    });
    this.getFixed = this.getFixed.bind(this);
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  getFixed(specifier: Specifier.Any): Effect.Effect<never, NonSemverError, Specifier.Any> {
    return pipe(
      specifier.getSemver(),
      Effect.map((semver) => setSemverRange(this.config.range, semver)),
      Effect.flatMap((nextSemver) => specifier.setSemver(nextSemver)),
    );
  }

  inspectAll() {
    return Effect.all(this.instances.map((instance) => this.inspect(instance)));
  }

  inspect(
    instance: Instance,
  ): Effect.Effect<
    never,
    never,
    Report.UnsupportedMismatch | Report.SemverRangeMismatch | Report.Valid
  > {
    const current = Specifier.create(instance, instance.rawSpecifier);
    return pipe(
      this.getFixed(current),
      Effect.match({
        // if range is fixable
        onSuccess: (valid) =>
          // if it is pinned and matches its pin
          instance.versionGroup._tag === 'Pinned' &&
          instance.rawSpecifier === instance.versionGroup.config.pinVersion
            ? // the pinned version takes precendence and is a match
              new Report.Valid({
                specifier: current,
              })
            : // if it is already like this on disk
            instance.rawSpecifier === valid.raw
            ? // it is a match
              new Report.Valid({
                specifier: current,
              })
            : // it is a mismatch and should be this one
              new Report.SemverRangeMismatch({
                fixable: valid,
              }),

        // if range is NOT fixable, it is a mismatch we can't auto-fix
        // as it seems to not be semver
        onFailure: () =>
          new Report.UnsupportedMismatch({
            unfixable: instance,
          }),
      }),
    );
  }
}
