import { Data, Effect } from 'effect';
import type { SemverGroupConfig } from '../config/types.js';
import type { Instance } from '../get-instances/instance.js';
import { Report } from '../report.js';
import type { Specifier } from '../specifier/index.js';
import type { NonSemverError } from '../specifier/lib/non-semver-error.js';

/**
 * Semver groups are disabled by default and, when that's the case, every
 * instance is assigned to this group. This group will allow anything.
 */
export class DisabledSemverGroup extends Data.TaggedClass('Disabled')<{
  config: SemverGroupConfig.Disabled;
  instances: Instance[];
  isCatchAll: boolean;
}> {
  groupType = 'semverGroup';

  constructor(isCatchAll: boolean, config: SemverGroupConfig.Disabled) {
    super({
      config,
      instances: [],
      isCatchAll,
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

  inspect(instance: Instance): Effect.Effect<never, never, Report.Disabled> {
    return Effect.succeed(new Report.Disabled(instance));
  }
}
