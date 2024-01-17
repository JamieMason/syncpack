import { Data, Effect } from 'effect';
import { getFilter } from '../config/get-filter.js';
import type { GroupConfig } from '../config/types.js';
import type { Ctx } from '../get-context/index.js';
import type { Instance } from '../get-instances/instance.js';
import { Report } from '../report.js';
import type { Specifier } from '../specifier/index.js';
import type { NonSemverError } from '../specifier/lib/non-semver-error.js';

/**
 * Instances which do not match a given `--filter` option are assigned to this
 * group and nothing will be changed.
 */
export class FilteredOutSemverGroup extends Data.TaggedClass('FilteredOut')<{
  config: GroupConfig;
  filter: string;
  instances: Instance[];
}> {
  groupType = 'semverGroup';

  constructor(ctx: Ctx) {
    super({
      config: {
        dependencies: ['**'],
        dependencyTypes: ['**'],
        label: 'Filtered out',
        packages: ['**'],
      },
      filter: getFilter(ctx.config),
      instances: [],
    });
  }

  canAdd(instance: Instance): boolean {
    return instance.name.search(new RegExp(this.filter)) === -1;
  }

  getFixed(specifier: Specifier.Any): Effect.Effect<never, NonSemverError, Specifier.Any> {
    return Effect.succeed(specifier);
  }

  inspectAll() {
    return Effect.all(this.instances.map((instance) => this.inspect(instance)));
  }

  inspect(instance: Instance): Effect.Effect<never, never, Report.FilteredOut> {
    return Effect.succeed(new Report.FilteredOut(instance));
  }
}
