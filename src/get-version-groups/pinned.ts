import * as Data from '@effect/data/Data';
import * as Effect from '@effect/io/Effect';
import { VersionGroupReport } from '.';
import type { VersionGroupConfig } from '../config/types';
import type { Instance } from '../get-package-json-files/instance';
import { groupBy } from './lib/group-by';

export class PinnedVersionGroup extends Data.TaggedClass('Pinned')<{
  config: VersionGroupConfig.Pinned;
  instances: Instance[];
}> {
  constructor(config: VersionGroupConfig.Pinned) {
    super({
      config,
      instances: [],
    });
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  inspect(): Effect.Effect<never, VersionGroupReport.PinnedMismatch, VersionGroupReport.Valid>[] {
    const instancesByName = groupBy('name', this.instances);
    const expectedVersion = this.config.pinVersion;

    return Object.entries(instancesByName).map(([name, instances]) => {
      if (hasMismatch(expectedVersion, instances)) {
        return Effect.fail(
          new VersionGroupReport.PinnedMismatch({
            name,
            instances,
            isValid: false,
            expectedVersion,
          }),
        );
      } else {
        return Effect.succeed(
          new VersionGroupReport.Valid({
            name,
            instances,
            isValid: true,
          }),
        );
      }
    });
  }
}

function hasMismatch(pinVersion: string, instances: Instance[]) {
  return instances.some((instance) => instance.version !== pinVersion);
}
