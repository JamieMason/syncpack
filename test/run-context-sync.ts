import * as Context from '@effect/data/Context';
import { match } from '@effect/data/Either';
import { identity, pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import type { CliConfig } from '../src/config/types';
import { CliConfigTag } from '../src/config/types';
import { createEnv } from '../src/env/create-env';
import { EnvTag } from '../src/env/tags';
import { getContext } from '../src/get-context';
import type { MockEnv } from './mock-env';

export function runContextSync(cli: Partial<CliConfig>, mockedEffects: MockEnv) {
  return pipe(
    Effect.runSyncEither(
      pipe(
        getContext(),
        Effect.provideContext(
          pipe(
            Context.empty(),
            Context.add(CliConfigTag, cli),
            Context.add(EnvTag, createEnv(mockedEffects)),
          ),
        ),
      ),
    ),
    match(identity, identity),
  );
}
