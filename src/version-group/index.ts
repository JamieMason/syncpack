import { Data } from 'effect';
import type { BannedVersionGroup } from './banned.js';
import type { FilteredOutVersionGroup } from './filtered-out.js';
import type { IgnoredVersionGroup } from './ignored.js';
import type { PinnedVersionGroup } from './pinned.js';
import type { SameRangeVersionGroup } from './same-range.js';
import type { SnappedToVersionGroup } from './snapped-to.js';
import type { StandardVersionGroup } from './standard.js';

export namespace VersionGroup {
  export type Banned = BannedVersionGroup;
  export type FilteredOut = FilteredOutVersionGroup;
  export type Ignored = IgnoredVersionGroup;
  export type Pinned = PinnedVersionGroup;
  export type SameRange = SameRangeVersionGroup;
  export type SnappedTo = SnappedToVersionGroup;
  export type Standard = StandardVersionGroup;

  export type Any = Banned | FilteredOut | Ignored | Pinned | SameRange | SnappedTo | Standard;

  export class ConfigError extends Data.TaggedClass('VersionGroupConfigError')<{
    readonly config: unknown;
    readonly error: string;
  }> {}
}
