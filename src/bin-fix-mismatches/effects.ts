import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import chalk from 'chalk';
import { EOL } from 'os';
import { isObject } from 'tightrope/guard/is-object';
import { isUndefined } from 'tightrope/guard/is-undefined';
import { ICON } from '../constants';
import type { VersionEffectInput as Input, VersionEffects } from '../create-program/effects';
import type { Ctx } from '../get-context';
import type { VersionGroupReport } from '../get-version-groups';
import { DELETE } from '../get-version-groups/lib/delete';
import { logGroupHeader } from '../lib/log-group-header';

export const fixMismatchesEffects: VersionEffects = {
  FilteredOut() {
    return Effect.unit();
  },
  Ignored() {
    return Effect.unit();
  },
  Valid() {
    return Effect.unit();
  },
  Banned: (input) => {
    return Effect.sync(() => removeVersions(input));
  },
  HighestSemverMismatch: (input) => {
    return Effect.sync(() => setVersions(input));
  },
  LowestSemverMismatch: (input) => {
    return Effect.sync(() => setVersions(input));
  },
  PinnedMismatch: (input) => {
    return Effect.sync(() => setVersions(input));
  },
  SameRangeMismatch: (input) => {
    return Effect.sync(() => pipe(input, logHeader, logSameRangeMismatch));
  },
  SnappedToMismatch: (input) => {
    return Effect.sync(() => setVersions(input));
  },
  UnsupportedMismatch: (input) => {
    return Effect.sync(() => pipe(input, logHeader, logUnsupportedMismatch));
  },
  WorkspaceMismatch: (input) => {
    return Effect.sync(() => setVersions(input));
  },
  TearDown(ctx) {
    return Effect.sync(() => deleteEmptyObjects(ctx));
  },
};

function logHeader<T extends VersionGroupReport.Any>(input: Input<T>) {
  if (input.index === 0) {
    logGroupHeader.versionGroup(input.group, input.index);
  }
  return input;
}

function setVersions({ report }: Input<VersionGroupReport.FixableCases>) {
  report.instances.forEach((instance) => {
    instance.setVersion(report.expectedVersion);
  });
}

function removeVersions({ report }: Input<VersionGroupReport.Banned>) {
  report.instances.forEach((instance) => {
    instance.setVersion(DELETE);
  });
}

function logSameRangeMismatch({ ctx, report }: Input<VersionGroupReport.SameRangeMismatch>) {
  ctx.isInvalid = true;
  console.log(
    chalk`{yellow %s %s} {dim has mismatched versions under the "sameRange" policy which syncpack cannot auto fix}%s`,
    ICON.panic,
    report.name,
    chalk`${EOL}  use {blue syncpack prompt} to fix manually`,
  );
}

function logUnsupportedMismatch({ ctx, report }: Input<VersionGroupReport.UnsupportedMismatch>) {
  ctx.isInvalid = true;
  console.log(
    chalk`{yellow %s %s} {dim has mismatched unsupported versions which syncpack cannot auto fix}%s`,
    ICON.panic,
    report.name,
    chalk`${EOL}  use {blue syncpack prompt} to fix manually`,
  );
}

/** Remove empty objects such as `{"dependencies": {}}` left after deleting */
function deleteEmptyObjects(ctx: Ctx) {
  ctx.packageJsonFiles.forEach((packageJsonFile) => {
    const contents = packageJsonFile.contents;
    Object.keys(contents).forEach((key) => {
      const value = contents[key];
      if (isObject(value) && Object.values(value).every(isUndefined)) {
        delete contents[key];
      }
    });
  });
}
