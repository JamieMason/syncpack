import { pipe } from '@effect/data/Function';
import { unify } from '@effect/data/Unify';
import * as Effect from '@effect/io/Effect';
import * as Match from '@effect/match';
import type { DeprecatedTypesError } from '../config/get-enabled-types';
import type { Env } from '../env/create-env';
import type { Ctx } from '../get-context';
import type { SemverGroupConfigError, SemverGroupReport } from '../get-semver-groups';
import { getSemverGroups } from '../get-semver-groups';
import type { SemverRangeEffects } from './effects';

export function createSemverRangesProgram(
  ctx: Ctx,
  effects: SemverRangeEffects,
): Effect.Effect<Env, SemverGroupConfigError | DeprecatedTypesError, Ctx> {
  return pipe(
    getSemverGroups(ctx),
    Effect.flatMap((semverGroups) =>
      Effect.all(
        semverGroups.map((group) =>
          Effect.all(
            group.inspect().map((reportEffect, index) =>
              pipe(
                unify(reportEffect),
                Effect.flatMap(
                  pipe(
                    Match.type<SemverGroupReport.ValidCases>(),
                    Match.tagsExhaustive({
                      FilteredOut: (report) =>
                        pipe(
                          effects.FilteredOut({ ctx, group, index, report }),
                          Effect.map(() => report as SemverGroupReport.ValidCases),
                        ),
                      Ignored: (report) =>
                        pipe(
                          effects.Ignored({ ctx, group, index, report }),
                          Effect.map(() => report as SemverGroupReport.ValidCases),
                        ),
                      Valid: (report) =>
                        pipe(
                          effects.Valid({ ctx, group, index, report }),
                          Effect.map(() => report as SemverGroupReport.ValidCases),
                        ),
                    }),
                  ),
                ),
                Effect.catchTags({
                  SemverRangeMismatch: (report) =>
                    pipe(
                      effects.SemverRangeMismatch({ ctx, group, index, report }),
                      Effect.map(() => report),
                    ),
                  UnsupportedVersion: (report) =>
                    pipe(
                      effects.UnsupportedVersion({ ctx, group, index, report }),
                      Effect.map(() => report),
                    ),
                  WorkspaceSemverRangeMismatch: (report) =>
                    pipe(
                      effects.WorkspaceSemverRangeMismatch({ ctx, group, index, report }),
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
