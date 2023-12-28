import { Data, Effect } from 'effect';
import { getFilter } from '../config/get-filter';
import type { GroupConfig } from '../config/types';
import type { Ctx } from '../get-context';
import type { Instance } from '../get-instances/instance';
import { Report } from '../report';
import { groupBy } from './lib/group-by';

export class FilteredOutVersionGroup extends Data.TaggedClass('FilteredOut')<{
  config: GroupConfig;
  filter: string;
  instances: Instance[];
}> {
  groupType = 'versionGroup';

  constructor(ctx: Ctx) {
    super({
      config: {
        dependencies: ['**'],
        dependencyTypes: ['**'],
        label: 'Filtered out',
        packages: ['**'],
      },
      filter: getFilter(ctx.config),
      instances: [] satisfies Instance[],
    });
  }

  canAdd(instance: Instance): boolean {
    return instance.name.search(new RegExp(this.filter)) === -1;
  }

  inspectAll(): Effect.Effect<never, never, Report.Version.Group[]> {
    return Effect.succeed(
      Object.entries(groupBy('name', this.instances)).map(([name, instances]) => ({
        name,
        reports: instances.map(
          (instance) =>
            // ✓ is ignored and dismissed as valid
            new Report.FilteredOut(instance),
        ),
      })),
    );
  }
}
