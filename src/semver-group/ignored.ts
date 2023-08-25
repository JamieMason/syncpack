import { Data, Effect } from 'effect';
import type { SemverGroupConfig } from '../config/types';
import type { Instance } from '../get-instances/instance';
import { Report } from '../report';
import type { Specifier } from '../specifier';
import type { NonSemverError } from '../specifier/lib/non-semver-error';

/** Every instance in this group is ignored and nothing will be changed */
export class IgnoredSemverGroup extends Data.TaggedClass('Ignored')<{
  config: SemverGroupConfig.Ignored;
  instances: Instance[];
}> {
  groupType = 'semverGroup';

  constructor(config: SemverGroupConfig.Ignored) {
    super({
      config,
      instances: [],
    });
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  getFixed(specifier: Specifier.Any): Effect.Effect<never, NonSemverError, Specifier.Any> {
    return Effect.succeed(specifier);
  }

  inspectAll() {
    return Effect.all(this.instances.map((instance) => this.inspect(instance)));
  }

  inspect(instance: Instance): Effect.Effect<never, never, Report.Ignored> {
    return Effect.succeed(
      new Report.Ignored({
        instance,
      }),
    );
  }
}
