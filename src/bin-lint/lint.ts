import chalk from 'chalk';
import { Context, Effect, pipe } from 'effect';
import { pipeline as lintSemverRanges } from '../bin-lint-semver-ranges/lint-semver-ranges';
import { pipeline as listMismatches } from '../bin-list-mismatches/list-mismatches';
import { CliConfigTag } from '../config/tag';
import { type CliConfig } from '../config/types';
import type { ErrorHandlers } from '../error-handlers/default-error-handlers';
import { defaultErrorHandlers } from '../error-handlers/default-error-handlers';
import { getContext } from '../get-context';
import type { Io } from '../io';
import { IoTag } from '../io';
import { exitIfInvalid } from '../io/exit-if-invalid';
import { withLogger } from '../lib/with-logger';

interface Input {
  io: Io;
  cli: Partial<CliConfig>;
  errorHandlers?: ErrorHandlers;
}

export function lint({ io, cli, errorHandlers = defaultErrorHandlers }: Input) {
  return pipe(
    getContext({ io, cli, errorHandlers }),
    Effect.tap(() => Effect.logInfo(chalk`{yellow Versions}`)),
    Effect.flatMap((ctx) => listMismatches(ctx, io, errorHandlers)),
    Effect.tap(() => Effect.logInfo(chalk`{yellow Semver Ranges}`)),
    Effect.flatMap((ctx) => lintSemverRanges(ctx, io, errorHandlers)),
    Effect.flatMap(exitIfInvalid),
    Effect.provide(pipe(Context.empty(), Context.add(CliConfigTag, cli), Context.add(IoTag, io))),
    withLogger,
  );
}
