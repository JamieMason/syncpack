import * as Data from '@effect/data/Data';
import * as Effect from '@effect/io/Effect';
import { SemverGroupReport } from '.';
import { getFilter } from '../config/get-filter';
import type { GroupConfig } from '../config/types';
import type { Ctx } from '../get-context';
import type { Instance } from '../instance';

export class FilteredOutSemverGroup extends Data.TaggedClass('FilteredOut')<{
  config: GroupConfig;
  filter: string;
  instances: Instance.Any[];
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
      instances: [],
    });
  }

  canAdd(instance: Instance.Any): boolean {
    return instance.name.search(new RegExp(this.filter)) === -1;
  }

  inspect(): Effect.Effect<never, never, SemverGroupReport.FilteredOut>[] {
    return this.instances.map((instance) =>
      Effect.succeed(
        new SemverGroupReport.FilteredOut({
          name: instance.name,
          instance,
          isValid: true,
        }),
      ),
    );
  }
}
