import chalk from 'chalk';
import { Context, Effect, flow, pipe } from 'effect';
import { gtr } from 'semver';
import { CliConfigTag } from '../config/tag';
import { type CliConfig } from '../config/types';
import { ICON } from '../constants';
import type { ErrorHandlers } from '../error-handlers/default-error-handlers';
import { defaultErrorHandlers } from '../error-handlers/default-error-handlers';
import { getContext } from '../get-context';
import { getInstances } from '../get-instances';
import type { Instance } from '../get-instances/instance';
import type { Io } from '../io';
import { IoTag } from '../io';
import { exitIfInvalid } from '../io/exit-if-invalid';
import { writeIfChanged } from '../io/write-if-changed';
import { withLogger } from '../lib/with-logger';
import { Specifier } from '../specifier';
import { updateEffects } from './effects';

export function update(
  io: Io,
  cli: Partial<CliConfig>,
  effects: typeof updateEffects = updateEffects,
  errorHandlers: ErrorHandlers = defaultErrorHandlers,
) {
  return pipe(
    Effect.Do,
    Effect.bind('ctx', () => getContext({ io, cli, errorHandlers })),
    Effect.bind('instances', ({ ctx }) => getInstances(ctx, io, errorHandlers)),
    Effect.bind('update', ({ instances }) =>
      pipe(
        Effect.succeed(instances.all),
        Effect.map((instances) => {
          const isVisitedByName: Record<string, boolean> = {};
          const updateable: Instance[] = [];
          instances.forEach((instance) => {
            if (
              !isVisitedByName[instance.name] &&
              (instance.versionGroup._tag === 'SameRange' ||
                instance.versionGroup._tag === 'Standard')
            ) {
              const specifier = Specifier.create(instance, instance.rawSpecifier);
              if (specifier._tag === 'RangeSpecifier' || specifier._tag === 'VersionSpecifier') {
                isVisitedByName[instance.name] = true;
                updateable.push(instance);
              }
            }
          });
          return updateable;
        }),
        Effect.tap(updateEffects.onFetchAllStart),
        Effect.flatMap((instances) =>
          pipe(
            instances,
            Effect.partition(
              (instance) =>
                pipe(
                  Effect.succeed(instance),
                  Effect.tap(() => updateEffects.onFetchStart(instance, instances.length)),
                  Effect.flatMap(effects.fetchLatestVersions),
                  Effect.tapBoth({
                    onFailure: () => updateEffects.onFetchEnd(instance),
                    onSuccess: ({ versions }) => updateEffects.onFetchEnd(instance, versions),
                  }),
                  // move up to date dependencies to error channel
                  Effect.flatMap((updateable) =>
                    gtr(updateable.versions.latest, instance.rawSpecifier)
                      ? pipe(
                          updateEffects.onOutdated(instance, updateable.versions.latest),
                          Effect.map(() => updateable),
                        )
                      : pipe(
                          updateEffects.onUpToDate(instance),
                          Effect.flatMap(() => Effect.fail(updateable)),
                        ),
                  ),
                  // log error but don't catch it
                  Effect.tapErrorTag('HttpError', ({ error }) =>
                    Effect.logError(chalk`{red ${ICON.cross} ${error}}`),
                  ),
                  // log error but don't catch it
                  Effect.tapErrorTag('NpmRegistryError', ({ error }) =>
                    Effect.logError(chalk`{red ${ICON.cross} ${error}}`),
                  ),
                ),
              { concurrency: 10 },
            ),
            // discard errors and up to date dependencies
            Effect.flatMap(([_, outOfDate]) => Effect.succeed(outOfDate)),
          ),
        ),
        // always remove the spinner when we're done
        Effect.tapBoth({
          onFailure: updateEffects.onFetchAllEnd,
          onSuccess: updateEffects.onFetchAllEnd,
        }),
        // ask the user which updates they want
        Effect.flatMap(updateEffects.promptForUpdates),
        // if we think the user cancelled, say so
        Effect.catchTag('PromptCancelled', () =>
          Effect.logInfo(
            chalk`{red ${ICON.panic}} aborting after {blue syncpack update} was cancelled`,
          ),
        ),
      ),
    ),
    Effect.flatMap(({ ctx }) =>
      pipe(
        writeIfChanged(ctx),
        Effect.catchTags({
          WriteFileError: flow(
            errorHandlers.WriteFileError,
            Effect.map(() => {
              ctx.isInvalid = true;
              return ctx;
            }),
          ),
        }),
      ),
    ),
    Effect.flatMap(exitIfInvalid),
    Effect.withConcurrency(10),
    Effect.provide(pipe(Context.empty(), Context.add(CliConfigTag, cli), Context.add(IoTag, io))),
    withLogger,
  );
}
