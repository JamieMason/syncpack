import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import chalk from 'chalk';
import { uniq } from 'tightrope/array/uniq';
import { ICON } from '../constants';
import type { VersionEffectInput as Input, VersionEffects } from '../create-program/effects';
import type { VersionGroupReport } from '../get-version-groups';
import { getUniqueSpecifiers } from '../get-version-groups/lib/get-unique-specifiers';
import type { Instance } from '../instance';
import { logGroupHeader } from '../lib/log-group-header';

export const listEffects: VersionEffects<void> = {
  onFilteredOut() {
    return Effect.unit();
  },
  onIgnored(input) {
    return Effect.sync(() => pipe(input, logHeader, logIgnored));
  },
  onValid(input) {
    return Effect.sync(() => pipe(input, logHeader, logValid));
  },
  onBanned(input) {
    return Effect.sync(() => pipe(input, logHeader, logBanned));
  },
  onHighestSemverMismatch(input) {
    return Effect.sync(() => pipe(input, logHeader, logFixableMismatch));
  },
  onLowestSemverMismatch(input) {
    return Effect.sync(() => pipe(input, logHeader, logFixableMismatch));
  },
  onPinnedMismatch(input) {
    return Effect.sync(() => pipe(input, logHeader, logFixableMismatch));
  },
  onSameRangeMismatch(input) {
    return Effect.sync(() => pipe(input, logHeader, logUnfixableMismatch));
  },
  onSnappedToMismatch(input) {
    return Effect.sync(() => pipe(input, logHeader, logFixableMismatch));
  },
  onNonSemverMismatch(input) {
    return Effect.sync(() => pipe(input, logHeader, logUnfixableMismatch));
  },
  onLocalPackageMismatch(input) {
    return Effect.sync(() => pipe(input, logHeader, logFixableMismatch));
  },
  onComplete() {
    return Effect.unit();
  },
};

function logHeader<T extends VersionGroupReport.Any>(input: Input<T>) {
  if (input.index === 0) {
    logGroupHeader.versionGroup(input.group, input.index);
  }
  return input;
}

function logFixableMismatch<T extends VersionGroupReport.FixableCases>({ report, ctx }: Input<T>) {
  ctx.isInvalid = true;
  console.log(
    chalk`{red %s %s} %s`,
    ICON.cross,
    report.name,
    listColouredSpecifiers(report.expectedVersion, report.instances),
  );
}

function logUnfixableMismatch<T extends VersionGroupReport.UnfixableCases>({
  report,
  ctx,
}: Input<T>) {
  ctx.isInvalid = true;
  console.log(
    chalk`{red %s %s} %s`,
    ICON.cross,
    report.name,
    getUniqueSpecifiers(report.instances)
      .map((instance) =>
        instance.getSemverSpecifier() !== null
          ? chalk.red(instance.specifier)
          : chalk.yellow(instance.specifier),
      )
      .join(chalk.dim(', ')),
  );
}

function logBanned({ report, ctx }: Input<VersionGroupReport.Banned>) {
  ctx.isInvalid = true;
  console.log(
    chalk`{red %s %s} {dim.red is banned in this version group}`,
    ICON.cross,
    report.name,
  );
}

function logIgnored({ report }: Input<VersionGroupReport.Ignored>) {
  console.log(chalk`{dim -} {dim %s} {white is ignored in this version group}`, report.name);
}

function logValid({ report }: Input<VersionGroupReport.Valid>) {
  console.log(chalk`{dim -} {white %s} {dim %s}`, report.name, report.instances?.[0]?.specifier);
}

function listColouredSpecifiers(pinVersion: string, instances: Instance.Any[]) {
  return getAllSpecifiers(pinVersion, instances)
    .map((version) => withColour(pinVersion, version))
    .join(chalk.dim(', '));
}

function withColour(pinVersion: string, version: string) {
  return version === pinVersion ? chalk.green(version) : chalk.red(version);
}

function getAllSpecifiers(pinVersion: string, instances: Instance.Any[]) {
  return uniq([pinVersion].concat(instances.map((i) => i.specifier)));
}
