import type * as Effect from '@effect/io/Effect';
import type { Env } from '../env/create-env';
import type { Ctx } from '../get-context';
import type { AnySemverGroup, SemverGroupReport as SR } from '../get-semver-groups';
import type { AnyVersionGroup, VersionGroupReport as VR } from '../get-version-groups';

type R = Effect.Effect<Env | never, never, void>;

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

export interface SemverRangeEffects {
  FilteredOut: (input: SemverRangeEffectInput<SR.FilteredOut>) => R;
  Ignored: (input: SemverRangeEffectInput<SR.Ignored>) => R;
  SemverRangeMismatch: (input: SemverRangeEffectInput<SR.SemverRangeMismatch>) => R;
  UnsupportedVersion: (input: SemverRangeEffectInput<SR.UnsupportedVersion>) => R;
  Valid: (input: SemverRangeEffectInput<SR.Valid>) => R;
  WorkspaceSemverRangeMismatch: (
    input: SemverRangeEffectInput<SR.WorkspaceSemverRangeMismatch>,
  ) => R;
  TearDown: (ctx: Ctx) => R;
}

export interface VersionEffects {
  Banned: (input: VersionEffectInput<VR.Banned>) => R;
  FilteredOut: (input: VersionEffectInput<VR.FilteredOut>) => R;
  HighestSemverMismatch: (input: VersionEffectInput<VR.HighestSemverMismatch>) => R;
  Ignored: (input: VersionEffectInput<VR.Ignored>) => R;
  LowestSemverMismatch: (input: VersionEffectInput<VR.LowestSemverMismatch>) => R;
  PinnedMismatch: (input: VersionEffectInput<VR.PinnedMismatch>) => R;
  SameRangeMismatch: (input: VersionEffectInput<VR.SameRangeMismatch>) => R;
  SnappedToMismatch: (input: VersionEffectInput<VR.SnappedToMismatch>) => R;
  UnsupportedMismatch: (input: VersionEffectInput<VR.UnsupportedMismatch>) => R;
  Valid: (input: VersionEffectInput<VR.Valid>) => R;
  WorkspaceMismatch: (input: VersionEffectInput<VR.WorkspaceMismatch>) => R;
  TearDown: (ctx: Ctx) => R;
}
