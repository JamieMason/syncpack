import * as Data from '@effect/data/Data';
import * as Effect from '@effect/io/Effect';
import { VersionGroupReport } from '.';
import { getFilter } from '../config/get-filter';
import type { GroupConfig } from '../config/types';
import type { Ctx } from '../get-context';
import type { Instance } from '../get-package-json-files/instance';
import { groupBy } from './lib/group-by';

export class FilteredOutVersionGroup extends Data.TaggedClass('FilteredOut')<{
  config: GroupConfig;
  filter: string;
  instances: Instance[];
}> {
  constructor(ctx: Ctx) {
    super({
      config: {
        dependencies: ['**'],
        dependencyTypes: [],
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

  inspect(): Effect.Effect<never, never, VersionGroupReport.FilteredOut>[] {
    const instancesByName = groupBy('name', this.instances);
    return Object.entries(instancesByName).map(([name, instances]) =>
      Effect.succeed(
        new VersionGroupReport.FilteredOut({
          name,
          instances,
          isValid: true,
        }),
      ),
    );
  }
}
