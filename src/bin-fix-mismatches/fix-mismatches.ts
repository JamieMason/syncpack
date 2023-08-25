import chalk from 'chalk';
import { Context, Effect, pipe } from 'effect';
import { isObject } from 'tightrope/guard/is-object';
import { isUndefined } from 'tightrope/guard/is-undefined';
import { logIgnoredSize } from '../bin-lint-semver-ranges/lint-semver-ranges';
import {
  logMissingLocalVersion,
  logMissingSnappedToMismatch,
  logSameRangeMismatch,
  logUnsupportedMismatch,
} from '../bin-list-mismatches/list-mismatches';
import { CliConfigTag } from '../config/tag';
import { type CliConfig } from '../config/types';
import { ICON } from '../constants';
import type { ErrorHandlers } from '../error-handlers/default-error-handlers';
import { chainErrorHandlers, defaultErrorHandlers } from '../error-handlers/default-error-handlers';
import type { Ctx } from '../get-context';
import { getContext } from '../get-context';
import { getInstances } from '../get-instances';
import type { Io } from '../io';
import { IoTag } from '../io';
import { exitIfInvalid } from '../io/exit-if-invalid';
import { writeIfChanged } from '../io/write-if-changed';
import { getVersionGroupHeader } from '../lib/get-group-header';
import { padStart } from '../lib/pad-start';
import { withLogger } from '../lib/with-logger';
import type { Report } from '../report';
import { DELETE } from '../version-group/lib/delete';

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
                const _tag = report._tag;
                if (_tag === 'Valid') {
                  validCount++;
                } else if (
                  _tag === 'Banned' ||
                  _tag === 'HighestSemverMismatch' ||
                  _tag === 'LocalPackageMismatch' ||
                  _tag === 'LowestSemverMismatch' ||
                  _tag === 'PinnedMismatch' ||
                  _tag === 'SemverRangeMismatch' ||
                  _tag === 'SnappedToMismatch'
                ) {
                  fixedCount++;
                  yield* $(fixMismatch(report));
                } else if (_tag === 'MissingLocalVersion') {
                  ctx.isInvalid = true;
                  unfixableCount++;
                  yield* $(logMissingLocalVersion(report));
                } else if (_tag === 'MissingSnappedToMismatch') {
                  ctx.isInvalid = true;
                  unfixableCount++;
                  yield* $(logMissingSnappedToMismatch(report));
                } else if (_tag === 'UnsupportedMismatch') {
                  ctx.isInvalid = true;
                  unfixableCount++;
                  yield* $(logUnsupportedMismatch(report));
                } else if (_tag === 'SameRangeMismatch') {
                  ctx.isInvalid = true;
                  unfixableCount++;
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
        Effect.catchTags(chainErrorHandlers(ctx, errorHandlers)),
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
        if (isObject(value) && Object.values(value).every(isUndefined)) {
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
