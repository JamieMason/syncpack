import chalk from 'chalk-template';
import { Context, Effect, pipe } from 'effect';
import { EOL } from 'os';
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
import { getVersionGroupHeader } from '../lib/get-group-header.js';
import { padStart } from '../lib/pad-start.js';
import { withLogger } from '../lib/with-logger.js';
import type { Report } from '../report.js';

interface Input {
  io: Io;
  cli: Partial<CliConfig>;
  errorHandlers?: ErrorHandlers;
}

export function listMismatches({ io, cli, errorHandlers = defaultErrorHandlers }: Input) {
  return pipe(
    getContext({ io, cli, errorHandlers }),
    Effect.flatMap((ctx) => pipeline(ctx, io, errorHandlers)),
    Effect.flatMap(exitIfInvalid),
    Effect.provide(pipe(Context.empty(), Context.add(CliConfigTag, cli), Context.add(IoTag, io))),
    withLogger,
  );
}

/** Exported to be reused by `syncpack lint` */
export function pipeline(
  ctx: Ctx,
  io: Io,
  errorHandlers: ErrorHandlers,
): Effect.Effect<never, never, Ctx> {
  return Effect.gen(function* ($) {
    const { versionGroups } = yield* $(getInstances(ctx, io, errorHandlers));
    let index = 0;

    for (const group of versionGroups) {
      const countByReportGroup: Record<Report.Any['_tagGroup'], number> = {
        Excluded: 0,
        Fixable: 0,
        Unfixable: 0,
        Valid: 0,
      };

      yield* $(Effect.logInfo(getVersionGroupHeader({ group, index })));

      for (const groupReport of yield* $(group.inspectAll())) {
        for (const report of groupReport.reports) {
          countByReportGroup[report._tagGroup]++;

          if (report.isInvalid) ctx.isInvalid = true;

          const logReport = onReportTag[report._tag];
          if (logReport) yield* $(logReport(report));
        }
      }

      yield* $(onReportGroup.Valid(countByReportGroup.Valid));
      yield* $(onReportGroup.Fixable(countByReportGroup.Fixable));
      yield* $(onReportGroup.Unfixable(countByReportGroup.Unfixable));
      yield* $(onReportGroup.Excluded(countByReportGroup.Excluded));

      index++;
    }
    return ctx;
  });
}

const onReportGroup: Record<
  Report.Any['_tagGroup'],
  (count: number) => Effect.Effect<never, never, void>
> = {
  Excluded(amount: number) {
    if (amount === 0) return Effect.unit;
    const msg = chalk`{gray ${padStart(amount)} ${ICON.rightArrow} ignored}`;
    return Effect.logInfo(msg);
  },
  Fixable(amount: number) {
    if (amount === 0) return Effect.unit;
    const msg = chalk`${padStart(amount)} {green ${ICON.tick}} can be auto-fixed`;
    return Effect.logInfo(msg);
  },
  Unfixable(amount: number) {
    if (amount === 0) return Effect.unit;
    const msg = chalk`{red ${padStart(amount)} ${
      ICON.panic
    } can be fixed manually using} {blue syncpack prompt}`;
    return Effect.logInfo(msg);
  },
  Valid(amount: number) {
    if (amount === 0) return Effect.unit;
    const msg = chalk`${padStart(amount)} {green ${ICON.tick}} already valid`;
    return Effect.logInfo(msg);
  },
};

