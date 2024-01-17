import { Data } from 'effect';
import type { DisabledSemverGroup } from './disabled.js';
import type { FilteredOutSemverGroup } from './filtered-out.js';
import type { IgnoredSemverGroup } from './ignored.js';
import type { WithRangeSemverGroup } from './with-range.js';

export namespace SemverGroup {
  export type Disabled = DisabledSemverGroup;
  export type FilteredOut = FilteredOutSemverGroup;
  export type Ignored = IgnoredSemverGroup;
  export type WithRange = WithRangeSemverGroup;

  export type Any = Disabled | FilteredOut | Ignored | WithRange;

  export class ConfigError extends Data.TaggedClass('SemverGroupConfigError')<{
    readonly config: unknown;
    readonly error: string;
  }> {}
}
