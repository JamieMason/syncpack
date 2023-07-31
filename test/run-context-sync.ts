import * as Context from '@effect/data/Context';
import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import type { CliConfig } from '../src/config/types';
import { CliConfigTag } from '../src/config/tag';
import { createEnv } from '../src/env/create-env';
import { EnvTag } from '../src/env/tags';
import { getContext } from '../src/get-context';
import type { MockEnv } from './mock-env';

export function runContextSync(
  cli: Partial<CliConfig>,
  mockedEffects: MockEnv,
  onValue: (value: any) => void,
) {
  Effect.runSync(
    pipe(
      getContext(),
      Effect.match({
        onFailure: onValue,
        onSuccess: onValue,
      }),
      Effect.provideContext(
        pipe(
          Context.empty(),
          Context.add(CliConfigTag, cli),
          Context.add(EnvTag, createEnv(mockedEffects)),
        ),
      ),
    ),
  );
}
