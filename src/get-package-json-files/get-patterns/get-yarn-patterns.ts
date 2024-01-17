import { Effect, Option as O, pipe } from 'effect';
import { join } from 'path';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings.js';
import { isNonEmptyObject } from 'tightrope/guard/is-non-empty-object.js';
import type { Io } from '../../io/index.js';
import { readJsonFileSync } from '../../io/read-json-file-sync.js';
import type { PackageJson } from '../package-json-file.js';

export function getYarnPatterns(io: Io): Effect.Effect<never, never, O.Option<string[]>> {
  return pipe(
    readJsonFileSync<PackageJson>(io, join(io.process.cwd(), 'package.json')),
    Effect.map((file) =>
      isNonEmptyObject(file.contents.workspaces) &&
      isArrayOfStrings(file.contents.workspaces.packages)
        ? O.some(file.contents.workspaces.packages)
        : isArrayOfStrings(file.contents.workspaces)
          ? O.some(file.contents.workspaces)
          : O.none(),
    ),
    Effect.catchTags({
      ReadFileError: () => Effect.succeed(O.none()),
      JsonParseError: () => Effect.succeed(O.none()),
    }),
  );
}
