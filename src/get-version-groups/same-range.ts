import * as Data from '@effect/data/Data';
import * as Effect from '@effect/io/Effect';
import intersects from 'semver/ranges/intersects';
import { VersionGroupReport } from '.';
import type { VersionGroupConfig } from '../config/types';
import type { Instance } from '../instance';
import { groupBy } from './lib/group-by';

export class SameRangeVersionGroup extends Data.TaggedClass('SameRange')<{
  config: VersionGroupConfig.SameRange;
  instances: Instance.Any[];
}> {
  constructor(config: VersionGroupConfig.SameRange) {
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
    VersionGroupReport.SameRangeMismatch,
    VersionGroupReport.Valid
  >[] {
    const instancesByName = groupBy('name', this.instances);
    return Object.entries(instancesByName).map(([name, instances]) => {
      if (hasMismatch(instances)) {
        return Effect.fail(
          new VersionGroupReport.SameRangeMismatch({
            name,
            instances,
            isValid: false,
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

/** Every range must fall within every other range */
function hasMismatch(instances: Instance.Any[]) {
  const loose = true;
  return instances.some((a) =>
    instances.some(
      (b) => !intersects(aliasWorkspaceRange(a.specifier), aliasWorkspaceRange(b.specifier), loose),
    ),
  );
}

function aliasWorkspaceRange(version: string): string {
  return version.startsWith('workspace:*') ? '*' : version;
}
