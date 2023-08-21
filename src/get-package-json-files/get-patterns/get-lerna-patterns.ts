import { pipe } from '@effect/data/Function';
import * as O from '@effect/data/Option';
import * as Effect from '@effect/io/Effect';
import { join } from 'path';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import type { Env } from '../../env/create-env';
import { EnvTag } from '../../env/tags';

interface LernaJson {
  packages?: string[];
}

export function getLernaPatterns(): Effect.Effect<Env, never, O.Option<string[]>> {
  return pipe(
    EnvTag,
    Effect.flatMap((env) => env.readJsonFileSync<LernaJson>(join(env.CWD, 'lerna.json'))),
    Effect.map((file) =>
      isArrayOfStrings(file?.contents?.packages) ? O.some(file.contents.packages) : O.none(),
    ),
    Effect.catchTags({
      ReadFileError: () => Effect.succeed(O.none()),
      JsonParseError: () => Effect.succeed(O.none()),
    }),
  );
}
