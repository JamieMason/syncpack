import * as Context from '@effect/data/Context';
import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import { type CliConfig } from '../config/types';
import { CliConfigTag } from '../config/tag';
import { createEnv } from '../env/create-env';
import type { DefaultEnv } from '../env/default-env';
import { exitIfInvalid } from '../env/exit-if-invalid';
import { EnvTag } from '../env/tags';
import { writeIfChanged } from '../env/write-if-changed';
import { createErrorHandlers } from '../error-handlers/create-error-handlers';
import { defaultErrorHandlers } from '../error-handlers/default-error-handlers';
import { getContext } from '../get-context';
import { format } from './format';

export function formatCli(cli: Partial<CliConfig>, env: DefaultEnv) {
  return pipe(
    getContext(),
    Effect.flatMap(format),
    Effect.flatMap(writeIfChanged),
    Effect.flatMap(exitIfInvalid),
    Effect.catchTags(createErrorHandlers(defaultErrorHandlers)),
    Effect.provideContext(
      pipe(Context.empty(), Context.add(CliConfigTag, cli), Context.add(EnvTag, createEnv(env))),
    ),
  );
}
