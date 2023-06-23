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

export function createSemverRangesProgram<T extends SemverRangeEffects<any>>(
  ctx: Ctx,
  effects: T,
): Effect.Effect<Env, SemverGroupConfigError | DeprecatedTypesError, Ctx> {
  return pipe(
    getSemverGroups(ctx),
    Effect.flatMap((semverGroups) =>
      Effect.allPar(
        semverGroups.flatMap((group) =>
          group.inspect().map((reportEffect, index) =>
            pipe(
              unify(reportEffect),
              Effect.flatMap(
                pipe(
                  Match.type<SemverGroupReport.ValidCases>(),
                  Match.tagsExhaustive({
                    FilteredOut(report) {
                      return effects.onFilteredOut({ ctx, group, index, report });
                    },
                    Ignored(report) {
                      return effects.onIgnored({ ctx, group, index, report });
                    },
                    Valid(report) {
                      return effects.onValid({ ctx, group, index, report });
                    },
                  }),
                ),
              ),
              Effect.catchTags({
                SemverRangeMismatch(report) {
                  return effects.onSemverRangeMismatch({ ctx, group, index, report });
                },
                UnsupportedVersion(report) {
                  return effects.onUnsupportedVersion({ ctx, group, index, report });
                },
                WorkspaceSemverRangeMismatch(report) {
                  return effects.onWorkspaceSemverRangeMismatch({ ctx, group, index, report });
                },
              }),
            ),
          ),
        ),
      ),
    ),
    Effect.flatMap((results) => effects.onComplete(ctx, results)),
    Effect.map(() => ctx),
  );
}
