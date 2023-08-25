import chalk from 'chalk';
import { Context, Effect, pipe } from 'effect';
import { uniq } from 'tightrope/array/uniq';
import { isString } from 'tightrope/guard/is-string';
import { logOtherCommands } from '../bin-list/list';
import { CliConfigTag } from '../config/tag';
import { type CliConfig } from '../config/types';
import { ICON } from '../constants';
import type { ErrorHandlers } from '../error-handlers/default-error-handlers';
import { chainErrorHandlers, defaultErrorHandlers } from '../error-handlers/default-error-handlers';
import { getContext } from '../get-context';
import { getInstances } from '../get-instances';
import type { Io } from '../io';
import { IoTag } from '../io';
import { askForChoice } from '../io/ask-for-choice';
import { askForInput } from '../io/ask-for-input';
import { exitIfInvalid } from '../io/exit-if-invalid';
import { writeIfChanged } from '../io/write-if-changed';
import { getVersionGroupHeader } from '../lib/get-group-header';
import { withLogger } from '../lib/with-logger';
import type { Report } from '../report';

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
        Effect.catchTags(chainErrorHandlers(ctx, errorHandlers)),
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
        groupReport.reports.map(
          (instanceReport) =>
            instanceReport.specifier?.raw ||
            instanceReport.fixable?.raw ||
            instanceReport.unfixable?.rawSpecifier,
        ),
      ).filter(isString);

      const OTHER = chalk.dim('Other');
      const SKIP = chalk.dim('Skip');
      const QUIT = chalk.dim('Quit');

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
