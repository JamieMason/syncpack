import { Data, Effect } from 'effect';
import type { VersionGroupConfig } from '../config/types.js';
import type { Instance } from '../get-instances/instance.js';
import { Report } from '../report.js';
import { groupBy } from './lib/group-by.js';

export class IgnoredVersionGroup extends Data.TaggedClass('Ignored')<{
  config: VersionGroupConfig.Ignored;
  instances: Instance[];
}> {
  groupType = 'versionGroup';

  constructor(config: VersionGroupConfig.Ignored) {
    super({
      config,
      instances: [],
    });
  }

  canAdd(_: Instance): boolean {
    return true;
  }

  inspectAll(): Effect.Effect<never, never, Report.Version.Group[]> {
    return Effect.succeed(
      Object.entries(groupBy('name', this.instances)).map(([name, instances]) => ({
        name,
        reports: instances.map(
          (instance) =>
            // ✓ is ignored and dismissed as valid
            new Report.Ignored(instance),
        ),
      })),
    );
  }
}
