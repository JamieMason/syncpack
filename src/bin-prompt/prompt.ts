import chalk from 'chalk-template';
import { Context, Effect, flow, pipe } from 'effect';
import { uniq } from 'tightrope/array/uniq.js';
import { isString } from 'tightrope/guard/is-string.js';
import { logOtherCommands } from '../bin-list/list.js';
import { CliConfigTag } from '../config/tag.js';
import { type CliConfig } from '../config/types.js';
import { ICON } from '../constants.js';
import type { ErrorHandlers } from '../error-handlers/default-error-handlers.js';
import { defaultErrorHandlers } from '../error-handlers/default-error-handlers.js';
import { getContext } from '../get-context/index.js';
import { getInstances } from '../get-instances/index.js';
import { askForChoice } from '../io/ask-for-choice.js';
import { askForInput } from '../io/ask-for-input.js';
import { exitIfInvalid } from '../io/exit-if-invalid.js';
import type { Io } from '../io/index.js';
import { IoTag } from '../io/index.js';
import { writeIfChanged } from '../io/write-if-changed.js';
import { getVersionGroupHeader } from '../lib/get-group-header.js';
import { withLogger } from '../lib/with-logger.js';
import type { Report } from '../report.js';

interface Input {
  io: Io;
  cli: Partial<CliConfig>;
  errorHandlers?: ErrorHandlers;
}

export function prompt({ io, cli, errorHandlers = defaultErrorHandlers }: Input) {
  return pipe(
    getContext({ io, cli, errorHandlers }),
    Effect.flatMap((ctx) =>
      pipe(
        Effect.gen(function* ($) {
          const { versionGroups } = yield* $(getInstances(ctx, io, errorHandlers));
          let unfixableCount = 0;
          let index = 0;
          for (const group of versionGroups) {
            const unfixable: Report.Version.Unfixable.Any[] = [];
            for (const groupReport of yield* $(group.inspectAll())) {
              for (const report of groupReport.reports) {
                if (isUnfixable(report)) {
                  unfixable.push(report);
                }
              }
              if (unfixable.length) {
                unfixableCount += unfixable.length;
                Effect.logInfo(getVersionGroupHeader({ group, index }));
                yield* $(askForNextVersion(groupReport, unfixable));
              }
            }
            index++;
          }

          if (unfixableCount) {
            yield* $(writeIfChanged(ctx));
          } else {
            const msg = chalk`{green ${ICON.tick}} no issues which syncpack cannot fix automatically`;
            yield* $(Effect.logInfo(msg));
            yield* $(logOtherCommands());
          }

          return ctx;
        }),
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

function isUnfixable(report: Report.Version.Any): report is Report.Version.Unfixable.Any {
  return (
    report._tag === 'MissingLocalVersion' ||
    report._tag === 'MissingSnappedToMismatch' ||
    report._tag === 'SameRangeMismatch' ||
    report._tag === 'UnsupportedMismatch'
  );
}

function askForNextVersion(
  groupReport: Report.Version.Group,
  unfixable: Report.Version.Unfixable.Any[],
) {
  return pipe(
    Effect.gen(function* ($) {
      const choices = uniq(
        groupReport.reports.map((report) =>
          report._tagGroup === 'Fixable'
            ? report.fixable.raw
            : report._tagGroup === 'Unfixable'
              ? report.unfixable.rawSpecifier
              : report._tagGroup === 'Valid'
                ? report.specifier.raw
                : null,
        ),
      ).filter(isString);

      const OTHER = chalk`{dim Other}`;
      const SKIP = chalk`{dim Skip}`;
      const QUIT = chalk`{dim Quit}`;

      // Ask user to choose a version to align on
      const choice = yield* $(
        askForChoice({
          message: groupReport.name,
          choices: [...choices, OTHER, SKIP, QUIT],
        }),
      );

      if (choice === SKIP) return;

      // @TODO: Learn https://www.effect.website/docs/data-types/exit
      if (choice === QUIT) return process.exit(0);

      const nextVersion =
        choice === OTHER
          ? yield* $(
              askForInput({
                message: chalk`${groupReport.name} {dim Enter a replacement version}`,
              }),
            )
          : choice;

      yield* $(
        pipe(
          unfixable,
          Effect.forEach((report) => report.unfixable.write(nextVersion)),
        ),
      );
    }),
    Effect.catchTags({
      AskForChoiceError: Effect.logDebug,
      AskForInputError: Effect.logDebug,
    }),
  );
}
