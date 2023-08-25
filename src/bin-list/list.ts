import chalk from 'chalk';
import { Context, Effect, pipe } from 'effect';
import { EOL } from 'os';
import { CliConfigTag } from '../config/tag';
import { type CliConfig } from '../config/types';
import { ICON } from '../constants';
import type { ErrorHandlers } from '../error-handlers/default-error-handlers';
import { defaultErrorHandlers } from '../error-handlers/default-error-handlers';
import { getContext } from '../get-context';
import { getInstances } from '../get-instances';
import type { Io } from '../io';
import { IoTag } from '../io';
import { exitIfInvalid } from '../io/exit-if-invalid';
import { getVersionGroupHeader } from '../lib/get-group-header';
import { padStart } from '../lib/pad-start';
import { withLogger } from '../lib/with-logger';
import type { Report } from '../report';

interface Input {
  io: Io;
  cli: Partial<CliConfig>;
  errorHandlers?: ErrorHandlers;
}

export function list({ io, cli, errorHandlers = defaultErrorHandlers }: Input) {
  return pipe(
    getContext({ io, cli, errorHandlers }),
    Effect.flatMap((ctx) =>
      Effect.gen(function* ($) {
        const { versionGroups } = yield* $(getInstances(ctx, io, errorHandlers));
        let index = 0;
        for (const group of versionGroups) {
          const groupSize = group.instances.length;

          if (group._tag === 'FilteredOut') {
            index++;
            continue;
          }

          yield* $(Effect.logInfo(getVersionGroupHeader({ group, index })));

          if (group._tag === 'Ignored') {
            const usages = `${padStart(groupSize)}x`;
            const msg = chalk`{gray ${usages} ignored}`;
            yield* $(Effect.logInfo(msg));
            index++;
            continue;
          }

          if (group._tag === 'Banned') {
            for (const groupReport of yield* $(group.inspectAll())) {
              const name = groupReport.name;
              const usages = `${padStart(groupReport.reports.length)}x`;
              const invalidLabel = chalk`{gray ${usages}} {red ${name}}{gray :}`;
              const msg = chalk`${invalidLabel} {red banned}`;
              yield* $(Effect.logInfo(msg));
            }
            ctx.isInvalid = true;
            index++;
            continue;
          }

          for (const groupReport of yield* $(group.inspectAll())) {
            const matches = new Set<string>();
            const mismatches = new Set<string>();

            for (const report of groupReport.reports) {
              const _tag = report._tag;
              if (_tag === 'Valid') {
                const actual = report.specifier.raw;
                matches.add(
                  report.specifier._tag === 'UnsupportedSpecifier'
                    ? chalk`{gray ${actual}} {gray.dim [UnsupportedSpecifier]}`
                    : chalk`{gray ${actual}}`,
                );
              } else if (
                _tag === 'HighestSemverMismatch' ||
                _tag === 'LocalPackageMismatch' ||
                _tag === 'LowestSemverMismatch' ||
                _tag === 'PinnedMismatch' ||
                _tag === 'SemverRangeMismatch' ||
                _tag === 'SnappedToMismatch'
              ) {
                mismatches.add(getLogForFixable(report));
                ctx.isInvalid = true;
              } else if (
                _tag === 'MissingLocalVersion' ||
                _tag === 'MissingSnappedToMismatch' ||
                _tag === 'UnsupportedMismatch' ||
                _tag === 'SameRangeMismatch'
              ) {
                mismatches.add(getLogForUnfixable(report));
                ctx.isInvalid = true;
              }
            }

            if (mismatches.size === 0) {
              yield* $(logMatchingReport(groupReport, matches));
            } else {
              yield* $(logMismatchingReport(groupReport, mismatches));
            }
          }
          index++;
        }

        yield* $(logOtherCommands());
        return ctx;
      }),
    ),
    Effect.flatMap(exitIfInvalid),
    Effect.provide(pipe(Context.empty(), Context.add(CliConfigTag, cli), Context.add(IoTag, io))),
    withLogger,
  );
}

function logMatchingReport(groupReport: Report.Version.Group, messages: Set<string>) {
  const name = groupReport.name;
  const usages = `${padStart(groupReport.reports.length)}x`;
  const label = chalk`{gray ${usages}} ${name}{gray :}`;
  return Effect.logInfo(chalk`${label} ${[...messages].join(chalk`{gray , }`)}`);
}

function logMismatchingReport(groupReport: Report.Version.Group, messages: Set<string>) {
  const name = groupReport.name;
  const usages = `${padStart(groupReport.reports.length)}x`;
  const label = chalk`{gray ${usages}} {red ${name}}{gray :}`;
  const indent = usages.replace(/./g, ' ');
  return Effect.logInfo(
    chalk`${label}${['', ...messages].join(chalk`${EOL}${indent} {red ${ICON.cross}} `)}`,
  );
}

function getLogForFixable(report: Report.Version.Fixable.Any) {
  const _tag = report._tag;
  const actual = report.fixable.instance.rawSpecifier;
  const expected = report.fixable.raw;
  return chalk`{red ${actual}} {gray ${ICON.rightArrow}} {green ${expected}} {gray.dim [${_tag}]}`;
}

function getLogForUnfixable(report: Report.Version.Unfixable.Any) {
  const _tag = report._tag;
  const actual = report.unfixable.rawSpecifier;
  return chalk`{red ${actual}} {gray ${ICON.rightArrow}} {gray.dim [${_tag}]}`;
}

export function logOtherCommands() {
  return Effect.logInfo(
    [
      '',
      '  What next?',
      chalk`{dim -} {yellow syncpack list-mismatches} to see more detail about mismatching versions`,
      chalk`{dim -} {yellow syncpack fix-mismatches} to fix version mismatches automatically`,
      chalk`{dim -} {yellow syncpack format} to sort and prettify your package.json files`,
      chalk`{dim -} {yellow syncpack update} to choose updates from the npm registry`,
      chalk`{dim -} {yellow syncpack --help} for everything else`,
      '',
    ].join(EOL),
  );
}
