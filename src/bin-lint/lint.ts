import chalk from 'chalk';
import { Context, Effect, pipe } from 'effect';
import { pipeline as format } from '../bin-format/format';
import { pipeline as lintSemverRanges } from '../bin-lint-semver-ranges/lint-semver-ranges';
import { pipeline as listMismatches } from '../bin-list-mismatches/list-mismatches';
import { CliConfigTag } from '../config/tag';
import { type CliConfig } from '../config/types';
import { ICON } from '../constants';
import type { ErrorHandlers } from '../error-handlers/default-error-handlers';
import { defaultErrorHandlers } from '../error-handlers/default-error-handlers';
import { getContext } from '../get-context';
import type { Io } from '../io';
import { IoTag } from '../io';
import { exitIfInvalid } from '../io/exit-if-invalid';
import { toJson } from '../io/to-json';
import { withLogger } from '../lib/with-logger';

interface Input {
  io: Io;
  cli: Partial<CliConfig>;
  errorHandlers?: ErrorHandlers;
}

export function lint({ io, cli, errorHandlers = defaultErrorHandlers }: Input) {
  return pipe(
    getContext({ io, cli, errorHandlers }),
    // Versions
    Effect.flatMap((ctx) =>
      Effect.gen(function* ($) {
        if (ctx.config.rcFile.lintVersions !== false) {
          yield* $(Effect.logInfo(chalk`{yellow Versions}`));
          yield* $(listMismatches(ctx, io, errorHandlers));
        }
        return ctx;
      }),
    ),
    // Semver Ranges
    Effect.flatMap((ctx) =>
      Effect.gen(function* ($) {
        if (ctx.config.rcFile.lintSemverRanges !== false) {
          yield* $(Effect.logInfo(chalk`{yellow Semver Ranges}`));
          yield* $(lintSemverRanges(ctx, io, errorHandlers));
        }
        return ctx;
      }),
    ),
    // Formatting
    Effect.flatMap((ctx) =>
      Effect.gen(function* ($) {
        if (ctx.config.rcFile.lintFormatting !== false) {
          yield* $(Effect.logInfo(chalk`{yellow Formatting}`));
          yield* $(format(ctx));
          for (const file of ctx.packageJsonFiles) {
            const nextJson = toJson(ctx, file);
            const hasChanged = file.jsonFile.json !== nextJson;
            const shortPath = file.jsonFile.shortPath;
            if (hasChanged) {
              ctx.isInvalid = true;
              yield* $(Effect.logInfo(chalk`{red ${ICON.cross}} ${shortPath}`));
            } else {
              yield* $(Effect.logInfo(chalk`{green ${ICON.tick}} ${shortPath}`));
            }
          }
        }
        return ctx;
      }),
    ),
    Effect.flatMap(exitIfInvalid),
    Effect.provide(pipe(Context.empty(), Context.add(CliConfigTag, cli), Context.add(IoTag, io))),
    withLogger,
  );
}
