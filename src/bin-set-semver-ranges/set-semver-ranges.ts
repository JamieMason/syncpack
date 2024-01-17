import { Context, Effect, flow, pipe } from 'effect';
import { isNonEmptyArray } from 'tightrope/guard/is-non-empty-array.js';
import {
  fixMismatch,
  logAlreadyValidSize,
  logFixedSize,
  logUnfixableSize,
} from '../bin-fix-mismatches/fix-mismatches.js';
import { logSemverGroupsDisabledWarning } from '../bin-lint-semver-ranges/lint-semver-ranges.js';
import { logUnsupportedMismatch } from '../bin-list-mismatches/list-mismatches.js';
import { CliConfigTag } from '../config/tag.js';
import { type CliConfig } from '../config/types.js';
import type { ErrorHandlers } from '../error-handlers/default-error-handlers.js';
import { defaultErrorHandlers } from '../error-handlers/default-error-handlers.js';
import { getContext } from '../get-context/index.js';
import { getInstances } from '../get-instances/index.js';
import { exitIfInvalid } from '../io/exit-if-invalid.js';
import type { Io } from '../io/index.js';
import { IoTag } from '../io/index.js';
import { writeIfChanged } from '../io/write-if-changed.js';
import { withLogger } from '../lib/with-logger.js';

interface Input {
  io: Io;
  cli: Partial<CliConfig>;
  errorHandlers?: ErrorHandlers;
}

export function setSemverRanges({ io, cli, errorHandlers = defaultErrorHandlers }: Input) {
  return pipe(
    getContext({ io, cli, errorHandlers }),
    Effect.flatMap((ctx) =>
      pipe(
        Effect.gen(function* ($) {
          // no semver groups have been configured, they are disabled by default
          if (!isNonEmptyArray(ctx.config.rcFile.semverGroups)) {
            ctx.isInvalid = true;
            yield* $(logSemverGroupsDisabledWarning());
            return ctx;
          }

          const { semverGroups } = yield* $(getInstances(ctx, io, errorHandlers));
          let fixedCount = 0;
          let unfixableCount = 0;
          let validCount = 0;

          for (const group of semverGroups) {
            if (group._tag === 'WithRange') {
              for (const instance of group.instances) {
                const report = yield* $(group.inspect(instance));
                const _tag = report._tag;
                if (_tag === 'SemverRangeMismatch') {
                  yield* $(fixMismatch(report));
                  fixedCount++;
                } else if (_tag === 'UnsupportedMismatch') {
                  yield* $(logUnsupportedMismatch(report));
                  unfixableCount++;
                } else {
                  validCount++;
                }
              }
            }
          }

          if (validCount) yield* $(logAlreadyValidSize(validCount));
          if (fixedCount) yield* $(logFixedSize(fixedCount));
          if (unfixableCount) yield* $(logUnfixableSize(unfixableCount));

          return ctx;
        }),
        Effect.flatMap(writeIfChanged),
        Effect.catchTags({
          WriteFileError: flow(
            errorHandlers.WriteFileError,
            Effect.map(() => {
              ctx.isInvalid = true;
              return ctx;
            }),
          ),
        }),
        Effect.flatMap(exitIfInvalid),
      ),
    ),
    Effect.provide(pipe(Context.empty(), Context.add(CliConfigTag, cli), Context.add(IoTag, io))),
    withLogger,
  );
}
