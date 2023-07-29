import * as Data from '@effect/data/Data';
import * as Effect from '@effect/io/Effect';
import { VersionGroupReport } from '.';
import type { VersionGroupConfig } from '../config/types';
import type { Instance } from '../instance';
import { groupBy } from './lib/group-by';

export class SnappedToVersionGroup extends Data.TaggedClass('SnappedTo')<{
  config: VersionGroupConfig.SnappedTo;
  instances: Instance.Any[];
}> {
  constructor(config: VersionGroupConfig.SnappedTo) {
    super({
      config,
      instances: [],
    });
  }

  canAdd(_: Instance.Any): boolean {
    return true;
  }

  inspect(): Effect.Effect<
    never,
    VersionGroupReport.SnappedToMismatch,
    VersionGroupReport.Valid
  >[] {
    const instancesByName = groupBy('name', this.instances);
    return Object.entries(instancesByName).map(([name, instances]) => {
      const snapTo = this.config.snapTo;
      const expectedVersion = getExpectedVersion(snapTo, instances);
      if (hasMismatch(expectedVersion, instances)) {
        return Effect.fail(
          new VersionGroupReport.SnappedToMismatch({
            name,
            instances,
            isValid: false,
            expectedVersion,
            snapTo,
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

function getExpectedVersion(snapTo: string[], instances: Instance.Any[]): string {
  const expectedVersion = instances
    .filter((i) => snapTo.includes(i.pkgName))
    .find((i) => i.specifier)?.specifier;
  if (expectedVersion) return expectedVersion;
  // @FIXME: create tagged error for this
  throw new Error('versionGroup.snapTo does not match any package versions');
}

function hasMismatch(expectedVersion: string, instances: Instance.Any[]) {
  return instances.some((instance) => instance.specifier !== expectedVersion);
}
