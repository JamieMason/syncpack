import chalk from 'chalk';
import { Context, Effect, pipe } from 'effect';
import { EOL } from 'os';
import { CliConfigTag } from '../config/tag';
import { type CliConfig } from '../config/types';
import { ICON } from '../constants';
import type { ErrorHandlers } from '../error-handlers/default-error-handlers';
import { defaultErrorHandlers } from '../error-handlers/default-error-handlers';
import type { Ctx } from '../get-context';
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
      const groupSize = group.instances.length;
      let fixableCount = 0;
      let unfixableCount = 0;
      let validCount = 0;

      if (group._tag === 'FilteredOut') {
        index++;
        continue;
      }

      yield* $(Effect.logInfo(getVersionGroupHeader({ group, index })));

      if (group._tag === 'Ignored') {
        const msg = chalk`{gray ${padStart(groupSize)} ${ICON.rightArrow} ignored}`;
        yield* $(Effect.logInfo(msg));
        index++;
        continue;
      }

      for (const groupReport of yield* $(group.inspectAll())) {
        for (const report of groupReport.reports) {
          const _tag = report._tag;
          if (_tag === 'Valid') {
            validCount++;
          } else if (_tag === 'Banned') {
            ctx.isInvalid = true;
            yield* $(logBanned(report));
            fixableCount++;
          } else if (_tag === 'HighestSemverMismatch') {
            ctx.isInvalid = true;
            yield* $(logHighestSemverMismatch(report));
            fixableCount++;
          } else if (_tag === 'LocalPackageMismatch') {
            ctx.isInvalid = true;
            yield* $(logLocalPackageMismatch(report));
            fixableCount++;
          } else if (_tag === 'LowestSemverMismatch') {
            ctx.isInvalid = true;
            yield* $(logLowestSemverMismatch(report));
            fixableCount++;
          } else if (_tag === 'PinnedMismatch') {
            ctx.isInvalid = true;
            yield* $(logPinnedMismatch(report));
            fixableCount++;
          } else if (_tag === 'SemverRangeMismatch') {
            ctx.isInvalid = true;
            yield* $(logSemverRangeMismatch(report));
            fixableCount++;
          } else if (_tag === 'SnappedToMismatch') {
            ctx.isInvalid = true;
            yield* $(logSnappedToMismatch(report));
            fixableCount++;
          } else if (_tag === 'MissingLocalVersion') {
            ctx.isInvalid = true;
            yield* $(logMissingLocalVersion(report));
            unfixableCount++;
          } else if (_tag === 'MissingSnappedToMismatch') {
            ctx.isInvalid = true;
            yield* $(logMissingSnappedToMismatch(report));
            unfixableCount++;
          } else if (_tag === 'UnsupportedMismatch') {
            ctx.isInvalid = true;
            yield* $(logUnsupportedMismatch(report));
            unfixableCount++;
          } else if (_tag === 'SameRangeMismatch') {
            ctx.isInvalid = true;
            yield* $(logSameRangeMismatch(report));
            unfixableCount++;
          }
        }
      }

      if (validCount) yield* $(logValidSize(validCount));
      if (fixableCount) yield* $(logFixableSize(fixableCount));
      if (unfixableCount) yield* $(logUnfixableSize(unfixableCount));

      index++;
    }
    return ctx;
  });
}

function logValidSize(amount: number) {
  const msg = chalk`${padStart(amount)} {green ${ICON.tick}} already valid`;
  return Effect.logInfo(msg);
}

function logFixableSize(amount: number) {
  const msg = chalk`${padStart(amount)} {green ${ICON.tick}} can be auto-fixed`;
  return Effect.logInfo(msg);
}

function logUnfixableSize(amount: number) {
  const msg = chalk`{red ${padStart(amount)} ${
    ICON.panic
  } can be fixed manually using} {blue syncpack prompt}`;
  return Effect.logInfo(msg);
}

function logBanned(report: Report.Banned) {
  const _tag = report._tag;
  const instance = report.fixable.instance;
  const name = instance.name;
  const jsonFile = instance.packageJsonFile.jsonFile;
  const path = instance.strategy.path;
  const shortPath = jsonFile.shortPath;

  return Effect.logInfo(
    chalk`{red ${ICON.cross}} ${name} {red banned} {gray ${shortPath} > ${path}} {gray.dim [${_tag}]}`,
  );
}

function logHighestSemverMismatch(report: Report.HighestSemverMismatch) {
  const _tag = report._tag;
  const fixable = report.fixable;
  const instance = fixable.instance;
  const jsonFile = instance.packageJsonFile.jsonFile;
  const actual = instance.rawSpecifier;
  const expected = fixable.raw;
  const name = instance.name;
  const path = instance.strategy.path;
  const shortPath = jsonFile.shortPath;

  return Effect.logInfo(
    chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {green ${expected}} {gray ${shortPath} > ${path}} {gray.dim [${_tag}]}`,
  );
}

function logLocalPackageMismatch(report: Report.LocalPackageMismatch) {
  const _tag = report._tag;
  const fixable = report.fixable;
  const instance = fixable.instance;
  const actual = instance.rawSpecifier;
  const expected = fixable.raw;
  const name = instance.name;
  const path = instance.strategy.path;
  const shortPath = instance.packageJsonFile.jsonFile.shortPath;

  return Effect.logInfo(
    chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {green ${expected}} {gray ${shortPath} > ${path}} {gray.dim [${_tag}]}`,
  );
}

