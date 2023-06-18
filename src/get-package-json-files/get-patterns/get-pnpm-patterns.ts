import { pipe } from '@effect/data/Function';
import * as O from '@effect/data/Option';
import * as Effect from '@effect/io/Effect';
import { join } from 'path';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { CWD } from '../../constants';
import { type Env } from '../../env/create-env';
import { EnvTag } from '../../env/tags';

interface PnpmWorkspace {
  packages?: string[];
}

export function getPnpmPatterns(): Effect.Effect<Env, never, O.Option<string[]>> {
  return pipe(
    // packages:
    //   - "packages/**"
    //   - "components/**"
    //   - "!**/test/**"
    EnvTag,
    Effect.flatMap((env) => env.readYamlFileSync<PnpmWorkspace>(join(CWD, 'pnpm-workspace.yaml'))),
    Effect.map((file) => (isArrayOfStrings(file?.packages) ? O.some(file.packages) : O.none())),
    Effect.catchTags({
      ReadYamlFileError: () => Effect.succeed(O.none()),
    }),
  );
}
