import { Effect, Option as O, pipe } from 'effect';
import { join } from 'path';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings.js';
import type { Io } from '../../io/index.js';
import { readJsonFileSync } from '../../io/read-json-file-sync.js';

interface LernaJson {
  packages?: string[];
}

export function getLernaPatterns(io: Io): Effect.Effect<never, never, O.Option<string[]>> {
  return pipe(
    readJsonFileSync<LernaJson>(io, join(io.process.cwd(), 'lerna.json')),
    Effect.map((file) =>
      isArrayOfStrings(file.contents.packages) ? O.some(file.contents.packages) : O.none(),
    ),
    Effect.catchTags({
      ReadFileError: () => Effect.succeed(O.none()),
      JsonParseError: () => Effect.succeed(O.none()),
    }),
  );
}