const onReportTag: Record<Report.Any['_tag'], (report: any) => Effect.Effect<never, never, void>> =
  {
    Banned(report: Report.Banned) {
      const _tag = report._tag;
      const instance = report.fixable.instance;
      const name = instance.name;
      const jsonFile = instance.packageJsonFile.jsonFile;
      const path = instance.strategy.path;
      const shortPath = jsonFile.shortPath;

      return Effect.logInfo(
        chalk`{red ${ICON.cross}} ${name} {red banned} {gray ${shortPath} > ${path}} {blue [${_tag}]}`,
      );
    },
    Disabled(_report: Report.Disabled) {
      return Effect.unit;
    },
    FilteredOut(_report: Report.FilteredOut) {
      return Effect.unit;
    },
    HighestSemverMismatch(report: Report.HighestSemverMismatch) {
      const _tag = report._tag;
      const fixable = report.fixable;
      const instance = fixable.instance;
      const jsonFile = instance.packageJsonFile.jsonFile;
      const actual = instance.rawSpecifier.raw;
      const expected = fixable.raw;
      const name = instance.name;
      const path = instance.strategy.path;
      const shortPath = jsonFile.shortPath;

      return Effect.logInfo(
        chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {green ${expected}} {gray ${shortPath} > ${path}} {blue [${_tag}]}`,
      );
    },
    Ignored(_report: Report.Ignored) {
      return Effect.unit;
    },
    LocalPackageMismatch(report: Report.LocalPackageMismatch) {
      const _tag = report._tag;
      const fixable = report.fixable;
      const instance = fixable.instance;
      const actual = instance.rawSpecifier.raw;
      const expected = fixable.raw;
      const name = instance.name;
      const path = instance.strategy.path;
      const shortPath = instance.packageJsonFile.jsonFile.shortPath;

      return Effect.logInfo(
        chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {green ${expected}} {gray ${shortPath} > ${path}} {blue [${_tag}]}`,
      );
    },
    LowestSemverMismatch(report: Report.LowestSemverMismatch) {
      const _tag = report._tag;
      const fixable = report.fixable;
      const instance = fixable.instance;
      const actual = instance.rawSpecifier.raw;
      const expected = fixable.raw;
      const name = instance.name;
      const path = instance.strategy.path;
      const shortPath = instance.packageJsonFile.jsonFile.shortPath;

      return Effect.logInfo(
        chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {green ${expected}} {gray ${shortPath} > ${path}} {blue [${_tag}]}`,
      );
    },
    MissingLocalVersion(report: Report.MissingLocalVersion) {
      const instance = report.unfixable;
      const localPath = report.localInstance.packageJsonFile.jsonFile.shortPath;
      const jsonFile = instance.packageJsonFile.jsonFile;
      const actual = instance.rawSpecifier.raw;
      const name = instance.name;
      const path = instance.strategy.path;
      const shortPath = jsonFile.shortPath;

      return Effect.logInfo(
        [
          chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {red ???} {gray ${shortPath} > ${path}} {blue [MissingLocalVersion]}`,
          chalk`  {red ${localPath} does not have a .version property which is exact semver}`,
        ].join(EOL),
      );
    },
    MissingSnappedToMismatch(report: Report.MissingSnappedToMismatch) {
      const instance = report.unfixable;
      const jsonFile = instance.packageJsonFile.jsonFile;
      const actual = instance.rawSpecifier.raw;
      const name = instance.name;
      const path = instance.strategy.path;
      const shortPath = jsonFile.shortPath;

      return Effect.logInfo(
        [
          chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {red ???} {gray ${shortPath} > ${path}} {blue [MissingSnappedToMismatch]}`,
          chalk`  {red no package in this groups .snapTo array depend on ${name}}`,
        ].join(EOL),
      );
    },
    PinnedMismatch(report: Report.PinnedMismatch) {
      const _tag = report._tag;
      const fixable = report.fixable;
      const instance = fixable.instance;
      const actual = instance.rawSpecifier.raw;
      const expected = fixable.raw;
      const name = instance.name;
      const path = instance.strategy.path;
      const shortPath = instance.packageJsonFile.jsonFile.shortPath;

      return Effect.logInfo(
        chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {green ${expected}} {gray ${shortPath} > ${path}} {blue [${_tag}]}`,
      );
    },
    SameRangeMismatch(report: Report.SameRangeMismatch) {
      const instance = report.unfixable;
      const jsonFile = instance.packageJsonFile.jsonFile;
      const actual = instance.rawSpecifier.raw;
      const name = instance.name;
      const path = instance.strategy.path;
      const shortPath = jsonFile.shortPath;
      const mismatches = report.mismatches;

      return Effect.logInfo(
        [
          chalk`{red ${ICON.cross}} ${name} {red range ${actual} does not include ${mismatches.join(
            ', ',
          )}} {gray ${shortPath} > ${path}} {blue [SameRangeMismatch]}`,
          chalk`  {gray use {blue syncpack prompt} to fix this manually}`,
        ].join(EOL),
      );
    },
    SemverRangeMismatch(report: Report.SemverRangeMismatch) {
      const _tag = report._tag;
      const fixable = report.fixable;
      const instance = fixable.instance;
      const actual = instance.rawSpecifier.raw;
      const expected = fixable.raw;
      const name = instance.name;
      const path = instance.strategy.path;
      const shortPath = instance.packageJsonFile.jsonFile.shortPath;

      return Effect.logInfo(
        chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {green ${expected}} {gray ${shortPath} > ${path}} {blue [${_tag}]}`,
      );
    },
    SnappedToMismatch(report: Report.SnappedToMismatch) {
      const _tag = report._tag;
      const fixable = report.fixable;
      const instance = fixable.instance;
      const actual = instance.rawSpecifier.raw;
      const expected = fixable.raw;
      const name = instance.name;
      const path = instance.strategy.path;
      const shortPath = instance.packageJsonFile.jsonFile.shortPath;

      return Effect.logInfo(
        chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {green ${expected}} {gray ${shortPath} > ${path}} {blue [${_tag}]}`,
      );
    },
    UnsupportedMismatch(report: Report.UnsupportedMismatch) {
      const instance = report.unfixable;
      const jsonFile = instance.packageJsonFile.jsonFile;
      const actual = instance.rawSpecifier.raw;
      const name = instance.name;
      const path = instance.strategy.path;
      const shortPath = jsonFile.shortPath;

      return Effect.logInfo(
        [
          chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {red ???} {gray ${shortPath} > ${path}} {blue [UnsupportedMismatch]}`,
          chalk`  {red use {blue syncpack prompt} to fix this manually}`,
        ].join(EOL),
      );
    },
    Valid(_report: Report.Valid) {
      return Effect.unit;
    },
  };

export const logMissingLocalVersion = onReportTag.MissingLocalVersion;
export const logMissingSnappedToMismatch = onReportTag.MissingSnappedToMismatch;
export const logUnsupportedMismatch = onReportTag.UnsupportedMismatch;
export const logSameRangeMismatch = onReportTag.SameRangeMismatch;
