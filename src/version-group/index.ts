import { Data } from 'effect';
import type { Union } from 'ts-toolbelt';
import type { BannedVersionGroup } from './banned';
import type { FilteredOutVersionGroup } from './filtered-out';
import type { IgnoredVersionGroup } from './ignored';
import type { PinnedVersionGroup } from './pinned';
import type { SameRangeVersionGroup } from './same-range';
import type { SnappedToVersionGroup } from './snapped-to';
import type { StandardVersionGroup } from './standard';

export namespace VersionGroup {
  export type Banned = BannedVersionGroup;
  export type FilteredOut = FilteredOutVersionGroup;
  export type Ignored = IgnoredVersionGroup;
  export type Pinned = PinnedVersionGroup;
  export type SameRange = SameRangeVersionGroup;
  export type SnappedTo = SnappedToVersionGroup;
  export type Standard = StandardVersionGroup;

  export type Any = Union.Strict<
    Banned | FilteredOut | Ignored | Pinned | SameRange | SnappedTo | Standard
  >;

  export class ConfigError extends Data.TaggedClass('VersionGroupConfigError')<{
    readonly config: unknown;
    readonly error: string;
  }> {}
}
