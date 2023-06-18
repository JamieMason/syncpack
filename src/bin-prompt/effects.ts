import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import chalk from 'chalk';
import { ICON } from '../constants';
import type { VersionEffectInput as Input, VersionEffects } from '../create-program/effects';
import { EnvTag } from '../env/tags';
import type { VersionGroupReport } from '../get-version-groups';
import { getUniqueVersions } from '../get-version-groups/lib/get-unique-versions';
import { logGroupHeader } from '../lib/log-group-header';

export const promptEffects: VersionEffects = {
  FilteredOut() {
    return Effect.unit();
  },
  Ignored() {
    return Effect.unit();
  },
  Valid() {
    return Effect.unit();
  },
  Banned() {
    return Effect.unit();
  },
  HighestSemverMismatch() {
    return Effect.unit();
  },
  LowestSemverMismatch() {
    return Effect.unit();
  },
  PinnedMismatch() {
    return Effect.unit();
  },
  SameRangeMismatch(input) {
    return pipe(
      Effect.sync(() => logHeader(input)),
      Effect.flatMap(askForNextVersion),
    );
  },
  SnappedToMismatch() {
    return Effect.unit();
  },
  UnsupportedMismatch(input) {
    return pipe(
      Effect.sync(() => logHeader(input)),
      Effect.flatMap(askForNextVersion),
    );
  },
  WorkspaceMismatch() {
    return Effect.unit();
  },
  TearDown() {
    return Effect.unit();
  },
};

function logHeader<T extends VersionGroupReport.Any>(input: Input<T>) {
  if (input.index === 0) {
    logGroupHeader.versionGroup(input.group, input.index);
  }
  return input;
}

function askForNextVersion({ report }: Input<VersionGroupReport.UnfixableCases>) {
  return pipe(
    Effect.gen(function* ($) {
      const OTHER = chalk.dim('Other');
      const SKIP = chalk.dim('Skip this dependency');
      const env = yield* $(EnvTag);
      const choice = yield* $(
        env.askForChoice({
          message: chalk`${report.name} {dim Choose a version to replace the others}`,
          choices: [...getUniqueVersions(report.instances), OTHER, SKIP],
        }),
      );
      if (choice === SKIP) return;
      const nextVersion =
        choice === OTHER
          ? yield* $(
              env.askForInput({
                message: chalk`${report.name} {dim Enter a new version to replace the others}`,
              }),
            )
          : choice;
      yield* $(
        Effect.sync(() => {
          report.instances.forEach((instance) => {
            instance.setVersion(nextVersion);
          });
        }),
      );
    }),
    Effect.catchTags({
      AskForChoiceError: (err) =>
        Effect.sync(() => {
          console.error(chalk.red(ICON.panic, 'AskForChoiceError:', err));
        }),
      AskForInputError: (err) =>
        Effect.sync(() => {
          console.error(chalk.red(ICON.panic, 'AskForInputError:', err));
        }),
    }),
  );
}
