import * as Data from '@effect/data/Data';
import * as Effect from '@effect/io/Effect';
import { VersionGroupReport } from '.';
import type { VersionGroupConfig } from '../config/types';
import type { Instance } from '../get-package-json-files/instance';
import { groupBy } from './lib/group-by';

export class BannedVersionGroup extends Data.TaggedClass('Banned')<{
  config: VersionGroupConfig.Banned;
  instances: Instance[];
}> {
  constructor(config: VersionGroupConfig.Banned) {
    super({
      config,
      instances: [],
    });
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  inspect(): Effect.Effect<never, VersionGroupReport.Banned, never>[] {
    const instancesByName = groupBy('name', this.instances);
    return Object.entries(instancesByName).map(([name, instances]) =>
      Effect.fail(
        new VersionGroupReport.Banned({
          name,
          instances,
          isValid: false,
        }),
      ),
    );
  }
}
