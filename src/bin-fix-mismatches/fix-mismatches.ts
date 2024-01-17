import chalk from 'chalk-template';
import { Context, Effect, flow, pipe } from 'effect';
import { isObject } from 'tightrope/guard/is-object.js';
import { isUndefined } from 'tightrope/guard/is-undefined.js';
import { logIgnoredSize } from '../bin-lint-semver-ranges/lint-semver-ranges.js';
import {
  logMissingLocalVersion,
  logMissingSnappedToMismatch,
  logSameRangeMismatch,
  logUnsupportedMismatch,
} from '../bin-list-mismatches/list-mismatches.js';
import { CliConfigTag } from '../config/tag.js';
import { type CliConfig } from '../config/types.js';
import { ICON } from '../constants.js';
import type { ErrorHandlers } from '../error-handlers/default-error-handlers.js';
import { defaultErrorHandlers } from '../error-handlers/default-error-handlers.js';
import type { Ctx } from '../get-context/index.js';
import { getContext } from '../get-context/index.js';
import { getInstances } from '../get-instances/index.js';
import { exitIfInvalid } from '../io/exit-if-invalid.js';
import type { Io } from '../io/index.js';
import { IoTag } from '../io/index.js';
import { writeIfChanged } from '../io/write-if-changed.js';
import { getVersionGroupHeader } from '../lib/get-group-header.js';
import { padStart } from '../lib/pad-start.js';
import { withLogger } from '../lib/with-logger.js';
import type { Report } from '../report.js';
import { DELETE } from '../version-group/lib/delete.js';

interface Input {
  io: Io;
  cli: Partial<CliConfig>;
  errorHandlers?: ErrorHandlers;
}

export function fixMismatches({ io, cli, errorHandlers = defaultErrorHandlers }: Input) {
  return pipe(
    getContext({ io, cli, errorHandlers }),
    Effect.flatMap((ctx) =>
      pipe(
        Effect.gen(function* ($) {
          const { versionGroups } = yield* $(getInstances(ctx, io, errorHandlers));
          let index = 0;
          for (const group of versionGroups) {
            const groupSize = group.instances.length;
            let fixedCount = 0;
            let unfixableCount = 0;
            let validCount = 0;

            if (group._tag === 'FilteredOut') {
              index++;
              continue;
            }

            yield* $(Effect.logInfo(getVersionGroupHeader({ group, index })));

            if (group._tag === 'Ignored') {
              yield* $(logIgnoredSize(groupSize));
              index++;
              continue;
            }

            for (const groupReport of yield* $(group.inspectAll())) {
              for (const report of groupReport.reports) {
                if (report._tagGroup === 'Valid') {
                  validCount++;
                } else if (report._tagGroup === 'Fixable') {
                  fixedCount++;
                  yield* $(fixMismatch(report));
                } else if (report._tagGroup === 'Unfixable') {
                  ctx.isInvalid = true;
                  unfixableCount++;
                }

                if (report._tag === 'MissingLocalVersion') {
                  yield* $(logMissingLocalVersion(report));
                } else if (report._tag === 'MissingSnappedToMismatch') {
                  yield* $(logMissingSnappedToMismatch(report));
                } else if (report._tag === 'UnsupportedMismatch') {
                  yield* $(logUnsupportedMismatch(report));
                } else if (report._tag === 'SameRangeMismatch') {
                  yield* $(logSameRangeMismatch(report));
                }
              }
            }

            if (validCount) yield* $(logAlreadyValidSize(validCount));
            if (fixedCount) yield* $(logFixedSize(fixedCount));
            if (unfixableCount) yield* $(logUnfixableSize(unfixableCount));

            index++;
          }

          yield* $(removeEmptyObjects(ctx));

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

export function fixMismatch(report: Report.Version.Fixable.Any) {
  return report.fixable.instance.write(report._tag === 'Banned' ? DELETE : report.fixable.raw);
}

/** Remove empty objects such as `{"dependencies": {}}` left after deleting */
function removeEmptyObjects(ctx: Ctx) {
  return Effect.sync(() => {
    ctx.packageJsonFiles.forEach((file) => {
      const contents = file.jsonFile.contents;
      Object.keys(contents).forEach((key) => {
        const value = contents[key];
        if (isObject(value) && value && Object.values(value).every(isUndefined)) {
          delete contents[key];
        }
      });
    });
  });
}

export function logAlreadyValidSize(amount: number) {
  const msg = chalk`${padStart(amount)} {green ${ICON.tick}} already valid`;
  return Effect.logInfo(msg);
}

export function logFixedSize(amount: number) {
  const msg = chalk`${padStart(amount)} {green ${ICON.tick}} fixed`;
  return Effect.logInfo(msg);
}

export function logUnfixableSize(amount: number) {
  const msg = chalk`{red ${padStart(amount)} ${
    ICON.panic
  } can be fixed manually using} {blue syncpack prompt}`;
  return Effect.logInfo(msg);
}
