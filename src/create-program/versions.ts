import { pipe } from '@effect/data/Function';
import { unify } from '@effect/data/Unify';
import * as Effect from '@effect/io/Effect';
import * as Match from '@effect/match';
import type { DeprecatedTypesError } from '../config/get-enabled-types';
import type { Env } from '../env/create-env';
import type { Ctx } from '../get-context';
import type { VersionGroupConfigError, VersionGroupReport } from '../get-version-groups';
import { getVersionGroups } from '../get-version-groups';
import type { VersionEffects } from './effects';

export function createVersionsProgram(
  ctx: Ctx,
  effects: VersionEffects,
): Effect.Effect<Env, VersionGroupConfigError | DeprecatedTypesError, Ctx> {
  return pipe(
    getVersionGroups(ctx),
    Effect.flatMap((versionGroups) =>
      Effect.all(
        versionGroups.map((group) =>
          Effect.all(
            group.inspect().map((reportEffect, index) =>
              pipe(
                unify(reportEffect),
                Effect.flatMap(
                  pipe(
                    Match.type<VersionGroupReport.ValidCases>(),
                    Match.tagsExhaustive({
                      FilteredOut: (report) =>
                        pipe(
                          effects.FilteredOut({ ctx, group, index, report }),
                          Effect.map(() => report as VersionGroupReport.ValidCases),
                        ),
                      Ignored: (report) =>
                        pipe(
                          effects.Ignored({ ctx, group, index, report }),
                          Effect.map(() => report as VersionGroupReport.ValidCases),
                        ),
                      Valid: (report) =>
                        pipe(
                          effects.Valid({ ctx, group, index, report }),
                          Effect.map(() => report as VersionGroupReport.ValidCases),
                        ),
                    }),
                  ),
                ),
                Effect.catchTags({
                  Banned: (report) =>
                    pipe(
                      effects.Banned({ ctx, group, index, report }),
                      Effect.map(() => report),
                    ),
                  HighestSemverMismatch: (report) =>
                    pipe(
                      effects.HighestSemverMismatch({ ctx, group, index, report }),
                      Effect.map(() => report),
                    ),
                  LowestSemverMismatch: (report) =>
                    pipe(
                      effects.LowestSemverMismatch({ ctx, group, index, report }),
                      Effect.map(() => report),
                    ),
                  PinnedMismatch: (report) =>
                    pipe(
                      effects.PinnedMismatch({ ctx, group, index, report }),
                      Effect.map(() => report),
                    ),
                  SameRangeMismatch: (report) =>
                    pipe(
                      effects.SameRangeMismatch({ ctx, group, index, report }),
                      Effect.map(() => report),
                    ),
                  SnappedToMismatch: (report) =>
                    pipe(
                      effects.SnappedToMismatch({ ctx, group, index, report }),
                      Effect.map(() => report),
                    ),
                  UnsupportedMismatch: (report) =>
                    pipe(
                      effects.UnsupportedMismatch({ ctx, group, index, report }),
                      Effect.map(() => report),
                    ),
                  WorkspaceMismatch: (report) =>
                    pipe(
                      effects.WorkspaceMismatch({ ctx, group, index, report }),
                      Effect.map(() => report),
                    ),
                }),
              ),
            ),
          ),
        ),
      ),
    ),
    Effect.flatMap(() => effects.TearDown(ctx)),
    Effect.map(() => ctx),
  );
}