function logLowestSemverMismatch(report: Report.LowestSemverMismatch) {
  const _tag = report._tag;
  const fixable = report.fixable;
  const instance = fixable.instance;
  const actual = instance.rawSpecifier;
  const expected = fixable.raw;
  const name = instance.name;
  const path = instance.strategy.path;
  const shortPath = instance.packageJsonFile.jsonFile.shortPath;

  return Effect.logInfo(
    chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {green ${expected}} {gray ${shortPath} > ${path}} {gray.dim [${_tag}]}`,
  );
}

function logPinnedMismatch(report: Report.PinnedMismatch) {
  const _tag = report._tag;
  const fixable = report.fixable;
  const instance = fixable.instance;
  const actual = instance.rawSpecifier;
  const expected = fixable.raw;
  const name = instance.name;
  const path = instance.strategy.path;
  const shortPath = instance.packageJsonFile.jsonFile.shortPath;

  return Effect.logInfo(
    chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {green ${expected}} {gray ${shortPath} > ${path}} {gray.dim [${_tag}]}`,
  );
}

function logSemverRangeMismatch(report: Report.SemverRangeMismatch) {
  const _tag = report._tag;
  const fixable = report.fixable;
  const instance = fixable.instance;
  const actual = instance.rawSpecifier;
  const expected = fixable.raw;
  const name = instance.name;
  const path = instance.strategy.path;
  const shortPath = instance.packageJsonFile.jsonFile.shortPath;

  return Effect.logInfo(
    chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {green ${expected}} {gray ${shortPath} > ${path}} {gray.dim [${_tag}]}`,
  );
}

function logSnappedToMismatch(report: Report.SnappedToMismatch) {
  const _tag = report._tag;
  const fixable = report.fixable;
  const instance = fixable.instance;
  const actual = instance.rawSpecifier;
  const expected = fixable.raw;
  const name = instance.name;
  const path = instance.strategy.path;
  const shortPath = instance.packageJsonFile.jsonFile.shortPath;

  return Effect.logInfo(
    chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {green ${expected}} {gray ${shortPath} > ${path}} {gray.dim [${_tag}]}`,
  );
}

export function logMissingLocalVersion(report: Report.MissingLocalVersion) {
  const instance = report.unfixable;
  const localPath = report.localInstance.packageJsonFile.jsonFile.shortPath;
  const jsonFile = instance.packageJsonFile.jsonFile;
  const actual = instance.rawSpecifier;
  const name = instance.name;
  const path = instance.strategy.path;
  const shortPath = jsonFile.shortPath;

  return Effect.logInfo(
    [
      chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {red ???} {gray ${shortPath} > ${path}} {gray.dim [missing local version]}`,
      chalk`  {red ${localPath} does not have a .version property which is exact semver}`,
    ].join(EOL),
  );
}

export function logMissingSnappedToMismatch(report: Report.MissingSnappedToMismatch) {
  const instance = report.unfixable;
  const jsonFile = instance.packageJsonFile.jsonFile;
  const actual = instance.rawSpecifier;
  const name = instance.name;
  const path = instance.strategy.path;
  const shortPath = jsonFile.shortPath;

  return Effect.logInfo(
    [
      chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {red ???} {gray ${shortPath} > ${path}} {gray.dim [missing snapTo version]}`,
      chalk`  {red no package in this groups .snapTo array depend on ${name}}`,
    ].join(EOL),
  );
}

export function logUnsupportedMismatch(report: Report.UnsupportedMismatch) {
  const instance = report.unfixable;
  const jsonFile = instance.packageJsonFile.jsonFile;
  const actual = instance.rawSpecifier;
  const name = instance.name;
  const path = instance.strategy.path;
  const shortPath = jsonFile.shortPath;

  return Effect.logInfo(
    [
      chalk`{red ${ICON.cross}} ${name} {red ${actual}} {dim ${ICON.rightArrow}} {red ???} {gray ${shortPath} > ${path}} {gray.dim [unsupported mismatch]}`,
      chalk`  {red use {blue syncpack prompt} to fix this manually}`,
    ].join(EOL),
  );
}

export function logSameRangeMismatch(report: Report.SameRangeMismatch) {
  const instance = report.unfixable;
  const jsonFile = instance.packageJsonFile.jsonFile;
  const actual = instance.rawSpecifier;
  const name = instance.name;
  const path = instance.strategy.path;
  const shortPath = jsonFile.shortPath;
  const mismatches = report.mismatches;

  return Effect.logInfo(
    [
      chalk`{red ${ICON.cross}} ${name} {red range ${actual} does not include ${mismatches.join(
        ', ',
      )}} {gray ${shortPath} > ${path}} {gray.dim [same range mismatch]}`,
      chalk`  {gray use {blue syncpack prompt} to fix this manually}`,
    ].join(EOL),
  );
}
