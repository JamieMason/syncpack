import { Data } from 'effect';
import type { Union } from 'ts-toolbelt';
import type { DisabledSemverGroup } from './disabled';
import type { FilteredOutSemverGroup } from './filtered-out';
import type { IgnoredSemverGroup } from './ignored';
import type { WithRangeSemverGroup } from './with-range';

export namespace SemverGroup {
  export type Disabled = DisabledSemverGroup;
  export type FilteredOut = FilteredOutSemverGroup;
  export type Ignored = IgnoredSemverGroup;
  export type WithRange = WithRangeSemverGroup;

  export type Any = Union.Strict<Disabled | FilteredOut | Ignored | WithRange>;

  export class ConfigError extends Data.TaggedClass('SemverGroupConfigError')<{
    readonly config: unknown;
    readonly error: string;
  }> {}
}
