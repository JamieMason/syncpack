import type * as Effect from '@effect/io/Effect';
import type { Env } from '../env/create-env';
import type { Ctx } from '../get-context';
import type { AnySemverGroup, SemverGroupReport as SR } from '../get-semver-groups';
import type { AnyVersionGroup, VersionGroupReport as VR } from '../get-version-groups';

export interface SemverRangeEffectInput<T> {
  ctx: Ctx;
  group: AnySemverGroup;
  index: number;
  report: T;
}

export interface VersionEffectInput<T> {
  ctx: Ctx;
  group: AnyVersionGroup;
  index: number;
  report: T;
}

export interface SemverRangeEffects<A> {
  onFilteredOut: (
    input: SemverRangeEffectInput<SR.FilteredOut>,
  ) => Effect.Effect<Env | never, never, A>;
  onIgnored: (input: SemverRangeEffectInput<SR.Ignored>) => Effect.Effect<Env | never, never, A>;
  onSemverRangeMismatch: (
    input: SemverRangeEffectInput<SR.SemverRangeMismatch>,
  ) => Effect.Effect<Env | never, never, A>;
  onUnsupportedVersion: (
    input: SemverRangeEffectInput<SR.UnsupportedVersion>,
  ) => Effect.Effect<Env | never, never, A>;
  onValid: (input: SemverRangeEffectInput<SR.Valid>) => Effect.Effect<Env | never, never, A>;
  onWorkspaceSemverRangeMismatch: (
    input: SemverRangeEffectInput<SR.WorkspaceSemverRangeMismatch>,
  ) => Effect.Effect<Env | never, never, A>;
  onComplete: (ctx: Ctx, results: A[]) => Effect.Effect<Env | never, never, A>;
}

export interface VersionEffects<A> {
  onBanned: (input: VersionEffectInput<VR.Banned>) => Effect.Effect<Env | never, never, A>;
  onFilteredOut: (
    input: VersionEffectInput<VR.FilteredOut>,
  ) => Effect.Effect<Env | never, never, A>;
  onHighestSemverMismatch: (
    input: VersionEffectInput<VR.HighestSemverMismatch>,
  ) => Effect.Effect<Env | never, never, A>;
  onIgnored: (input: VersionEffectInput<VR.Ignored>) => Effect.Effect<Env | never, never, A>;
  onLowestSemverMismatch: (
    input: VersionEffectInput<VR.LowestSemverMismatch>,
  ) => Effect.Effect<Env | never, never, A>;
  onPinnedMismatch: (
    input: VersionEffectInput<VR.PinnedMismatch>,
  ) => Effect.Effect<Env | never, never, A>;
  onSameRangeMismatch: (
    input: VersionEffectInput<VR.SameRangeMismatch>,
  ) => Effect.Effect<Env | never, never, A>;
  onSnappedToMismatch: (
    input: VersionEffectInput<VR.SnappedToMismatch>,
  ) => Effect.Effect<Env | never, never, A>;
  onUnsupportedMismatch: (
    input: VersionEffectInput<VR.UnsupportedMismatch>,
  ) => Effect.Effect<Env | never, never, A>;
  onValid: (input: VersionEffectInput<VR.Valid>) => Effect.Effect<Env | never, never, A>;
  onWorkspaceMismatch: (
    input: VersionEffectInput<VR.WorkspaceMismatch>,
  ) => Effect.Effect<Env | never, never, A>;
  onComplete: (ctx: Ctx, results: A[]) => Effect.Effect<Env | never, never, A>;
}
