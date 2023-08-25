import type { Effect } from 'effect';
import type { Ctx } from '../get-context';
import type { Io } from '../io';
import type { Report } from '../report';
import type { SemverGroup } from '../semver-group';
import type { VersionGroup } from '../version-group';

export interface SemverRangeEffectInput<T extends Report.Semver.Any> {
  _tag: T['_tag'];
  ctx: Ctx;
  group: SemverGroup.Any;
  index: number;
  report: T;
}

export interface VersionEffectInput<T extends Report.Version.Any> {
  _tag: T['_tag'];
  ctx: Ctx;
  group: VersionGroup.Any;
  index: number;
  report: T;
}

export type Handler<I> = (input: I) => Effect.Effect<Io | never, never, void>;
type SemverHandler<I extends Report.Semver.Any> = Handler<SemverRangeEffectInput<I>>;
type VersionHandler<I extends Report.Version.Any> = Handler<VersionEffectInput<I>>;

export interface SemverRangeEffects {
  // Misc
  onSemverGroup: Handler<{
    ctx: Ctx;
    group: SemverGroup.Any;
    index: number;
  }>;
  // Valid Instances
  onDisabled: SemverHandler<Report.Disabled>;
  onFilteredOut: SemverHandler<Report.FilteredOut>;
  onIgnored: SemverHandler<Report.Ignored>;
  onValid: SemverHandler<Report.Valid>;
  // Fixable Instances
  onSemverRangeMismatch: SemverHandler<Report.SemverRangeMismatch>;
  // Unfixable Instances
  onUnsupportedMismatch: SemverHandler<Report.UnsupportedMismatch>;
}

export interface VersionEffects {
  // Misc
  onComplete: (ctx: Ctx) => Effect.Effect<never, never, void>;
  onVersionGroup: Handler<{
    ctx: Ctx;
    group: VersionGroup.Any;
    index: number;
  }>;
  // Valid Instances
  onDisabled: VersionHandler<Report.Disabled>;
  onFilteredOut: VersionHandler<Report.FilteredOut>;
  onIgnored: VersionHandler<Report.Ignored>;
  onValid: VersionHandler<Report.Valid>;
  // Fixable Instances
  onFixable: VersionHandler<Report.Version.Fixable.Any>;
  onBanned: VersionHandler<Report.Banned>;
  onHighestSemverMismatch: VersionHandler<Report.HighestSemverMismatch>;
  onLocalPackageMismatch: VersionHandler<Report.LocalPackageMismatch>;
  onLowestSemverMismatch: VersionHandler<Report.LowestSemverMismatch>;
  onPinnedMismatch: VersionHandler<Report.PinnedMismatch>;
  onSemverRangeMismatch: VersionHandler<Report.SemverRangeMismatch>;
  onSnappedToMismatch: VersionHandler<Report.SnappedToMismatch>;
  // Unfixable Instances
  onUnfixable: VersionHandler<Report.Version.Unfixable.Any>;
  onMissingLocalVersion: VersionHandler<Report.MissingLocalVersion>;
  onMissingSnappedToMismatch: VersionHandler<Report.MissingSnappedToMismatch>;
  onUnsupportedMismatch: VersionHandler<Report.UnsupportedMismatch>;
  onSameRangeMismatch: VersionHandler<Report.SameRangeMismatch>;
}
