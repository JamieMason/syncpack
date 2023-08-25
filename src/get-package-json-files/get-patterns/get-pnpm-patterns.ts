import { Effect, Option as O, pipe } from 'effect';
import { join } from 'path';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { type Io } from '../../io';
import { readYamlFileSync } from '../../io/read-yaml-file-sync';

interface PnpmWorkspace {
  packages?: string[];
}

export function getPnpmPatterns(io: Io): Effect.Effect<never, never, O.Option<string[]>> {
  return pipe(
    // packages:
    //   - "packages/**"
    //   - "components/**"
    //   - "!**/test/**"
    readYamlFileSync<PnpmWorkspace>(io, join(io.process.cwd(), 'pnpm-workspace.yaml')),
    Effect.map((file) => (isArrayOfStrings(file?.packages) ? O.some(file.packages) : O.none())),
    Effect.catchTags({
      ReadYamlFileError: () => Effect.succeed(O.none()),
    }),
  );
}
