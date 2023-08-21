import { pipe } from '@effect/data/Function';
import * as O from '@effect/data/Option';
import * as Effect from '@effect/io/Effect';
import { join } from 'path';
import { isArrayOfStrings } from 'tightrope/guard/is-array-of-strings';
import { isNonEmptyObject } from 'tightrope/guard/is-non-empty-object';
import type { Env } from '../../env/create-env';
import { EnvTag } from '../../env/tags';
import type { PackageJson } from '../package-json-file';

export function getYarnPatterns(): Effect.Effect<Env, never, O.Option<string[]>> {
  return pipe(
    EnvTag,
    Effect.flatMap((env) => env.readJsonFileSync<PackageJson>(join(env.CWD, 'package.json'))),
    Effect.map((file) =>
      isNonEmptyObject(file?.contents?.workspaces) &&
      isArrayOfStrings(file.contents.workspaces?.packages)
        ? O.some(file.contents.workspaces.packages)
        : isArrayOfStrings(file?.contents?.workspaces)
        ? O.some(file.contents.workspaces)
        : O.none(),
    ),
    Effect.catchTags({
      ReadFileError: () => Effect.succeed(O.none()),
      JsonParseError: () => Effect.succeed(O.none()),
    }),
  );
}
