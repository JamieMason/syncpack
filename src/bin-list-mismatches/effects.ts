import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import chalk from 'chalk';
import { EOL } from 'os';
import { ICON } from '../constants';
import type { VersionEffectInput as Input, VersionEffects } from '../create-program/effects';
import type { VersionGroupReport } from '../get-version-groups';
import { logGroupHeader } from '../lib/log-group-header';

export const listMismatchesEffects: VersionEffects<void> = {
  onFilteredOut() {
    return Effect.unit;
  },
  onIgnored() {
    return Effect.unit;
  },
  onValid() {
    return Effect.unit;
  },
  onBanned(input) {
    return Effect.sync(() => pipe(input, logHeader, logBanned));
  },
  onHighestSemverMismatch(input) {
    return Effect.sync(() => pipe(input, logHeader, logHighLowSemverMismatch));
  },
  onLowestSemverMismatch(input) {
    return Effect.sync(() => pipe(input, logHeader, logHighLowSemverMismatch));
  },
  onPinnedMismatch(input) {
    return Effect.sync(() => pipe(input, logHeader, logPinnedMismatch));
  },
  onSameRangeMismatch(input) {
    return Effect.sync(() => pipe(input, logHeader, logSameRangeMismatch));
  },
  onSnappedToMismatch(input) {
    return Effect.sync(() => pipe(input, logHeader, logSnappedToMismatch));
  },
  onNonSemverMismatch(input) {
    return Effect.sync(() => pipe(input, logHeader, logNonSemverMismatch));
  },
  onLocalPackageMismatch(input) {
    return Effect.sync(() => pipe(input, logHeader, logLocalPackageMismatch));
  },
  onComplete() {
    return Effect.unit;
  },
};

function logHeader<T extends VersionGroupReport.Any>(input: Input<T>) {
  if (input.index === 0) {
    logGroupHeader.versionGroup(input.group, input.index);
  }
  return input;
}

function logBanned({ report, ctx }: Input<VersionGroupReport.Banned>) {
  ctx.isInvalid = true;
  console.log(chalk`  {red %s} %s {dim is banned in this version group}`, ICON.cross, report.name);
  report.instances.forEach((instance) => {
    console.log(
      chalk`  {red %s} {dim in %s of %s}`,
      instance.specifier,
      instance.strategy.path,
      instance.packageJsonFile.shortPath,
    );
  });
}

function logHighLowSemverMismatch({
  report,
  ctx,
}: Input<VersionGroupReport.HighLowSemverMismatch>) {
  ctx.isInvalid = true;
  console.log(
    chalk`{red %s} %s {green %s} {dim is the %s valid semver version in use}`,
    ICON.cross,
    report.name,
    report.expectedVersion,
    report._tag === 'LowestSemverMismatch' ? 'lowest' : 'highest',
  );
  report.instances.forEach((instance) => {
    if (instance.specifier !== report.expectedVersion) {
      console.log(
        chalk`  {red %s} {dim in %s of %s}`,
        instance.specifier,
        instance.strategy.path,
        instance.packageJsonFile.shortPath,
      );
    }
  });
}

function logPinnedMismatch({ report, ctx }: Input<VersionGroupReport.PinnedMismatch>) {
  ctx.isInvalid = true;
  console.log(
    chalk`{red %s} %s {dim is pinned in this version group at} {green %s}`,
    ICON.cross,
    report.name,
    report.expectedVersion,
  );
  report.instances.forEach((instance) => {
    if (instance.specifier !== report.expectedVersion) {
      console.log(
        chalk`  {red %s} {dim in %s of %s}`,
        instance.specifier,
        instance.strategy.path,
        instance.packageJsonFile.shortPath,
      );
    }
  });
}

function logSnappedToMismatch({ report, ctx }: Input<VersionGroupReport.SnappedToMismatch>) {
  ctx.isInvalid = true;
  console.log(
    chalk`{red %s} %s {dim should snap to {reset.green %s}, used by %s}`,
    ICON.cross,
    report.name,
    report.expectedVersion,
    report.snapTo.join(' || '),
  );
  report.instances.forEach((instance) => {
    if (instance.specifier !== report.expectedVersion) {
      console.log(
        chalk`  {red %s} {dim in %s of %s}`,
        instance.specifier,
        instance.strategy.path,
        instance.packageJsonFile.shortPath,
      );
    }
  });
}

function logSameRangeMismatch({ report, ctx }: Input<VersionGroupReport.SameRangeMismatch>) {
  ctx.isInvalid = true;
  console.log(
    chalk`{yellow %s %s} {dim has mismatched versions under the "sameRange" policy which syncpack cannot auto fix}%s`,
    ICON.panic,
    report.name,
    chalk`${EOL}  use {blue syncpack prompt} to fix manually`,
  );
  report.instances.forEach((instance) => {
    console.log(
      chalk`  {yellow %s} {dim in %s of %s}`,
      instance.specifier,
      instance.strategy.path,
      instance.packageJsonFile.shortPath,
    );
  });
}

function logNonSemverMismatch({ report, ctx }: Input<VersionGroupReport.NonSemverMismatch>) {
  ctx.isInvalid = true;
  console.log(
    chalk`{yellow %s %s} {dim has mismatched unsupported versions which syncpack cannot auto fix}%s`,
    ICON.panic,
    report.name,
    chalk`${EOL}  use {blue syncpack prompt} to fix manually`,
  );
  report.instances.forEach((instance) => {
    console.log(
      chalk`  {yellow %s} {dim in %s of %s}`,
      instance.specifier,
      instance.strategy.path,
      instance.packageJsonFile.shortPath,
    );
  });
}

function logLocalPackageMismatch({ report, ctx }: Input<VersionGroupReport.LocalPackageMismatch>) {
  ctx.isInvalid = true;
  console.log(
    chalk`{red %s} %s {green %s} {dim is developed in this repo at %s}`,
    ICON.cross,
    report.name,
    report.expectedVersion,
    report.localPackageInstance.packageJsonFile.shortPath,
  );
  report.instances.forEach((instance) => {
    if (instance.specifier !== report.expectedVersion) {
      console.log(
        chalk`  {red %s} {dim in %s of %s}`,
        instance.specifier,
        instance.strategy.path,
        instance.packageJsonFile.shortPath,
      );
    }
  });
}
