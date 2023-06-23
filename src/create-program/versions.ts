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

export function createVersionsProgram<T extends VersionEffects<any>>(
  ctx: Ctx,
  effects: T,
): Effect.Effect<Env, VersionGroupConfigError | DeprecatedTypesError, Ctx> {
  return pipe(
    getVersionGroups(ctx),
    Effect.flatMap((versionGroups) =>
      Effect.allPar(
        versionGroups.flatMap((group) =>
          group.inspect().map((reportEffect, index) =>
            pipe(
              unify(reportEffect),
              Effect.flatMap(
                pipe(
                  Match.type<VersionGroupReport.ValidCases>(),
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
                Banned(report) {
                  return effects.onBanned({ ctx, group, index, report });
                },
                HighestSemverMismatch(report) {
                  return effects.onHighestSemverMismatch({ ctx, group, index, report });
                },
                LowestSemverMismatch(report) {
                  return effects.onLowestSemverMismatch({ ctx, group, index, report });
                },
                PinnedMismatch(report) {
                  return effects.onPinnedMismatch({ ctx, group, index, report });
                },
                SameRangeMismatch(report) {
                  return effects.onSameRangeMismatch({ ctx, group, index, report });
                },
                SnappedToMismatch(report) {
                  return effects.onSnappedToMismatch({ ctx, group, index, report });
                },
                UnsupportedMismatch(report) {
                  return effects.onUnsupportedMismatch({ ctx, group, index, report });
                },
                WorkspaceMismatch(report) {
                  return effects.onWorkspaceMismatch({ ctx, group, index, report });
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
