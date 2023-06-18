import * as Data from '@effect/data/Data';
import * as Effect from '@effect/io/Effect';
import { VersionGroupReport } from '.';
import type { VersionGroupConfig } from '../config/types';
import type { Instance } from '../get-package-json-files/instance';
import { groupBy } from './lib/group-by';

export class IgnoredVersionGroup extends Data.TaggedClass('Ignored')<{
  config: VersionGroupConfig.Ignored;
  instances: Instance[];
}> {
  constructor(config: VersionGroupConfig.Ignored) {
    super({
      config,
      instances: [],
    });
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  inspect(): Effect.Effect<never, never, VersionGroupReport.Ignored>[] {
    const instancesByName = groupBy('name', this.instances);
    return Object.entries(instancesByName).map(([name, instances]) =>
      Effect.succeed(
        new VersionGroupReport.Ignored({
          name,
          instances,
          isValid: true,
        }),
      ),
    );
  }
}
